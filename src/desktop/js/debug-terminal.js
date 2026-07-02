(function () {
  const debugState = {
    adbSupported: false,
    activeKind: null,
    sshPrefs: null,
    sshAdapters: [],
    listenersReady: false,
  };

  function getTerminal(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugTerminal' : 'sshDebugTerminal');
  }

  function getStatus(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugStatus' : 'sshDebugStatus');
  }

  function appendDebugOutput(kind, text) {
    const terminal = getTerminal(kind);
    if (!terminal || !text) return;
    terminal.textContent += text;
    terminal.scrollTop = terminal.scrollHeight;
  }

  function setDebugStatus(kind, text) {
    const el = getStatus(kind);
    if (el) el.textContent = text;
  }

  function currentPage() {
    const activePage = document.querySelector('.page.active');
    return activePage ? activePage.id.replace('page-', '') : '';
  }

  async function loadDebugPrefs() {
    try {
      const prefs = await invoke('get_debug_terminal_prefs');
      debugState.sshPrefs = prefs || {};
      const userInput = document.getElementById('sshUsername');
      const hostInput = document.getElementById('sshHost');
      if (userInput && prefs?.ssh_username) userInput.value = prefs.ssh_username;
      if (hostInput && prefs?.ssh_last_ip) hostInput.value = prefs.ssh_last_ip;
    } catch (e) {
      console.warn('[DebugTerminal] 读取偏好失败:', e);
    }
  }

  async function saveDebugPrefs() {
    const prefs = {
      ssh_username: document.getElementById('sshUsername')?.value?.trim() || null,
      ssh_last_adapter: document.getElementById('sshAdapterSelect')?.selectedOptions?.[0]?.dataset?.adapterName || null,
      ssh_last_ip: document.getElementById('sshHost')?.value?.trim() || null,
    };
    try {
      debugState.sshPrefs = await invoke('save_debug_terminal_prefs', { prefs });
    } catch (e) {
      console.warn('[DebugTerminal] 保存偏好失败:', e);
    }
  }

  async function refreshSshDebugAdapters() {
    const select = document.getElementById('sshAdapterSelect');
    if (!select) return;
    try {
      const adapters = await invoke('list_debug_network_adapters');
      debugState.sshAdapters = adapters || [];
      select.innerHTML = '';
      if (!adapters || adapters.length === 0) {
        select.innerHTML = '<option value="">未找到可用有线网卡</option>';
        return;
      }
      adapters.forEach((adapter, idx) => {
        const option = document.createElement('option');
        option.value = adapter.gateway || '';
        option.dataset.adapterName = adapter.name;
        option.textContent = `${adapter.name} (${adapter.ip_address}) -> 网关: ${adapter.gateway}`;
        select.appendChild(option);
        if (debugState.sshPrefs?.ssh_last_adapter && debugState.sshPrefs.ssh_last_adapter === adapter.name) {
          select.selectedIndex = idx;
        }
      });
      if (select.selectedIndex < 0) select.selectedIndex = 0;
      handleSshAdapterChange();
    } catch (e) {
      select.innerHTML = `<option value="">刷新网卡失败: ${escapeHtml(String(e))}</option>`;
    }
  }

  function handleSshAdapterChange() {
    const select = document.getElementById('sshAdapterSelect');
    const hostInput = document.getElementById('sshHost');
    if (!select || !hostInput) return;
    const selected = select.selectedOptions && select.selectedOptions[0];
    if (!selected) return;
    hostInput.value = selected.value || hostInput.value || '';
    saveDebugPrefs().catch(() => {});
  }

  async function ensureAdbEnabled() {
    if (!state.connected) {
      setDebugStatus('adb', '请先连接模组，再使用 ADB 调试。');
      return false;
    }
    try {
      const toggles = await invoke('get_feature_toggles');
      if (!toggles.adb) {
        setDebugStatus('adb', 'ADB 未开启，请先到“系统信息”页开启 ADB，并重启设备后重新连接。');
        return false;
      }
      return true;
    } catch (e) {
      setDebugStatus('adb', '读取 ADB 开关状态失败: ' + e.message);
      return false;
    }
  }

  async function connectAdbDebug() {
    if (!(await ensureAdbEnabled())) return;
    try {
      clearDebugTerminal('adb');
      await invoke('start_adb_session');
      debugState.activeKind = 'adb';
      state.debugTerminal.activeKind = 'adb';
      setDebugStatus('adb', 'ADB shell 连接中...');
    } catch (e) {
      setDebugStatus('adb', e.message || String(e));
      showToast('ADB 调试连接失败: ' + (e.message || String(e)), 'err');
    }
  }

  async function connectSshDebug() {
    const host = document.getElementById('sshHost')?.value?.trim() || '';
    const username = document.getElementById('sshUsername')?.value?.trim() || '';
    const password = document.getElementById('sshPassword')?.value || '';
    if (!host || !username || !password) {
      showToast('请完整填写 SSH 连接信息', 'err');
      return;
    }
    try {
      await saveDebugPrefs();
      clearDebugTerminal('ssh');
      await invoke('start_ssh_session', { host, username, password });
      debugState.activeKind = 'ssh';
      state.debugTerminal.activeKind = 'ssh';
      setDebugStatus('ssh', `SSH 连接中: ${host}`);
    } catch (e) {
      setDebugStatus('ssh', e.message || String(e));
      showToast('SSH 调试连接失败: ' + (e.message || String(e)), 'err');
    }
  }

  async function disconnectDebugTerminal() {
    try {
      await invoke('close_debug_terminal_session');
    } catch (e) {
      console.warn('[DebugTerminal] 断开失败:', e);
    }
    debugState.activeKind = null;
    state.debugTerminal.activeKind = null;
    const pwd = document.getElementById('sshPassword');
    if (pwd) pwd.value = '';
    setDebugStatus('adb', 'ADB 调试已断开。');
    setDebugStatus('ssh', 'SSH 调试已断开。');
  }

  async function writeCurrentDebugInput(kind, inputId) {
    const input = document.getElementById(inputId);
    if (!input) return;
    const value = input.value;
    if (!value) return;
    try {
      await invoke('write_debug_terminal_input', { input: value + '\n' });
      input.value = '';
    } catch (e) {
      showToast('发送调试命令失败: ' + (e.message || String(e)), 'err');
    }
  }

  function clearDebugTerminal(kind) {
    const terminal = getTerminal(kind);
    if (terminal) terminal.textContent = '';
  }

  function handleDebugOutputEvent(event) {
    const payload = event.payload || {};
    if (!payload.kind || !payload.text) return;
    appendDebugOutput(payload.kind, payload.text);
    if (payload.stream === 'system') {
      setDebugStatus(payload.kind, payload.text.trim() || payload.text);
    }
  }

  async function initDebugTerminal() {
    try {
      const caps = await invoke('get_debug_terminal_capabilities');
      debugState.adbSupported = !!caps.adb_supported;
      state.debugTerminal.capabilities = caps;
      const adbNav = document.getElementById('adbDebugNav');
      if (adbNav) adbNav.style.display = caps.adb_supported ? '' : 'none';
      await loadDebugPrefs();
      await refreshSshDebugAdapters();
      if (!debugState.listenersReady) {
        const listen = window.__TAURI__?.event?.listen;
        if (listen) {
          listen('debug-terminal-output', handleDebugOutputEvent);
          debugState.listenersReady = true;
        }
      }
    } catch (e) {
      console.warn('[DebugTerminal] 初始化失败:', e);
    }
  }

  function handlePageChange(prevPage, nextPage) {
    const leavingDebugPage = (prevPage === 'adbdebug' || prevPage === 'sshdebug') &&
      nextPage !== prevPage &&
      nextPage !== 'adbdebug' &&
      nextPage !== 'sshdebug';
    if (leavingDebugPage && debugState.activeKind) {
      disconnectDebugTerminal().catch(() => {});
    }
    if (nextPage === 'adbdebug') {
      ensureAdbEnabled().catch(() => {});
    }
    if (nextPage === 'sshdebug') {
      refreshSshDebugAdapters().catch(() => {});
    }
  }

  function handleAdbDebugKey(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      sendAdbDebugInput();
    }
  }

  function handleSshDebugKey(event) {
    if (event.key === 'Enter') {
      event.preventDefault();
      sendSshDebugInput();
    }
  }

  window.initDebugTerminal = initDebugTerminal;
  window.connectAdbDebug = connectAdbDebug;
  window.connectSshDebug = connectSshDebug;
  window.disconnectDebugTerminal = disconnectDebugTerminal;
  window.sendAdbDebugInput = () => writeCurrentDebugInput('adb', 'adbDebugInput');
  window.sendSshDebugInput = () => writeCurrentDebugInput('ssh', 'sshDebugInput');
  window.clearDebugTerminal = clearDebugTerminal;
  window.refreshSshDebugAdapters = refreshSshDebugAdapters;
  window.handleSshAdapterChange = handleSshAdapterChange;
  window.handleAdbDebugKey = handleAdbDebugKey;
  window.handleSshDebugKey = handleSshDebugKey;
  window.handleDebugTerminalPageChange = handlePageChange;
})();
