(function () {
  const debugState = {
    adbSupported: false,
    activeKind: null,
    pendingKind: null,
    sshPrefs: null,
    sshAdapters: [],
    listenersReady: false,
    writeChain: Promise.resolve(),
  };

  function getTerminal(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugTerminal' : 'sshDebugTerminal');
  }

  function getStatus(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugStatus' : 'sshDebugStatus');
  }

  function getConnectBtn(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugConnectBtn' : 'sshDebugConnectBtn');
  }

  function getDisconnectBtn(kind) {
    return document.getElementById(kind === 'adb' ? 'adbDebugDisconnectBtn' : 'sshDebugDisconnectBtn');
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

  function focusDebugTerminal(kind) {
    const terminal = getTerminal(kind);
    if (terminal) terminal.focus();
  }

  function updateTerminalState() {
    ['adb', 'ssh'].forEach(kind => {
      const terminal = getTerminal(kind);
      if (!terminal) return;
      terminal.classList.toggle('is-active', debugState.activeKind === kind || debugState.pendingKind === kind);
    });
  }

  function resetDebugSession(kind, disconnectedText) {
    if (debugState.activeKind === kind) debugState.activeKind = null;
    if (debugState.pendingKind === kind) debugState.pendingKind = null;
    state.debugTerminal.activeKind = debugState.activeKind;
    if (kind === 'ssh') {
      const pwd = document.getElementById('sshPassword');
      if (pwd) pwd.value = '';
    }
    if (disconnectedText) setDebugStatus(kind, disconnectedText);
    updateDebugControls();
    updateTerminalState();
  }

  function updateDebugControls() {
    const busyKind = debugState.pendingKind || debugState.activeKind;
    ['adb', 'ssh'].forEach(kind => {
      const connectBtn = getConnectBtn(kind);
      const disconnectBtn = getDisconnectBtn(kind);
      if (!connectBtn || !disconnectBtn) return;

      const isPending = debugState.pendingKind === kind;
      const isActive = debugState.activeKind === kind;
      const blockedByOther = !!busyKind && busyKind !== kind;

      connectBtn.disabled = isPending || isActive || blockedByOther;
      connectBtn.textContent = isPending
        ? t('btn_connecting')
        : isActive
          ? t('btn_connected')
          : t('btn_connect');

      disconnectBtn.disabled = !(isPending || isActive);
    });
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

  function ensureAdbSupported() {
    if (!debugState.adbSupported) {
      setDebugStatus('adb', '当前环境不支持 ADB 调试。');
      return false;
    }
    return true;
  }

  function ensureNoOtherSession(kind) {
    const busyKind = debugState.pendingKind || debugState.activeKind;
    if (busyKind && busyKind !== kind) {
      showToast(t('debug_terminal_busy_other'), 'err');
      return false;
    }
    return true;
  }

  async function connectAdbDebug() {
    if (!ensureNoOtherSession('adb')) return;
    if (!ensureAdbSupported()) return;
    try {
      clearDebugTerminal('adb');
      debugState.pendingKind = 'adb';
      debugState.activeKind = null;
      state.debugTerminal.activeKind = null;
      updateDebugControls();
      updateTerminalState();
      setDebugStatus('adb', t('adb_debug_connecting'));
      await invoke('start_adb_session');
      focusDebugTerminal('adb');
    } catch (e) {
      debugState.pendingKind = null;
      updateDebugControls();
      updateTerminalState();
      setDebugStatus('adb', e.message || String(e));
      showToast('ADB 调试连接失败: ' + (e.message || String(e)), 'err');
    }
  }

  async function connectSshDebug() {
    if (!ensureNoOtherSession('ssh')) return;
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
      debugState.pendingKind = 'ssh';
      debugState.activeKind = null;
      state.debugTerminal.activeKind = null;
      updateDebugControls();
      updateTerminalState();
      setDebugStatus('ssh', `${t('ssh_debug_connecting')}: ${host}`);
      await invoke('start_ssh_session', { host, username, password });
      focusDebugTerminal('ssh');
    } catch (e) {
      debugState.pendingKind = null;
      updateDebugControls();
      updateTerminalState();
      setDebugStatus('ssh', e.message || String(e));
      showToast('SSH 调试连接失败: ' + (e.message || String(e)), 'err');
    }
  }

  async function disconnectDebugTerminal(kind) {
    const currentKind = kind || debugState.pendingKind || debugState.activeKind;
    if (!currentKind || (debugState.pendingKind !== currentKind && debugState.activeKind !== currentKind)) {
      showToast(t('debug_no_active_session'), 'info');
      return;
    }
    try {
      await invoke('close_debug_terminal_session');
    } catch (e) {
      console.warn('[DebugTerminal] 断开失败:', e);
    }
    resetDebugSession(currentKind, currentKind === 'adb' ? t('adb_debug_disconnected') : t('ssh_debug_disconnected'));
  }

  function queueDebugTerminalInput(kind, input) {
    if (debugState.activeKind !== kind) {
      const message = kind === 'adb' ? t('adb_debug_waiting') : t('ssh_debug_waiting');
      setDebugStatus(kind, message);
      return;
    }
    debugState.writeChain = debugState.writeChain
      .catch(() => {})
      .then(() => invoke('write_debug_terminal_input', { input }))
      .catch(e => {
        showToast('发送调试命令失败: ' + (e.message || String(e)), 'err');
      });
  }

  function keyToTerminalSequence(event) {
    if (event.metaKey) return null;
    if (event.ctrlKey && !event.altKey) {
      const lower = event.key.toLowerCase();
      if (lower === 'c') return '\u0003';
      if (lower === 'd') return '\u0004';
      if (lower === 'l') return '\u000c';
      return null;
    }
    if (event.key === 'Enter') return '\n';
    if (event.key === 'Tab') return '\t';
    if (event.key === 'Backspace') return '\u007f';
    if (event.key === 'Escape') return '\u001b';
    if (event.key === 'ArrowUp') return '\u001b[A';
    if (event.key === 'ArrowDown') return '\u001b[B';
    if (event.key === 'ArrowRight') return '\u001b[C';
    if (event.key === 'ArrowLeft') return '\u001b[D';
    if (event.key === 'Home') return '\u001b[H';
    if (event.key === 'End') return '\u001b[F';
    if (event.key === 'Delete') return '\u001b[3~';
    if (event.key.length === 1 && !event.altKey) return event.key;
    return null;
  }

  function handleDebugTerminalKey(event, kind) {
    const sequence = keyToTerminalSequence(event);
    if (sequence == null) return;
    event.preventDefault();
    queueDebugTerminalInput(kind, sequence);
  }

  function handleDebugTerminalPaste(event, kind) {
    const text = event.clipboardData?.getData('text');
    if (!text) return;
    event.preventDefault();
    queueDebugTerminalInput(kind, text);
  }

  function clearDebugTerminal(kind) {
    const terminal = getTerminal(kind);
    if (terminal) terminal.textContent = '';
  }

  function handleSystemMessage(kind, text) {
    const trimmed = (text || '').trim();
    if (!trimmed) return;

    if (kind === 'adb') {
      if (trimmed.includes('ADB shell 已启动')) {
        debugState.pendingKind = null;
        debugState.activeKind = 'adb';
        state.debugTerminal.activeKind = 'adb';
        setDebugStatus('adb', t('adb_debug_connected'));
        updateDebugControls();
        updateTerminalState();
        focusDebugTerminal('adb');
        return;
      }
      if (trimmed.includes('ADB shell 已退出') || trimmed.includes('启动 adb shell 失败') || trimmed.includes('写入 ADB shell 失败')) {
        resetDebugSession('adb', t('adb_debug_disconnected'));
        return;
      }
    }

    if (kind === 'ssh') {
      if (trimmed.includes('SSH shell 已连接')) {
        debugState.pendingKind = null;
        debugState.activeKind = 'ssh';
        state.debugTerminal.activeKind = 'ssh';
        setDebugStatus('ssh', t('ssh_debug_connected'));
        updateDebugControls();
        updateTerminalState();
        focusDebugTerminal('ssh');
        return;
      }
      if (trimmed.includes('SSH shell 已断开') || trimmed.includes('SSH shell 已退出') || trimmed.includes('SSH 用户名/密码认证失败') || trimmed.includes('连接 SSH 设备失败')) {
        resetDebugSession('ssh', t('ssh_debug_disconnected'));
      }
    }
  }

  function handleDebugOutputEvent(event) {
    const payload = event.payload || {};
    if (!payload.kind || !payload.text) return;
    appendDebugOutput(payload.kind, payload.text);
    if (payload.stream === 'system') {
      setDebugStatus(payload.kind, payload.text.trim() || payload.text);
      handleSystemMessage(payload.kind, payload.text);
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
      updateDebugControls();
      updateTerminalState();
    } catch (e) {
      console.warn('[DebugTerminal] 初始化失败:', e);
    }
  }

  function handlePageChange(prevPage, nextPage) {
    const leavingDebugPage = (prevPage === 'adbdebug' || prevPage === 'sshdebug') &&
      nextPage !== prevPage &&
      nextPage !== 'adbdebug' &&
      nextPage !== 'sshdebug';
    if (leavingDebugPage && (debugState.activeKind || debugState.pendingKind)) {
      disconnectDebugTerminal(debugState.activeKind || debugState.pendingKind).catch(() => {});
    }
    if (nextPage === 'adbdebug') {
      ensureAdbSupported();
      updateDebugControls();
      updateTerminalState();
    }
    if (nextPage === 'sshdebug') {
      refreshSshDebugAdapters().catch(() => {});
      updateDebugControls();
      updateTerminalState();
    }
  }

  window.initDebugTerminal = initDebugTerminal;
  window.connectAdbDebug = connectAdbDebug;
  window.connectSshDebug = connectSshDebug;
  window.disconnectDebugTerminal = disconnectDebugTerminal;
  window.clearDebugTerminal = clearDebugTerminal;
  window.refreshSshDebugAdapters = refreshSshDebugAdapters;
  window.handleSshAdapterChange = handleSshAdapterChange;
  window.focusDebugTerminal = focusDebugTerminal;
  window.handleDebugTerminalKey = handleDebugTerminalKey;
  window.handleDebugTerminalPaste = handleDebugTerminalPaste;
  window.handleDebugTerminalPageChange = handlePageChange;
})();
