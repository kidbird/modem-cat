    const invoke = window.__TAURI__?.core?.invoke;

    // ── State ──
    const state = {
      connected: false,
      dataConnected: false,
      dataApn: '',
      isDark: true,
      atHistory: [],
      atHistoryIdx: -1,
      bandConfig: null,
      connectedPort: '',
      idle: false,
    };

    // ── Data Connection ──
    async function toggleDataConnection() {
      const btn = document.getElementById('dataConnectBtn');
      if (state.dataConnected) {
        try {
          await invoke('disconnect_data');
          await flushAtLog();
          state.dataConnected = false;
          updateDataConnectionUI();
          addTerminalLine('[数据] 已断开数据连接', 'cmd');
        } catch (e) {
          addTerminalLine('[数据] 断开失败: ' + e, 'err');
        }
      } else {
        if (!state.connected) { alert('请先连接模组'); return; }
        btn.textContent = '连接中...';
        btn.classList.add('connecting');
        try {
          await invoke('connect_data');
          await flushAtLog();
          state.dataConnected = true;
          state.dataApn = '已连接';
          updateDataConnectionUI();
          addTerminalLine('[数据] 已建立数据连接', 'ok');
          refreshIpInfo();
        } catch (e) {
          addTerminalLine('[数据] 连接失败: ' + e, 'err');
          state.dataConnected = false;
          updateDataConnectionUI();
        }
      }
    }

    function updateDataConnectionUI() {
      const statusLabel = document.getElementById('dataStatusLabel');
      const icon = document.getElementById('dataDot');
      const btn = document.getElementById('dataConnectBtn');

      if (state.dataConnected) {
        statusLabel.textContent = state.dataApn || '已连接';
        statusLabel.classList.add('active');
        icon.classList.add('data-on');
        btn.textContent = '断开';
        btn.classList.add('active');
        btn.classList.remove('connecting');
      } else {
        statusLabel.textContent = '未连接';
        statusLabel.classList.remove('active');
        icon.classList.remove('data-on');
        btn.textContent = '连接';
        btn.classList.remove('active', 'connecting');
      }
    }

    // ── Theme ──
    (function () {
      const saved = localStorage.getItem('theme');
      if (saved === 'light') setTheme('light');
    })();

    function toggleTheme() { setTheme(state.isDark ? 'light' : 'dark'); }

    function setTheme(theme) {
      state.isDark = theme === 'dark';
      document.documentElement.setAttribute('data-theme', theme);
      localStorage.setItem('theme', theme);
      document.querySelector('.theme-toggle').textContent = state.isDark ? '☾' : '☀';
    }

    // ── Loading overlay ──
    function showLoading(text, sub) {
      document.getElementById('loadingText').textContent = text || '正在加载...';
      document.getElementById('loadingSub').textContent = sub || '';
      const overlay = document.getElementById('loadingOverlay');
      overlay.style.display = 'flex';
      // Force reflow to ensure animation starts reliably
      void overlay.offsetHeight;
    }
    function setLoadingText(text, sub) {
      document.getElementById('loadingText').textContent = text || '正在加载...';
      document.getElementById('loadingSub').textContent = sub || '';
    }
    function hideLoading() {
      document.getElementById('loadingOverlay').style.display = 'none';
    }

    // ── Toast notification ──
    let toastTimer = null;
    function showToast(text, type) {
      const toast = document.getElementById('toast');
      const toastText = document.getElementById('toastText');
      toastText.textContent = text;
      toast.style.background = type === 'ok' ? 'var(--success)' : 'var(--error)';
      toast.style.color = '#fff';
      toast.style.display = 'block';
      toast.style.opacity = '1';
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => {
        toast.style.opacity = '0';
        setTimeout(() => { toast.style.display = 'none'; }, 200);
      }, 500);
    }

    document.querySelectorAll('.nav-item:not(.disabled)').forEach(item => {
      item.addEventListener('click', function () {
        document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
        this.classList.add('active');
        document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
        document.getElementById('page-' + this.dataset.page).classList.add('active');
        if (this.dataset.page === 'hardware' && state.connected) {
          loadHardwarePage();
        }
      });
    });

    async function loadHardwarePage() {
      showLoading('正在加载系统信息...', '查询模组信息与功能开关');
      try {
        await refreshHardwareInfo();
        setLoadingText('正在加载系统信息...', '查询功能开关');
        await refreshFeatureToggles();
      } catch (e) {
        addTerminalLine('[系统] 加载失败: ' + e, 'err');
      }
      hideLoading();
    }

    // ── Connection ──
    async function toggleConnection() {
      const btn = document.getElementById('connectBtn');
      const label = document.getElementById('statusLabel');
      if (state.connected) {
        // 断开：先挂断拨号，再断开串口
        if (state.dataConnected) {
          try {
            await invoke('disconnect_data');
            await flushAtLog();
            addTerminalLine('[数据] 已挂断拨号', 'cmd');
          } catch (e) {
            addTerminalLine('[数据] 挂断拨号失败: ' + e, 'err');
          }
        }
        try {
          await invoke('disconnect');
        } catch (e) { /* ignore */ }
        state.connected = false;
        state.dataConnected = false;
        state.dataApn = '';
        state.connectedPort = '';
        state.idle = false;
        updateConnectionUI(false);
        updateDataConnectionUI();
        clearData();
        addTerminalLine('[连接] 已断开', 'cmd');
      } else {
        // 连接
        btn.textContent = '连接中...';
        btn.disabled = true;
        label.textContent = '正在检测AT端口...';
        showLoading('正在连接模组...', '检测 AT 端口');
        addTerminalLine('[连接] 正在检测AT端口...', 'info');
        try {
          const portName = await invoke('auto_connect_at');
          state.connected = true;
          state.idle = false;
          state.connectedPort = portName;
          document.getElementById('connectionParams').value = portName;
          updateConnectionUI(true);
          addTerminalLine(`[连接] 已连接到 ${portName}`, 'ok');
          label.textContent = portName;
          try {
            setLoadingText('正在获取模组数据...', '查询状态、硬件、频段信息');
            await refreshAll();
          } catch (e) {
            addTerminalLine('[刷新] 数据刷新异常: ' + e, 'err');
          }
          hideLoading();
        } catch (e) {
          hideLoading();
          const errMsg = String(e);
          console.error('[连接] auto_connect_at 失败:', errMsg);
          addTerminalLine('[连接] ' + errMsg, 'err');
          // No AT port — go idle gracefully
          state.connected = false;
          state.idle = true;
          updateConnectionUI(false);
          label.textContent = '待机中';
          label.style.color = 'var(--text-muted)';
        }
      }
    }

    function updateConnectionUI(connected) {
      const btn = document.getElementById('connectBtn');
      const icon = document.getElementById('statusDot');
      const label = document.getElementById('statusLabel');
      const connType = document.getElementById('connectionType');
      const connParams = document.getElementById('connectionParams');
      connType.disabled = connected;
      connParams.disabled = connected;
      if (connected) {
        btn.textContent = '断开';
        btn.disabled = false;
        btn.className = 'btn btn-danger';
        icon.classList.add('connected');
        label.style.color = '';
        const params = document.getElementById('connectionParams').value.trim();
        label.textContent = params || '已连接';
      } else {
        btn.textContent = '连接';
        btn.disabled = false;
        btn.className = 'btn btn-primary';
        icon.classList.remove('connected');
        if (state.idle) {
          label.textContent = '待机中';
          label.style.color = 'var(--text-muted)';
        } else {
          label.textContent = '未连接';
          label.style.color = '';
        }
      }
    }

    // ── 数据刷新 via AT adapter layer ──
    async function refreshAll() {
      if (!state.connected) return;
      showLoading('正在获取模组数据...', '查询网络状态');
      addTerminalLine('[刷新] 开始获取模组数据...', 'info');
      try {
        await refreshModemStatus();
      } catch (e) {
        addTerminalLine('[刷新] 模组状态获取失败: ' + e, 'err');
      }
      setLoadingText('正在获取模组数据...', '查询 IP 与 APN');
      try {
        await refreshIpInfo();
      } catch (e) {
        console.warn('refreshIpInfo failed:', e);
      }
      try {
        await refreshHardwareInfo();
      } catch (e) {
        console.warn('refreshHardwareInfo failed:', e);
      }
      try {
        await refreshApnList();
      } catch (e) {
        console.warn('refreshApnList failed:', e);
      }
      setLoadingText('正在获取模组数据...', '查询 QoS 与流量');
      try {
        await refreshQos();
      } catch (e) {
        console.warn('refreshQos failed:', e);
      }
      try {
        await refreshTraffic();
      } catch (e) {
        console.warn('refreshTraffic failed:', e);
      }
      hideLoading();
      addTerminalLine('[刷新] 数据刷新完成', 'info');
    }

    async function refreshModemStatus() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      showLoading('正在获取网络状态...', '请稍候');
      addTerminalLine('[状态] 正在查询模组状态...', 'info');
      try {
        const s = await invoke('get_modem_status');
        addTerminalLine(`[状态] SIM=${s.simStatus} 注册=${s.regStatus} 连接=${s.connStatus} 网络=${s.networkType} 运营商=${s.operator}`, 'info');
        if (s.simStatus === 'READY') setTextGood('simStatus', '已插入'); else if (s.simStatus === 'NO SIM') setTextWarn('simStatus', 'NO SIM'); else setText('simStatus', s.simStatus || '--');
        // Query actual SIM slot and update display
        querySimSlot(s.simStatus === 'READY');
        const noSim = s.simStatus !== 'READY';
        const regMap = { 'NOCONN': '已注册', 'CONNECT': '已注册', 'LIMSRV': '限制服务', 'SEARCH': '搜网' };
        const regText = regMap[s.regStatus] || s.regStatus || '--';
        const regEl = document.getElementById('regStatus');
        if (regEl) { regEl.textContent = regText; regEl.className = (s.regStatus === 'NOCONN' || s.regStatus === 'CONNECT') ? 'info-value good' : 'info-value'; }
        const connEl = document.getElementById('connStatus');
        if (connEl) { connEl.textContent = s.connStatus || '--'; connEl.className = s.connStatus === '已连接' ? 'info-value good' : 'info-value'; }
        setTextData('imei', noSim ? '--' : (s.imei || '--'));
        setTextData('iccid', noSim ? '--' : (s.iccid || '--'));
        setTextData('operator', noSim ? '--' : (s.operator || '--'));
        setTextData('networkType', noSim ? '--' : (s.networkType || '--'));
        setTextData('pci', noSim ? '--' : (s.pci || '--'));
        setTextData('cellid', noSim ? '--' : (s.cellId || '--'));
        setTextData('arfcn', noSim ? '--' : (s.arfcn || '--'));
        setTextData('bandwidth', noSim ? '--' : (s.bandwidth || '--'));
        setTextData('rsrp', noSim ? '--' : (s.rsrp || '--'));
        setTextData('rsrq', noSim ? '--' : (s.rsrq || '--'));
        setTextData('sinr', noSim ? '--' : (s.sinr || '--'));
        setTextData('txPower', noSim ? '--' : (s.txPower || '--'));
        setTextData('rxLevel', noSim ? '--' : (s.rxLevel || '--'));
        setTextData('ant0', noSim ? '' : (s.antValues?.[0] || ''));
        setTextData('ant1', noSim ? '' : (s.antValues?.[1] || ''));
        setTextData('ant2', noSim ? '' : (s.antValues?.[2] || ''));
        setTextData('ant3', noSim ? '' : (s.antValues?.[3] || ''));

        // SCS display
        const scsMap = { '0': '15 KHz', '1': '30 KHz', '2': '60 KHz', '3': '120 KHz', '4': '240 KHz' };
        setTextData('scs', noSim ? '--' : (scsMap[s.scs] || (s.scs ? s.scs + ' KHz' : '--')));

        // Update data connection state from CGACT
        if (s.connStatus === '已连接' && !state.dataConnected) {
          state.dataConnected = true;
          state.dataApn = '已连接';
          updateDataConnectionUI();
        } else if (s.connStatus !== '已连接' && state.dataConnected) {
          state.dataConnected = false;
          state.dataApn = '';
          updateDataConnectionUI();
        }
        showToast('刷新成功', 'ok');

      } catch (e) {
        console.error('Failed to refresh modem status:', e);
        addTerminalLine('[状态] 刷新失败：' + e, 'err');
        showToast('刷新失败：' + e, 'err');
      } finally {
        hideLoading();
      }
    }

    // ── SIM Slot Selector ──

    async function querySimSlot(simReady) {
      try {
        const slot = await invoke('get_sim_slot');
        updateSimSlotUI(slot, simReady);
      } catch (e) {
        console.warn('Query SIM slot failed:', e);
        updateSimSlotUI(1, false);
      }
    }

    function updateSimSlotUI(slot, simReady) {
      const swEl = document.getElementById('simSlot');
      if (swEl) swEl.textContent = 'SIM ' + slot;
      document.querySelectorAll('.sim-slot-option').forEach(opt => {
        opt.classList.toggle('active', parseInt(opt.dataset.slot) === slot);
      });
    }

    function toggleSimSlotDropdown() {
      const sel = document.getElementById('simSlotSwitch');
      if (!sel) return;
      sel.classList.toggle('open');
    }

    // Close dropdown on outside click
    document.addEventListener('click', (e) => {
      const sel = document.getElementById('simSlotSwitch');
      if (sel && !sel.contains(e.target)) sel.classList.remove('open');
    });

    async function switchSimSlot(slot) {
      const sel = document.getElementById('simSlotSelector');
      if (sel) sel.classList.remove('open');
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      try {
        showLoading('切换 SIM 槽...', '请稍候');
        addTerminalLine('[SIM] 切换到 SIM ' + slot + '...', 'info');
        await invoke('set_sim_slot', { slot });
        updateSimSlotUI(slot, true);
        addTerminalLine('[SIM] 切换成功，等待网络注册...', 'ok');
        showToast('已切换到 SIM ' + slot + '，等待网络注册...', 'ok');
        // Wait for modem to re-register on network before querying
        showLoading('等待网络注册...', '模组正在重新搜网');
        await new Promise(r => setTimeout(r, 5000));
        // Refresh all network status after SIM switch
        await refreshModemStatus();
      } catch (e) {
        addTerminalLine('[SIM] 切换失败：' + e, 'err');
        showToast('切换失败：' + e, 'err');
      } finally {
        hideLoading();
      }
    }

    function clearData() {
      const ids = ['simStatus','regStatus','connStatus','imei','iccid',
        'operator','networkType','pci','cellid','arfcn','bandwidth','rsrp','rsrq','sinr','txPower','rxLevel','cqi','scs',
        'ant0','ant1','ant2','ant3','ulBandwidth','dlBandwidth','ulTraffic','dlTraffic',
        'hwModel','hwManufacturer','hwFirmware','hwApBaseline','hwCpBaseline','hwSocTemp','hwPaTemp',
        'ipv4Addr','ipv4Mask','ipv4Gw','ipv4Dns','ipv6Addr','ipv6Dns'];
      ids.forEach(id => {
        const el = document.getElementById(id);
        if (el) { el.textContent = '--'; el.className = 'info-value muted'; }
      });
      document.getElementById('lteNeighborBody').innerHTML =
        '<tr><td colspan="6" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
      document.getElementById('nrNeighborBody').innerHTML =
        '<tr><td colspan="6" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
      document.getElementById('logoText').textContent = 'Modem Cat';
    }

    function setText(id, val) {
      const el = document.getElementById(id);
      if (el) { el.textContent = val; el.className = 'info-value'; }
    }
    function setTextGood(id, val) {
      const el = document.getElementById(id);
      if (el) { el.textContent = val; el.className = 'info-value good'; }
    }
    function setTextWarn(id, val) {
      const el = document.getElementById(id);
      if (el) { el.textContent = val; el.className = 'info-value warn'; }
    }
    function setTextData(id, val) {
      const el = document.getElementById(id);
      if (!el) return;
      el.textContent = val;
      el.className = (val && val !== '--' && val !== '') ? 'info-value data' : 'info-value muted';
    }

    // ── AT Terminal ──
    async function sendAtCommand() {
      const input = document.getElementById('atCommand');
      const cmd = input.value.trim();
      if (!cmd) return;

      state.atHistory.unshift(cmd);
      state.atHistoryIdx = -1;
      input.value = '';

      addTerminalLine('› ' + cmd, 'cmd');
      const startTime = performance.now();
      try {
        const resp = await invoke('send_raw_at', { command: cmd });
        const elapsed = Math.round(performance.now() - startTime);
        resp.split('\n').forEach(line => {
          const trimmed = line.trim();
          if (!trimmed) return;
          if (trimmed === 'OK') addTerminalLine('< ' + trimmed + '  [' + elapsed + 'ms]', 'ok');
          else if (trimmed.startsWith('ERROR')) addTerminalLine('< ' + trimmed + '  [' + elapsed + 'ms]', 'err');
          else addTerminalLine('< ' + trimmed, 'resp');
        });
      } catch (e) {
        const elapsed = Math.round(performance.now() - startTime);
        addTerminalLine('< ERROR: ' + e + '  [' + elapsed + 'ms]', 'err');
      }
    }

    function quickAt(cmd) {
      document.getElementById('atCommand').value = cmd;
      sendAtCommand();
    }

    function handleAtKey(e) {
      const input = document.getElementById('atCommand');
      if (e.key === 'Enter') { sendAtCommand(); return; }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        if (state.atHistoryIdx < state.atHistory.length - 1) {
          state.atHistoryIdx++;
          input.value = state.atHistory[state.atHistoryIdx];
        }
      }
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (state.atHistoryIdx > 0) {
          state.atHistoryIdx--;
          input.value = state.atHistory[state.atHistoryIdx];
        } else {
          state.atHistoryIdx = -1;
          input.value = '';
        }
      }
    }

    function addTerminalLine(text, cls) {
      const terminal = document.getElementById('terminal');
      const line = document.createElement('div');
      line.className = 'terminal-line' + (cls ? ' ' + cls : '');
      line.textContent = text;
      terminal.appendChild(line);
      terminal.scrollTop = terminal.scrollHeight;
    }

    async function flushAtLog() {
      try {
        const cmds = await invoke('pop_at_commands');
        for (const cmd of cmds) {
          addTerminalLine('›› ' + cmd, 'cmd');
        }
      } catch (_) {}
    }

    function clearTerminal() {
      document.getElementById('terminal').innerHTML = '';
    }

    // ── 蜂窝网络 Tab 切换 ──
    let neighborLoaded = false;
    let netlockLoaded = false;
    function switchCellularTab(tab, btn) {
      document.querySelectorAll('#page-cellular .tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('#page-cellular > .panel > .tab-panel').forEach(p => p.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById('ctab-' + tab).classList.add('active');
      // Lazy-load neighbor data on first tab switch
      if (tab === 'neighbor' && state.connected && !neighborLoaded) {
        neighborLoaded = true;
        loadNeighborCells();
      }
      // Load bands + network mode when switching to netlock tab
      if (tab === 'netlock' && state.connected) {
        loadNetlockData();
      }
      if (tab === 'apn' && state.connected) {
        refreshApnList();
      }
      if (tab === '5glan' && state.connected) {
        refresh5Glan();
      }
      // Query current cell lock when switching to celllock tab
      if (tab === 'celllock' && state.connected) {
        queryCellLock();
      }
    }

    // ── APN ──
    let apnData = [];
    let editingApnIdx = -1;

    function renderApnList() {
      const list = document.getElementById('apnList');
      if (apnData.length === 0) {
        list.innerHTML = '<div style="color:var(--text-muted);font-size:12px;padding:8px 0;">暂无 APN，请点击下方按钮添加</div>';
        return;
      }
      list.innerHTML = apnData.map((a, i) => `
        <div class="apn-item ${editingApnIdx === i ? 'selected' : ''}" onclick="editApn(${i})">
          <div class="apn-item-info">
            <div class="apn-item-name">${a.name}</div>
            <div class="apn-item-meta">${a.ip.toUpperCase()} · 鉴权: ${a.auth.toUpperCase()}${a.user ? ' · 用户: ' + a.user : ''}</div>
          </div>
          <button class="btn btn-secondary btn-sm ${a.active ? 'active' : ''}" onclick="event.stopPropagation();toggleApnActive(${i})">${a.active ? '已激活' : '激活'}</button>
          <button class="btn btn-secondary btn-sm" onclick="event.stopPropagation();deleteApn(${i})">删除</button>
        </div>
      `).join('');
    }

    function populateCidSelect(selectedCid) {
      const sel = document.getElementById('apnCid');
      const usedCids = new Set(apnData.map(a => a.cid));
      sel.innerHTML = '';
      for (let c = 1; c <= 16; c++) {
        if (c === selectedCid || !usedCids.has(c)) {
          const opt = document.createElement('option');
          opt.value = c;
          opt.textContent = c;
          if (c === selectedCid) opt.selected = true;
          sel.appendChild(opt);
        }
      }
      if (!selectedCid && sel.options.length > 0) sel.selectedIndex = 0;
    }

    function openApnModal() {
      editingApnIdx = -1;
      ['apnName','apnUser','apnPass'].forEach(id => document.getElementById(id).value = '');
      document.getElementById('apnAuth').value = 'none';
      document.getElementById('apnIp').value = 'ipv4';
      document.getElementById('apnModalTitle').textContent = '新增 APN';
      populateCidSelect(null);
      document.getElementById('apnModalOverlay').classList.add('active');
      renderApnList();
    }

    function editApn(i) {
      editingApnIdx = i;
      const a = apnData[i];
      document.getElementById('apnName').value = a.name;
      document.getElementById('apnUser').value = a.user;
      document.getElementById('apnPass').value = '';
      document.getElementById('apnAuth').value = a.auth;
      document.getElementById('apnIp').value = a.ip;
      document.getElementById('apnModalTitle').textContent = '编辑 APN';
      populateCidSelect(a.cid);
      document.getElementById('apnModalOverlay').classList.add('active');
      renderApnList();
    }

    function closeApnModal() {
      document.getElementById('apnModalOverlay').classList.remove('active');
      editingApnIdx = -1;
      renderApnList();
    }

    document.addEventListener('keydown', e => {
      if (e.key === 'Escape' && document.getElementById('apnModalOverlay').classList.contains('active')) {
        closeApnModal();
      }
    });

    async function saveApn() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const name = document.getElementById('apnName').value.trim();
      if (!name) { showToast('请输入 APN 名称', 'err'); return; }
      const cid = parseInt(document.getElementById('apnCid').value);
      const authMap = { none: 0, pap: 1, chap: 2 };
      const ipTypeMap = { ipv4: 1, ipv6: 2, ipv4v6: 3, ethernet: 4 };
      const auth = document.getElementById('apnAuth').value;
      const ipType = document.getElementById('apnIp').value;
      const user = document.getElementById('apnUser').value.trim();
      const pass = document.getElementById('apnPass').value;

      try {
        showLoading('正在保存 APN...');
        await invoke('set_apn_config', {
          cid,
          contextType: ipTypeMap[ipType] || 1,
          apn: name,
          username: user,
          password: pass,
          authType: authMap[auth] || 0,
        });
        await flushAtLog();
        hideLoading();
        showToast(`APN 已保存: ${name}`, 'ok');
        addTerminalLine(`[APN] 已保存: CID=${cid} ${name}`, 'ok');
        closeApnModal();
        await refreshApnList();
      } catch (e) {
        hideLoading();
        showToast('APN 保存失败: ' + e, 'err');
        addTerminalLine('[APN] 保存失败: ' + e, 'err');
      }
    }

    async function deleteApn(i) {
      if (!confirm(`删除 APN "${apnData[i].name}"？`)) return;
      if (!state.connected) { alert('请先连接模组'); return; }
      try {
        await invoke('delete_apn_config', { cid: apnData[i].cid });
        await flushAtLog();
        addTerminalLine(`[APN] 已删除: ${apnData[i].name}`, 'ok');
        await refreshApnList();
      } catch (e) {
        addTerminalLine('[APN] 删除失败: ' + e, 'err');
      }
    }

    async function toggleApnActive(i) {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const cid = apnData[i].cid;
      const active = !apnData[i].active;
      try {
        showLoading(active ? '正在激活 APN...' : '正在去激活 APN...');
        await invoke('set_apn_active', { cid, active });
        await flushAtLog();
        hideLoading();
        showToast(active ? `APN 已激活: ${apnData[i].name}` : `APN 已去激活: ${apnData[i].name}`, 'ok');
        addTerminalLine(`[APN] ${active ? '激活' : '去激活'}: CID=${cid} ${apnData[i].name}`, 'ok');
        await refreshApnList();
      } catch (e) {
        hideLoading();
        showToast('操作失败: ' + e, 'err');
        addTerminalLine('[APN] 操作失败: ' + e, 'err');
      }
    }

    // ── 5GLAN ──
    let glanData = [];

    function render5Glan() {
      const container = document.getElementById('glanToggles');
      if (!glanData.length) {
        container.innerHTML = '<div style="color:var(--text-muted);font-size:12px;padding:8px 0;">暂无 5GLAN 数据</div>';
        return;
      }
      container.innerHTML = glanData.map(g => `
        <div class="toggle-row" style="align-items:center;border-bottom:1px solid var(--border-color);">
          <div><div class="toggle-label">CID ${g.cid}</div></div>
          <label class="toggle-switch"><input type="checkbox" ${g.enabled ? 'checked' : ''} onchange="set5Glan(${g.cid}, this.checked)"><span class="toggle-track"></span></label>
        </div>
      `).join('');
    }

    async function refresh5Glan() {
      if (!state.connected) { render5Glan(); return; }
      try {
        glanData = await invoke('get_5glan');
        render5Glan();
      } catch (e) {
        console.warn('refresh5Glan failed:', e);
        glanData = [];
        render5Glan();
      }
    }

    async function set5Glan(cid, enabled) {
      if (!state.connected) return;
      try {
        await invoke('set_5glan', { cid, enabled });
        await flushAtLog();
        addTerminalLine(`[5GLAN] CID${cid} → ${enabled ? '开启' : '关闭'}`, 'ok');
        await refresh5Glan();
      } catch (e) {
        showToast('5GLAN 设置失败: ' + e, 'err');
        addTerminalLine('[5GLAN] 设置失败: ' + e, 'err');
        await refresh5Glan();
      }
    }

    // ── 网络锁定 ──
    async function loadNetlockData() {
      if (!state.connected) return;
      // Load preferred network mode
      try {
        const mode = await invoke('get_network_mode');
        const modeMap = { 'AUTO': 'auto', 'NR5G': 'nr5g', 'NR5G-SA': '5gsa', 'NR5G-NSA': 'nrNsa', 'LTE': 'lte', 'WCDMA': 'wcdma' };
        const sel = document.getElementById('preferredNetwork');
        sel.value = modeMap[mode] || 'auto';
      } catch (e) {
        console.warn('get_network_mode failed:', e);
      }
      // Load bands independently
      await refreshBands();
    }

    async function applyPreferredNetwork() {
      const sel = document.getElementById('preferredNetwork');
      const modeMap = { auto: 'AUTO', nr5g: 'NR5G', '5gsa': 'NR5G-SA', nrNsa: 'NR5G-NSA', lte: 'LTE', wcdma: 'WCDMA' };
      try {
        showLoading('正在保存首选网络...');
        await invoke('set_network_mode_cmd', { mode: modeMap[sel.value] || sel.value });
        await flushAtLog();
        hideLoading();
        showToast('首选网络保存成功', 'ok');
        addTerminalLine(`[网络] 首选网络: ${sel.selectedOptions[0].text}`, 'ok');
        try {
          const mode = await invoke('get_network_mode');
          addTerminalLine(`[网络] 回读原始值: ${mode}`, 'info');
          const modeMapBack = { 'AUTO': 'auto', 'NR5G': 'nr5g', 'NR5G-SA': '5gsa', 'NR5G-NSA': 'nrNsa', 'LTE': 'lte', 'WCDMA': 'wcdma' };
          sel.value = modeMapBack[mode] || 'auto';
        } catch (e2) { console.warn('get_network_mode after save failed:', e2); }
      } catch (e) {
        hideLoading();
        showToast('首选网络保存失败: ' + e, 'err');
        addTerminalLine('[网络] 设置失败: ' + e, 'err');
      }
    }

    // ── Band selection ──
    function renderBandGrid(bands, locked, gridId, invalid) {
      const grid = document.getElementById(gridId);
      if (!bands || bands.length === 0) {
        const msg = document.createElement('div');
        msg.style.cssText = 'color:var(--text-muted);font-size:12px;padding:8px;';
        msg.textContent = '暂无数据';
        grid.replaceChildren(msg);
        return;
      }
      grid.innerHTML = '';
      // Render supported bands
      bands.forEach(band => {
        const chip = document.createElement('label');
        chip.className = 'band-chip';
        chip.dataset.band = band;
        const input = document.createElement('input');
        input.type = 'checkbox';
        const span = document.createElement('span');
        span.textContent = band;
        chip.appendChild(input);
        chip.appendChild(span);
        if (locked.includes(band)) {
          chip.classList.add('checked');
        }
        chip.addEventListener('click', function (e) {
          e.preventDefault();
          this.classList.toggle('checked');
        });
        grid.appendChild(chip);
      });
      // Append invalid bands (locked but not hardware-supported)
      if (invalid && invalid.length > 0) {
        invalid.forEach(band => {
          const chip = document.createElement('label');
          chip.className = 'band-chip invalid checked';
          chip.dataset.band = band;
          chip.title = '此频段超出硬件支持范围';
          const input = document.createElement('input');
          input.type = 'checkbox';
          const span = document.createElement('span');
          span.textContent = band + ' ⚠';
          chip.appendChild(input);
          chip.appendChild(span);
          chip.addEventListener('click', function (e) {
            e.preventDefault();
            this.classList.toggle('checked');
          });
          grid.appendChild(chip);
        });
      }
    }

    async function refreshBands() {
      if (!state.connected) return;
      try {
        const cfg = await invoke('get_bands');
        state.bandConfig = cfg;
        renderBandGrid(cfg.lteSupported, cfg.lteLocked, 'bandGridLte', cfg.lteInvalid);
        renderBandGrid(cfg.nrSupported, cfg.nrLocked, 'bandGridNr', cfg.nrInvalid);
      } catch (e) {
        console.warn('refreshBands failed:', e);
      }
    }

    async function applyBandLock() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const lteChecked = [...document.querySelectorAll('#bandGridLte .band-chip.checked')]
        .map(c => c.dataset.band.replace('B', ''));
      const nrChecked = [...document.querySelectorAll('#bandGridNr .band-chip.checked')]
        .map(c => c.dataset.band.replace('n', ''));
      if (lteChecked.length === 0 && nrChecked.length === 0) { showToast('请选择至少一个频段', 'err'); return; }
      const lteStr = lteChecked.join(':');
      const nrStr = nrChecked.join(':');
      try {
        showLoading('正在保存频段配置...');
        await invoke('set_bands', { lte: lteStr, nr: nrStr });
        await flushAtLog();
        hideLoading();
        showToast('频段配置保存成功', 'ok');
        addTerminalLine(`[频段选择] LTE: ${lteChecked.length > 0 ? lteChecked.map(b => 'B' + b).join(', ') : '无'} | NR: ${nrChecked.length > 0 ? nrChecked.map(b => 'n' + b).join(', ') : '无'}`, 'ok');
        await refreshBands();
      } catch (e) {
        hideLoading();
        showToast('频段配置保存失败: ' + e, 'err');
        addTerminalLine('[频段选择] 保存失败: ' + e, 'err');
      }
    }

    async function resetBandLock() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      if (!confirm('确认重置为模组默认频段？')) return;
      try {
        showLoading('正在重置频段...', '请稍候');
        await invoke('reset_all_bands');
        await flushAtLog();
        hideLoading();
        showToast('频段已重置为默认值', 'ok');
        addTerminalLine('[频段选择] 已重置为默认频段', 'ok');
        await refreshBands();
      } catch (e) {
        hideLoading();
        showToast('频段重置失败: ' + e, 'err');
        addTerminalLine('[频段选择] 重置失败: ' + e, 'err');
      }
    }

    async function applyOperatorLock() {
      const mcc = document.getElementById('lockMcc').value.trim();
      const mnc = document.getElementById('lockMnc').value.trim();
      if (!mcc || !mnc) { showToast('请输入 MCC 和 MNC', 'err'); return; }
      const plmn = mcc + mnc;
      try {
        showLoading('正在锁定PLMN...');
        await invoke('send_raw_at', { command: 'AT+QSIMLOCK="PN","12345678",2,"' + plmn + '"' });
        await flushAtLog();
        hideLoading();
        showToast('PLMN 锁定成功', 'ok');
        addTerminalLine(`[PLMN锁定] MCC=${mcc} MNC=${mnc}`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('PLMN 锁定失败: ' + e, 'err');
        addTerminalLine('[PLMN锁定] 失败: ' + e, 'err');
      }
    }

    async function clearOperatorLock() {
      try {
        showLoading('正在解锁PLMN...');
        await invoke('send_raw_at', { command: 'AT+QSIMLOCK="PN","12345678"' });
        await flushAtLog();
        hideLoading();
        showToast('PLMN 已解锁', 'ok');
        addTerminalLine('[PLMN锁定] 已解锁', 'ok');
        document.getElementById('lockMcc').value = '';
        document.getElementById('lockMnc').value = '';
      } catch (e) {
        hideLoading();
        showToast('解锁失败: ' + e, 'err');
        addTerminalLine('[PLMN锁定] 解锁失败: ' + e, 'err');
      }
    }

    // ── 小区 / 频点锁定 ──

    function parseLockResponse(resp, prefix) {
      const items = [];
      const lines = resp.split('\n').map(l => l.trim().replace(/^\+/, ''));
      for (const line of lines) {
        if (!line.startsWith(prefix + ':')) continue;
        const data = line.substring(prefix.length + 1).trim();
        const parts = data.split(',').map(s => s.trim().replace(/"/g, ''));
        // Response format: "common/5g",<enable>,<freq>[,<pci>]
        // or: "common/5g",<freq> (when enable is omitted, freq is present = locked)
        if (parts.length >= 2) {
          let freq, pci;
          if (parts.length >= 3 && (parts[1] === '0' || parts[1] === '1')) {
            // format: "common/5g",1,freq[,pci]
            if (parts[1] !== '1') continue;
            freq = parts[2];
            pci = parts.length >= 4 ? parts[3] : '';
          } else {
            // format: "common/5g",freq  (no enable flag, presence = locked)
            freq = parts[1];
            pci = parts.length >= 3 ? parts[2] : '';
          }
          if (freq && freq !== '0') {
            items.push({ freq, pci });
          }
        }
      }
      return items;
    }

    async function queryCellLock() {
      if (!state.connected) return;
      const quota = document.getElementById('lockQuota');
      const list = document.getElementById('lockList');
      const allItems = [];
      try {
        const lockResp = await invoke('send_raw_at', { command: 'AT+QNWLOCK="common/5g"' });
        await flushAtLog();
        addTerminalLine('[锁定查询] QNWLOCK => ' + lockResp.trim(), 'info');
        const cellItems = parseLockResponse(lockResp, 'QNWLOCK');
        for (const e of cellItems) {
          allItems.push({ type: '小区', freq: e.freq, pci: e.pci });
        }
      } catch (e) {
        addTerminalLine('[锁定查询] QNWLOCK 失败: ' + e, 'err');
      }
      try {
        const freqResp = await invoke('send_raw_at', { command: 'AT+QNWLOCKFREQ="common/5g"' });
        await flushAtLog();
        addTerminalLine('[锁定查询] QNWLOCKFREQ => ' + freqResp.trim(), 'info');
        const freqItems = parseLockResponse(freqResp, 'QNWLOCKFREQ');
        for (const e of freqItems) {
          allItems.push({ type: '频点', freq: e.freq, pci: '' });
        }
      } catch (e) {
        addTerminalLine('[锁定查询] QNWLOCKFREQ 失败: ' + e, 'err');
      }

      if (allItems.length > 0) {
        quota.textContent = `已锁定 ${allItems.length} 条`;
        list.innerHTML = allItems.map(e => `
          <div class="lock-item">
            <span class="lock-item-badge">${e.type}</span>
            <span class="lock-item-info">频点 ${e.freq}${e.pci ? '  PCI ' + e.pci : ''}</span>
          </div>
        `).join('');
      } else {
        quota.textContent = '当前锁定：无';
        list.innerHTML = '';
      }
    }

    async function saveCellLock() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const arfcn = document.getElementById('lockArfcn').value.trim();
      const pci = document.getElementById('lockPci').value.trim();
      if (!arfcn) { showToast('请输入频点', 'err'); return; }
      try {
        showLoading('正在保存锁定...');
        let cmd;
        if (pci) {
          cmd = 'AT+QNWLOCK="common/5g",1,' + arfcn + ',' + pci;
        } else {
          cmd = 'AT+QNWLOCKFREQ="common/5g",1,' + arfcn;
        }
        const resp = await invoke('send_raw_at', { command: cmd });
        await flushAtLog();
        hideLoading();
        if (resp.trim().includes('OK')) {
          showToast('锁定保存成功', 'ok');
          addTerminalLine(`[锁定] ${pci ? '小区+频点' : '仅频点'} 频点=${arfcn}${pci ? ' PCI=' + pci : ''}`, 'ok');
        } else {
          showToast('锁定失败: 模组未返回OK', 'err');
          addTerminalLine('[锁定] 失败: ' + resp.trim(), 'err');
        }
        await queryCellLock();
      } catch (e) {
        hideLoading();
        showToast('锁定失败: ' + e, 'err');
        addTerminalLine('[锁定] 失败: ' + e, 'err');
      }
    }

    async function clearCellLock() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      try {
        showLoading('正在清除锁定...');
        // Clear both cell locks and frequency locks
        await invoke('send_raw_at', { command: 'AT+QNWLOCK="common/5g",0' });
        await flushAtLog();
        await invoke('send_raw_at', { command: 'AT+QNWLOCKFREQ="common/5g",0' });
        await flushAtLog();
        hideLoading();
        showToast('锁定已清除', 'ok');
        addTerminalLine('[锁定] 已清除全部', 'ok');
        document.getElementById('lockArfcn').value = '';
        document.getElementById('lockPci').value = '';
        await queryCellLock();
      } catch (e) {
        hideLoading();
        showToast('清除失败: ' + e, 'err');
        addTerminalLine('[锁定] 清除失败: ' + e, 'err');
      }
    }

    // ── 邻区信息 ──
    function switchNeighborTab(tab, btn) {
      const neighborPanel = document.getElementById('ctab-neighbor');
      neighborPanel.querySelectorAll('.sub-tab-btn').forEach(b => b.classList.remove('active'));
      neighborPanel.querySelectorAll('.tab-panel').forEach(p => p.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById('tab-' + tab).classList.add('active');
    }

    async function loadNeighborCells() {
      if (!state.connected) return;
      showLoading('正在获取模组数据...', '查询邻区信息');
      addTerminalLine('[邻区] 正在获取...', 'info');
      try {
        const result = await invoke('get_neighbor_cells');
        populateLteNeighbors(result.lte || []);
        populateNrNeighbors(result.nr || []);
        addTerminalLine(`[邻区] LTE ${(result.lte || []).length} 个, NR ${(result.nr || []).length} 个`, 'ok');
      } catch (e) {
        addTerminalLine('[邻区] 获取失败: ' + e, 'err');
      }
      hideLoading();
    }

    async function refreshNeighbors() {
      if (!state.connected) return;
      showLoading('正在获取模组数据...', '刷新邻区信息');
      addTerminalLine('[邻区] 正在刷新...', 'info');
      try {
        const result = await invoke('get_neighbor_cells');
        populateLteNeighbors(result.lte || []);
        populateNrNeighbors(result.nr || []);
        addTerminalLine(`[邻区] LTE ${(result.lte || []).length} 个, NR ${(result.nr || []).length} 个`, 'ok');
      } catch (e) {
        addTerminalLine('[邻区] 刷新失败: ' + e, 'err');
      }
      hideLoading();
    }

    function populateLteNeighbors(rows) {
      if (!rows || rows.length === 0) {
        document.getElementById('lteNeighborBody').innerHTML = '<tr><td colspan="4" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
        return;
      }
      document.getElementById('lteNeighborBody').innerHTML = rows.map(r =>
        `<tr><td>${r.pci || '--'}</td><td>${r.rsrp || '--'}</td><td>${r.rsrq || '--'}</td><td>${r.earfcn || '--'}</td></tr>`
      ).join('');
    }

    function populateNrNeighbors(rows) {
      if (!rows || rows.length === 0) {
        document.getElementById('nrNeighborBody').innerHTML = '<tr><td colspan="5" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
        return;
      }
      document.getElementById('nrNeighborBody').innerHTML = rows.map(r =>
        `<tr><td>${r.pci || '--'}</td><td>${r.rsrp || '--'}</td><td>${r.rsrq || '--'}</td><td>${r.sinr || '--'}</td><td>${r.arfcn || '--'}</td></tr>`
      ).join('');
    }

    // ── IP 信息 ──
    async function refreshIpInfo() {
      if (!state.connected) return;
      try {
        const ip = await invoke('get_ip_info');
        setTextData('ipv4Addr', ip.ipv4Addr || '--');
        setTextData('ipv4Mask', ip.ipv4Mask || '--');
        setTextData('ipv4Gw', ip.ipv4Gw || '--');
        setTextData('ipv4Dns', ip.ipv4Dns || '--');
        setTextData('ipv6Addr', ip.ipv6Addr || '--');
        setTextData('ipv6Dns', ip.ipv6Dns || '--');
      } catch (e) {
        console.warn('refreshIpInfo failed:', e);
      }
    }

    function applyMtu() {
      const val = parseInt(document.getElementById('mtuValue').value);
      if (isNaN(val) || val < 576 || val > 9000) { alert('MTU 范围：576 ~ 9000'); return; }
      addTerminalLine(`[MTU] 已设置: ${val} 字节`, 'ok');
    }

    async function applyDmz() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const ip = document.getElementById('dmzHost').value.trim();
      if (!ip) { showToast('请输入 DMZ 主机 IP 地址', 'err'); return; }
      try {
        showLoading('正在设置 DMZ...');
        await invoke('send_raw_at', { command: 'AT+QDMZ=1,4,' + ip });
        await flushAtLog();
        hideLoading();
        showToast('DMZ 设置成功', 'ok');
        addTerminalLine(`[DMZ] 已设置主机: ${ip}`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('DMZ 设置失败: ' + e, 'err');
        addTerminalLine('[DMZ] 设置失败: ' + e, 'err');
      }
    }

    async function clearDmz() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      try {
        showLoading('正在清除 DMZ...');
        await invoke('send_raw_at', { command: 'AT+QDMZ=0,4' });
        await flushAtLog();
        hideLoading();
        showToast('DMZ 已清除', 'ok');
        document.getElementById('dmzHost').value = '';
        addTerminalLine('[DMZ] 已清除', 'ok');
      } catch (e) {
        hideLoading();
        showToast('DMZ 清除失败: ' + e, 'err');
        addTerminalLine('[DMZ] 清除失败: ' + e, 'err');
      }
    }

    // ── 系统操作 ──
    async function refreshHardwareInfo() {
      if (!state.connected) { addTerminalLine('[系统] 未连接', 'err'); return; }
      try {
        const hw = await invoke('get_hardware_info');
        setTextData('hwModel', hw.model || '--');
        setTextData('hwManufacturer', hw.manufacturer || '--');
        setTextData('hwFirmware', hw.firmware || '--');
        setTextData('hwApBaseline', hw.apBaseline || '--');
        setTextData('hwCpBaseline', hw.cpBaseline || '--');
        setTextData('hwSocTemp', hw.socTemp || '--');
        setTextData('hwPaTemp', hw.paTemp || '--');
        if (hw.model) {
          document.getElementById('logoText').textContent = hw.model;
        }
        addTerminalLine(`[系统] 型号=${hw.model} 固件=${hw.firmware}`, 'info');
      } catch (e) {
        addTerminalLine('[系统] 读取失败: ' + e, 'err');
      }
    }

    async function refreshFeatureToggles() {
      if (!state.connected) return;
      try {
        const t = await invoke('get_feature_toggles');
        document.getElementById('togglePcie').checked = t.pcieMode;
        document.getElementById('toggleEthernet').checked = t.ethernet;
        document.getElementById('toggleEthAt').checked = t.ethAt;
        document.getElementById('toggleUartAt').checked = t.uartAt;
        document.getElementById('toggleAdb').checked = t.adb;
        document.getElementById('toggleProxyArp').checked = t.proxyarp;
        document.getElementById('toggleNapt').checked = t.napt;
        document.getElementById('toggleNetmask').checked = t.netmask;
      } catch (e) {
        console.warn('refreshFeatureToggles failed:', e);
      }
      try {
        const mode = await invoke('get_usbnet_mode');
        const sel = document.getElementById('usbNetMode');
        sel.value = String(mode);
      } catch (e) {
        console.warn('refreshUsbnet failed:', e);
      }
    }

    async function refreshQos() {
      if (!state.connected) return;
      try {
        const q = await invoke('get_qos_info');
        setTextData('cqi', q.cqi || '--');
        setTextData('ulBandwidth', q.ulBandwidth || '--');
        setTextData('dlBandwidth', q.dlBandwidth || '--');
      } catch (e) {
        console.warn('refreshQos failed:', e);
      }
    }

    function formatBytes(bytes) {
      if (bytes === 0) return '0 B';
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    }

    async function refreshTraffic() {
      if (!state.connected) return;
      try {
        const t = await invoke('get_traffic');
        setTextData('ulTraffic', formatBytes(t.ulBytes));
        setTextData('dlTraffic', formatBytes(t.dlBytes));
      } catch (e) {
        console.warn('refreshTraffic failed:', e);
      }
    }

    async function saveUsbNetMode() {
      const mode = document.getElementById('usbNetMode');
      const label = mode.selectedOptions[0].text;
      const value = parseInt(mode.value);
      if (!confirm(`切换 USB 网卡模式为「${label}」？\n模组将自动重启，连接会短暂中断。`)) return;
      if (!state.connected) { addTerminalLine('[USB] 未连接', 'err'); return; }
      try {
        await invoke('set_usbnet_mode', { mode: value });
        await flushAtLog();
        addTerminalLine(`[USB] 网卡模式切换为: ${label}，正在重启...`, 'cmd');
        // Reboot — device will disconnect, so mark disconnected immediately
        try { await invoke('reboot_modem'); await flushAtLog(); } catch (_) {}
        // Clean up backend transport
        invoke('disconnect').catch(() => {});
        // Proactively disconnect since the device is rebooting
        state.connected = false;
        state.dataConnected = false;
        state.connectedPort = '';
        state.idle = true;
        updateConnectionUI(false);
        updateDataConnectionUI();
        clearData();
        const statusLabel = document.getElementById('statusLabel');
        statusLabel.textContent = '重启中...';
        statusLabel.style.color = 'var(--warning, orange)';
        addTerminalLine('[USB] 模组正在重启，等待设备重新上线...', 'info');
      } catch (e) {
        addTerminalLine('[USB] 设置失败: ' + e, 'err');
      }
    }

    async function applyToggle(feature, enabled) {
      const labels = { adb: 'ADB', ethAt: 'ETH AT', uartAt: 'UART AT', pcieMode: 'PCIe ↔ 以太网', ethernet: 'Ethernet', proxyArp: 'Proxy ARP', napt: 'NAPT 端口转换', netmask: '动态子网掩码' };
      if (!state.connected) return;
      try {
        await invoke('set_feature_toggle', { feature, enabled });
        await flushAtLog();
        addTerminalLine(`[功能] ${labels[feature]} 已${enabled ? '开启' : '关闭'}`, enabled ? 'ok' : 'info');
      } catch (e) {
        addTerminalLine(`[功能] ${labels[feature]} 设置失败: ${e}`, 'err');
        // Revert checkbox
        const map = { adb: 'toggleAdb', ethAt: 'toggleEthAt', uartAt: 'toggleUartAt', pcieMode: 'togglePcie', ethernet: 'toggleEthernet', proxyArp: 'toggleProxyArp', napt: 'toggleNapt', netmask: 'toggleNetmask' };
        const el = document.getElementById(map[feature]);
        if (el) el.checked = !enabled;
      }
    }

    async function confirmAction(action) {
      const labels = { reboot: '重启模组', factory: '恢复出厂设置（所有配置将丢失）' };
      if (!confirm(`确认要执行：${labels[action]}？`)) return;
      addTerminalLine(`[操作] 正在执行: ${labels[action]}...`, 'cmd');
      try {
        if (action === 'reboot') { try { await invoke('reboot_modem'); } catch (_) {} }
        else if (action === 'factory') { try { await invoke('factory_reset'); } catch (_) {} }
        await flushAtLog();
        // Clean up backend transport
        invoke('disconnect').catch(() => {});
        // Both actions cause reboot — mark disconnected, wait for device to come back
        state.connected = false;
        state.dataConnected = false;
        state.connectedPort = '';
        state.idle = true;
        updateConnectionUI(false);
        updateDataConnectionUI();
        clearData();
        const statusLabel = document.getElementById('statusLabel');
        statusLabel.textContent = '重启中...';
        statusLabel.style.color = 'var(--warning, orange)';
        addTerminalLine('[操作] 模组正在重启，等待设备重新上线...', 'info');
      } catch (e) {
        addTerminalLine('[操作] 执行失败: ' + e, 'err');
      }
    }

    // ── APN list refresh ──
    async function refreshApnList() {
      if (!state.connected) { renderApnList(); return; }
      try {
        const list = await invoke('get_apn_list');
        console.log('[APN] raw list:', list);
        apnData = (list || []).map(a => ({
          name: a.apnName || '',
          user: a.username || '',
          pass: '',
          auth: ['none', 'pap', 'chap'][a.authType] || 'none',
          ip: a.ipType ? a.ipType.toLowerCase() : 'ipv4',
          active: !!a.active,
          cid: a.cid || 0,
        }));
      } catch (e) {
        console.warn('refreshApnList failed:', e);
        apnData = [];
      }
      renderApnList();
    }

    // ── 初始化 ──
    renderApnList();

    // ── USB 插拔检测：监听从 Rust 后端发出的事件 ──
    function setupUsbMonitor() {
      const listen = window.__TAURI__?.event?.listen;
      if (!listen) {
        console.warn('[USB] Tauri event system not available');
        return;
      }
      listen('port-changed', (event) => {
        const { added, removed } = event.payload;
        const activeConn = document.getElementById('connectionType').value;
        const isSerialMode = activeConn === 'serial';

        // Disconnected port matches our current connection
        if (state.connected && state.connectedPort && removed.includes(state.connectedPort)) {
          console.log('[USB] Connected port removed, disconnecting');
          addTerminalLine('[USB] AT端口已拔出，断开连接', 'cmd');
          // Force disconnect
          if (state.dataConnected) {
            invoke('disconnect_data').catch(() => {});
          }
          invoke('disconnect').catch(() => {});
          state.connected = false;
          state.dataConnected = false;
          state.dataApn = '';
          state.connectedPort = '';
          state.idle = true;
          updateConnectionUI(false);
          updateDataConnectionUI();
          clearData();
          const label = document.getElementById('statusLabel');
          label.textContent = '待机中';
          label.style.color = 'var(--text-muted)';
        }

        // New port appeared and we're idle → auto-connect
        if (!state.connected && state.idle && added.length > 0) {
          console.log('[USB] New port detected, trying auto-connect');
          addTerminalLine('[USB] 检测到新端口，等待设备就绪后连接...', 'info');
          // Wait for modem to fully boot before probing AT
          setTimeout(() => {
            if (!state.connected && state.idle) toggleConnection();
          }, 5000);
        }
      });
    }

    async function doInit() {
      const label = document.getElementById('statusLabel');
      label.textContent = '正在初始化...';
      showLoading('正在初始化...', '扫描串口端口');
      try {
        await refreshPortList();
      } catch (e) {
        addTerminalLine('[初始化] 端口列表失败: ' + e, 'err');
      }
      // 自动连接（USB 模式）
      try {
        await toggleConnection();
      } catch (e) {
        hideLoading();
        addTerminalLine('[初始化] 连接失败: ' + e, 'err');
      }
      // 启动 USB 监控
      setupUsbMonitor();
    }

    // Tauri v2: withGlobalTauri ensures window.__TAURI__ is available before scripts run
    // Guard against double-init with a flag
    let _initStarted = false;
    function safeDoInit() {
      if (_initStarted || state.connected) return;
      _initStarted = true;
      console.log('[Init] Starting initialization');
      doInit();
    }

    if (window.__TAURI__) {
      // IPC ready, init immediately
      console.log('[Init] __TAURI__ available, initializing immediately');
      safeDoInit();
    } else {
      console.warn('[Init] __TAURI__ not available, waiting for DOMContentLoaded');
      window.addEventListener('DOMContentLoaded', safeDoInit);
    }

    // Fallback: ensure init runs even if the above mechanisms fail
    setTimeout(() => {
      if (!_initStarted && !state.connected) {
        console.log('[Init] Timeout fallback triggering init');
        safeDoInit();
      }
    }, 3000);

    // ── 刷新串口列表 ──
    async function refreshPortList() {
      const select = document.getElementById('connectionParams');
      try {
        const ports = await invoke('list_ports');
        select.innerHTML = '<option value="">-- 选择端口 --</option>';
        if (ports.length === 0) {
          select.innerHTML += '<option value="" disabled>未找到串口</option>';
        } else {
          // AT ports first, then others
          const sorted = [...ports].sort((a, b) => (b.isAtPort ? 1 : 0) - (a.isAtPort ? 1 : 0));
          sorted.forEach(port => {
            const option = document.createElement('option');
            option.value = port.portName;
            option.textContent = port.displayName || port.portName;
            if (port.isAtPort) option.style.color = 'var(--accent)';
            select.appendChild(option);
          });
          // Auto-select the first AT port
          const atPort = ports.find(p => p.isAtPort);
          if (atPort) select.value = atPort.portName;
        }
      } catch (e) {
        console.error('Failed to list ports:', e);
        select.innerHTML = '<option value="">刷新失败: ' + String(e).slice(0, 60) + '</option>';
      }
    }

    // ── Timing display ──
    function displayTimingStats(stats, totalMs) {
      const entries = stats.entries || [];
      if (entries.length === 0) return;

      addTerminalLine('═══════════════════════════════════════', 'info');
      addTerminalLine('[时延分析] 详细耗时 breakdown:', 'info');
      addTerminalLine('───────────────────────────────────────', 'info');

      // Group by phase
      const phases = {};
      for (const e of entries) {
        if (!phases[e.phase]) phases[e.phase] = { items: [], total: 0 };
        phases[e.phase].items.push(e);
        phases[e.phase].total += e.durationMs;
      }

      // Transport level: individual AT commands
      const atEntries = entries.filter(e => e.phase === 'transport');
      if (atEntries.length > 0) {
        addTerminalLine(`[AT指令] 共 ${atEntries.length} 条:`, 'info');
        for (const e of atEntries) {
          const bar = '█'.repeat(Math.max(1, Math.round(e.durationMs / 20)));
          const status = e.success ? '✓' : '✗';
          addTerminalLine(`  ${status} ${e.command.padEnd(35)} ${String(e.durationMs).padStart(5)}ms ${bar}`, e.success ? 'info' : 'err');
        }
        const atTotal = atEntries.reduce((s, e) => s + e.durationMs, 0);
        addTerminalLine(`  AT指令总耗时: ${atTotal}ms`, 'info');
      }

      // Query level phases
      const queryPhases = ['query_modem_status', 'query_hardware_info', 'query_feature_toggles',
        'query_ip_info', 'query_apn_list', 'query_bands', 'query_neighbor_cells',
        'query_qos', 'query_traffic'];
      for (const qp of queryPhases) {
        const entry = entries.find(e => e.command === qp);
        if (entry) {
          const bar = '█'.repeat(Math.max(1, Math.round(entry.durationMs / 50)));
          addTerminalLine(`  [模组查询] ${qp.padEnd(30)} ${String(entry.durationMs).padStart(5)}ms ${bar}`, 'info');
        }
      }

      // Tauri command level
      const tauriEntries = entries.filter(e => e.phase === 'tauri_cmd');
      if (tauriEntries.length > 0) {
        addTerminalLine(`[Tauri命令] 共 ${tauriEntries.length} 条:`, 'info');
        for (const e of tauriEntries) {
          const bar = '█'.repeat(Math.max(1, Math.round(e.durationMs / 50)));
          addTerminalLine(`  ${e.command.padEnd(35)} ${String(e.durationMs).padStart(5)}ms ${bar}`, 'info');
        }
      }

      // Auto-connect
      const autoConn = entries.find(e => e.command === 'auto_connect_at');
      if (autoConn) {
        addTerminalLine(`[自动连接] ${String(autoConn.durationMs).padStart(5)}ms`, 'info');
      }

      addTerminalLine('───────────────────────────────────────', 'info');
      addTerminalLine(`[时延汇总] AT指令总: ${stats.totalMs || 0}ms | 前端总耗时: ${totalMs}ms`, 'info');
      addTerminalLine('═══════════════════════════════════════', 'info');
    }

    // ── 频段初始化（空，等待连接后从模组加载） ──
    renderBandGrid([], [], 'bandGridLte');
    renderBandGrid([], [], 'bandGridNr');

    // ── 关于对话框 ──
    function showAbout() {
      document.getElementById('aboutOverlay').style.display = 'flex';
    }
    function hideAbout() {
      document.getElementById('aboutOverlay').style.display = 'none';
    }
    {
      const listen = window.__TAURI__?.event?.listen;
      if (listen) {
        listen('show-about', showAbout);
      }
    }
