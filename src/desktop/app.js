
    // ── DOM cache (one-time lookup, avoids redundant getElementById) ──
    const $ = {};
    function cacheDom() {
      $.statusLabel = document.getElementById('statusLabel');
      $.connectBtn = document.getElementById('connectBtn');
      $.dataStatusLabel = document.getElementById('dataStatusLabel');
      $.dataDot = document.getElementById('dataDot');
      $.dataConnectBtn = document.getElementById('dataConnectBtn');
      $.terminal = document.getElementById('terminal');
      $.atCommand = document.getElementById('atCommand');
      $.toast = document.getElementById('toast');
      $.toastText = document.getElementById('toastText');
      $.loadingOverlay = document.getElementById('loadingOverlay');
      $.loadingText = document.getElementById('loadingText');
      $.loadingSub = document.getElementById('loadingSub');
      $.connectionParams = document.getElementById('connectionParams');
      $.connectionAuthRow = document.getElementById('connectionAuthRow');
      $.connectionUsername = document.getElementById('connectionUsername');
      $.connectionPassword = document.getElementById('connectionPassword');
      $.appVersion = document.getElementById('appVersion');
      $.aboutVersion = document.getElementById('aboutVersion');
    }

    // ── Data Connection ──
    async function toggleDataConnection() {
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
        $.dataConnectBtn.textContent = '连接中...';
        $.dataConnectBtn.classList.add('connecting');
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
      if (state.dataConnected) {
        $.dataStatusLabel.textContent = state.dataApn || '已连接';
        $.dataStatusLabel.classList.add('active');
        $.dataDot.classList.add('data-on');
        $.dataConnectBtn.textContent = '断开';
        $.dataConnectBtn.classList.add('active');
        $.dataConnectBtn.classList.remove('connecting');
      } else {
        $.dataStatusLabel.textContent = '未连接';
        $.dataStatusLabel.classList.remove('active');
        $.dataDot.classList.remove('data-on');
        $.dataConnectBtn.textContent = '连接';
        $.dataConnectBtn.classList.remove('active', 'connecting');
      }
    }


    // ── UI Scale ──
    function applyUiScale(scale) {
      const numericScale = parseFloat(scale) || 1.0;
      // 直接应用缩放至根元素 html，消除物理边距空白和百分比对齐舍入问题
      document.documentElement.style.zoom = numericScale;

      // 清除 body 和 app-wrap 的行内样式，让其完全依靠 CSS 的 height: 100% 充满视口
      document.body.style.zoom = '';
      const appWrap = document.getElementById('appWrap');
      if (appWrap) {
        appWrap.style.zoom = '';
        appWrap.style.width = '';
        appWrap.style.height = '';
      }
    }

    function autoScaleUI() {
      const isAuto = localStorage.getItem('ui-scale-auto') !== 'false';
      if (!isAuto) return;

      const targetWidth = 1200;
      const targetHeight = 940;
      const w = window.innerWidth;
      const h = window.innerHeight;

      const scaleX = w / targetWidth;
      const scaleY = h / targetHeight;
      let scale = Math.min(scaleX, scaleY);

      // 将上限放宽到 3.0，允许 2K / 4K 超大分辨率全屏时，界面能被等比放大以填满视口，彻底消除侧边和底部的空白
      scale = Math.max(0.6, Math.min(3.0, scale));
      applyUiScale(scale);
    }

    (function () {
      const isAuto = localStorage.getItem('ui-scale-auto') !== 'false';
      if (isAuto) {
        autoScaleUI();
      } else {
        const savedScale = localStorage.getItem('ui-scale');
        if (savedScale) {
          applyUiScale(savedScale);
        }
      }
    })();

    window.addEventListener('resize', () => {
      requestAnimationFrame(autoScaleUI);
    });

    function setUiScaleMode(mode) {
      const isAuto = mode === 'auto';
      localStorage.setItem('ui-scale-auto', isAuto ? 'true' : 'false');
      updateUiScaleModeUI(mode);

      if (isAuto) {
        const manualGroup = document.getElementById('manualScaleRow');
        if (manualGroup) manualGroup.style.opacity = '0.4';
        autoScaleUI();
      } else {
        const manualGroup = document.getElementById('manualScaleRow');
        if (manualGroup) manualGroup.style.opacity = '1.0';
        const savedScale = parseFloat(localStorage.getItem('ui-scale')) || 1.0;
        setUiScale(savedScale);
      }
    }

    function updateUiScaleModeUI(mode) {
      const autoBtn = document.getElementById('scaleModeAuto');
      const manualBtn = document.getElementById('scaleModeManual');
      if (autoBtn) autoBtn.classList.toggle('active', mode === 'auto');
      if (manualBtn) manualBtn.classList.toggle('active', mode === 'manual');
    }

    function setUiScale(scale) {
      applyUiScale(scale);
      localStorage.setItem('ui-scale', scale);
      updateUiScaleToggle(scale);
    }

    function updateUiScaleToggle(scale) {
      const scales = [0.8, 0.9, 1.0, 1.1, 1.2];
      scales.forEach(s => {
        const btn = document.getElementById('scale' + Math.round(s * 100));
        if (btn) btn.classList.toggle('active', Math.abs(s - scale) < 0.01);
      });
    }

    async function setMqttEnabled(enabled) {
      try {
        await invoke('set_mqtt_enabled', { enabled });
        const liveEnabled = await invoke('get_mqtt_enabled');
        updateMqttUI(liveEnabled);
      } catch (e) {
        console.error('Failed to set MQTT enabled:', e);
        updateMqttUI(false);
        showToast('设置失败: ' + (e.message || String(e)), 'err');
      }
    }
    window.setMqttEnabled = setMqttEnabled;

    function updateMqttUI(enabled) {
      const onBtn = document.getElementById('mqttEnabledOn');
      const offBtn = document.getElementById('mqttEnabledOff');
      if (onBtn) onBtn.classList.toggle('active', enabled);
      if (offBtn) offBtn.classList.toggle('active', !enabled);
    }

    async function initMqttSetting() {
      try {
        const enabled = await invoke('get_mqtt_enabled');
        updateMqttUI(enabled);
      } catch (e) {
        console.error('Failed to read MQTT state on startup:', e);
        updateMqttUI(false);
      }
    }

    // ── Loading overlay ──
    function showLoading(text, sub) {
      $.loadingText.textContent = text || '正在加载...';
      $.loadingSub.textContent = sub || '';
      $.loadingOverlay.style.display = 'flex';
      requestAnimationFrame(() => {});
    }
    function setLoadingText(text, sub) {
      $.loadingText.textContent = text || '正在加载...';
      $.loadingSub.textContent = sub || '';
    }
    function hideLoading() {
      $.loadingOverlay.style.display = 'none';
    }

    // ── Utility ──
    function escapeHtml(s) {
      return String(s ?? '').replace(/&/g,'&amp;').replace(/</g,'&lt;')
                            .replace(/>/g,'&gt;').replace(/"/g,'&quot;');
    }
    function isDigits(s, min, max) {
      return new RegExp(`^\\d{${min},${max}}$`).test(s);
    }

    // ── Toast notification ──
    let toastTimer = null;
    function showToast(text, type) {
      $.toastText.textContent = text;
      $.toast.style.background = type === 'ok' ? 'var(--success)' : 'var(--error)';
      $.toast.style.color = '#fff';
      $.toast.style.display = 'block';
      $.toast.style.opacity = '1';
      if (toastTimer) clearTimeout(toastTimer);
      toastTimer = setTimeout(() => {
        $.toast.style.opacity = '0';
        setTimeout(() => { $.toast.style.display = 'none'; }, 200);
      }, type === 'err' ? 4000 : 2000);
    }

    // Event delegation: single listener on the nav parent instead of per-item
    document.querySelector('.nav').addEventListener('click', (e) => {
      const item = e.target.closest('.nav-item:not(.disabled)');
      if (!item) return;
      const prevPage = document.querySelector('.page.active')?.id?.replace('page-', '') || '';
      document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
      item.classList.add('active');
      document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
      document.getElementById('page-' + item.dataset.page).classList.add('active');
      if (window.handleDebugTerminalPageChange) {
        window.handleDebugTerminalPageChange(prevPage, item.dataset.page);
      }
      if (item.dataset.page === 'status' && state.connected) {
        const activeTabBtn = document.querySelector('#page-status .tab-btn.active');
        if (activeTabBtn) activeTabBtn.click();
      }
      if (item.dataset.page === 'monitor') {
        initMonitorPage();
      }
      if (item.dataset.page === 'cellular' && state.connected) {
        const activeTabBtn = document.querySelector('#page-cellular .tab-btn.active');
        if (activeTabBtn) activeTabBtn.click();
      }
      if (item.dataset.page === 'hardware' && state.connected) {
        loadHardwarePage();
      }
      if (item.dataset.page === 'ip' && state.connected) {
        refreshLanConfig();
      }
      if (item.dataset.page === 'scene') {
        loadScenePage();
      }
      if (item.dataset.page === 'atmanual') {
        initAtdbPage();
      }
      if (item.dataset.page === 'adbdebug' && window.initDebugTerminal) {
        window.initDebugTerminal().catch(() => {});
      }
      if (item.dataset.page === 'sshdebug' && window.initDebugTerminal) {
        window.initDebugTerminal().catch(() => {});
      }
    });

    async function loadHardwarePage() {
      showLoading('正在加载模组配置...', '查询模组信息与配置');
      try {
        await refreshHardwareInfo();
      } catch (e) {
        addTerminalLine('[系统] 加载失败: ' + e, 'err');
      }
      hideLoading();
    }

    // ── Connection ──
    async function toggleConnection(isAuto = false) {
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
        // 断开串口：加超时保护。如果后端 transport.close() 卡住（fd 坏死 /
        // WebSocket 远端无响应），3 秒后前端强制标记断开并更新 UI。
        // 后端已 force_shutdown (非-blocking)，正常情况下 <500ms 返回。
        try {
          await Promise.race([
            invoke('disconnect'),
            new Promise((_, rj) => setTimeout(() => rj(new Error('断开超时(3s)，已强制标记断开')), 3000)),
          ]);
        } catch (e) {
          addTerminalLine('[连接] ' + e, 'warn');
        }
        // ALWAYS update local state regardless of whether the IPC completed,
        // so the UI port is never stuck in "连接中" / disabled state.
        state.connected = false;
        state.dataConnected = false;
        state.dataApn = '';
        state.connectedPort = '';
        state.idle = false;
        state.model = '';
        state.chipVendor = '';
        state.currentBand = '';
        state.transport = undefined; // 阻止后续 IPC 复用死 transport
        updateConnectionUI(false);
        updateDataConnectionUI();
        clearData();
        updateVlanPanelAccess();
        updateCellLockUI();
        $.connectBtn.disabled = false;
        $.connectBtn.textContent = '连接';
        $.connectBtn.className = 'btn btn-primary';
        addTerminalLine('[连接] 已断开', 'cmd');
      } else {
        // 连接
        const selectedPort = isAuto ? "" : $.connectionParams.value;
        const connType = document.getElementById('connectionType')?.value || 'serial';

        $.connectBtn.textContent = '连接中...';
        $.connectBtn.disabled = true;

        if (connType === 'serial' && selectedPort) {
          $.statusLabel.textContent = `正在连接 ${selectedPort}...`;
          showLoading(`正在连接 ${selectedPort}...`, '打开串口');
          addTerminalLine(`[连接] 正在连接串口 ${selectedPort}...`, 'info');
          try {
            await invoke('connect_serial', { portName: selectedPort, baudRate: 115200 });
            state.connected = true;
            state.idle = false;
            state.connectedPort = selectedPort;
            updateConnectionUI(true);
            addTerminalLine(`[连接] 已连接到 ${selectedPort}`, 'ok');
            $.statusLabel.textContent = selectedPort;
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
            console.error('[连接] 连接失败:', errMsg);
            addTerminalLine('[连接] ' + errMsg, 'err');
            state.connected = false;
            state.idle = true;
            updateConnectionUI(false);
            $.statusLabel.textContent = '待机中';
            $.statusLabel.style.color = 'var(--text-muted)';
          }
        } else if (connType === 'ethernet' && selectedPort) {
          const username = $.connectionUsername?.value?.trim() || null;
          const password = $.connectionPassword?.value?.trim() || null;
          $.statusLabel.textContent = `正在连接 ${selectedPort}...`;
          showLoading(`正在连接 ${selectedPort}...`, '建立 WebSocket 连接');
          addTerminalLine(`[连接] 正在通过网关 ${selectedPort}:8888 连接模组...`, 'info');
          try {
            await invoke('connect_websocket', { host: selectedPort, port: 8888, username, password });
            state.connected = true;
            state.idle = false;
            state.connectedPort = selectedPort;
            updateConnectionUI(true);
            addTerminalLine(`[连接] 已成功连接到网关 ${selectedPort}:8888`, 'ok');
            $.statusLabel.textContent = selectedPort;
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
            console.error('[连接] WebSocket连接失败:', errMsg);
            addTerminalLine('[连接] ' + errMsg, 'err');
            state.connected = false;
            state.idle = true;
            updateConnectionUI(false);
            $.statusLabel.textContent = '待机中';
            $.statusLabel.style.color = 'var(--text-muted)';
          }
        } else {
          // 自动连接
          $.statusLabel.textContent = '正在检测AT端口...';
          showLoading('正在连接模组...', '检测 AT 端口');
          addTerminalLine('[连接] 正在检测AT端口...', 'info');
          try {
            const portName = await invoke('auto_connect_at');
            state.connected = true;
            state.idle = false;
            state.connectedPort = portName;
            $.connectionParams.value = portName;
            updateConnectionUI(true);
            addTerminalLine(`[连接] 已连接到 ${portName}`, 'ok');
            $.statusLabel.textContent = portName;
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
            state.connected = false;
            state.idle = true;
            updateConnectionUI(false);
            $.statusLabel.textContent = '待机中';
            $.statusLabel.style.color = 'var(--text-muted)';
          }
        }
      }
    }

    function updateConnectionUI(connected) {
      const connType = document.getElementById('connectionType');
      if (connType) connType.disabled = false;
      $.connectionParams.disabled = connected;
      if ($.connectionUsername) $.connectionUsername.disabled = connected;
      if ($.connectionPassword) $.connectionPassword.disabled = connected;
      const icon = document.getElementById('statusDot');
      if (connected) {
        $.connectBtn.textContent = '断开';
        $.connectBtn.disabled = false;
        $.connectBtn.className = 'btn btn-danger';
        icon.classList.add('connected');
        $.statusLabel.style.color = '';
        $.statusLabel.textContent = $.connectionParams.value.trim() || '已连接';
        syncRfState().catch(() => {});
      } else {
        $.connectBtn.textContent = '连接';
        $.connectBtn.disabled = false;
        $.connectBtn.className = 'btn btn-primary';
        icon.classList.remove('connected');
        if (state.idle) {
          $.statusLabel.textContent = '待机中';
          $.statusLabel.style.color = 'var(--text-muted)';
        } else {
          $.statusLabel.textContent = '未连接';
          $.statusLabel.style.color = '';
        }
        resetHardwareTabs();
      }
    }

    // ── 数据刷新 via AT adapter layer ──
    async function refreshAll() {
      if (!state.connected) return;
      showLoading('正在获取模组数据...', '查询网络状态');
      addTerminalLine('[刷新] 开始获取模组数据...', 'info');
      try {
        await refreshModemStatus(false);
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

    async function refreshModemStatus(standalone = true) {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      if (standalone) showLoading('正在获取网络状态...', '请稍候');
      addTerminalLine('[状态] 正在查询模组状态...', 'info');
      try {
        const s = await invoke('get_modem_status');
        console.log('[modem-status]', JSON.stringify(s));
        addTerminalLine(`[状态] SIM=${s.simStatus} 注册=${s.regStatus} 连接=${s.connStatus} 网络=${s.networkType} 运营商=${s.operator}`, 'info');
        if (s.simStatus === 'READY') setTextGood('simStatus', '已插入'); else if (s.simStatus === 'NO SIM') setTextWarn('simStatus', 'NO SIM'); else setText('simStatus', s.simStatus || '--');
        querySimSlot(s.simStatus === 'READY');
        const noSim = s.simStatus !== 'READY';
        const regMap = { 'NOCONN': '已注册', 'CONNECT': '已注册', 'IDLE': '空闲', 'LIMSRV': '限制服务', 'SEARCH': '搜网', 'DENIED': '拒绝注册', 'UNKNOWN': '未知' };
        const regText = regMap[s.regStatus] || s.regStatus || '--';
        const regEl = document.getElementById('regStatus');
        const regGood = s.regStatus === 'CONNECT' || s.regStatus === 'NOCONN' || s.regStatus === 'IDLE';
        if (regEl) { regEl.textContent = regText; regEl.className = regGood ? 'info-value good' : 'info-value'; }
        const connEl = document.getElementById('connStatus');
        if (connEl) { connEl.textContent = s.connStatus || '--'; connEl.className = s.connStatus === '已连接' ? 'info-value good' : 'info-value'; }
        setTextData('imei', s.imei || '--');
        setTextData('iccid', noSim ? '--' : (s.iccid || '--'));
        setTextData('operator', noSim ? '--' : (s.operator || '--'));
        setTextData('networkType', noSim ? '--' : (s.networkType || '--'));
        setTextData('pci', noSim ? '--' : (s.pci || '--'));
        setTextData('cellid', noSim ? '--' : (s.cellId || '--'));
        setTextData('arfcn', noSim ? '--' : (s.arfcn || '--'));
        setTextData('band', noSim ? '--' : (s.band || '--'));
        setTextData('bandwidth', noSim ? '--' : (s.bandwidth || '--'));
        setTextData('rsrp', noSim ? '--' : formatRsrpFrontend(s.rsrp));
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

        // Track chip vendor and current band for cell lock / VLAN UI
        state.chipVendor = s.chipVendor || '';
        state.currentBand = s.band || '';
        updateVlanPanelAccess();
        updateCellLockUI();
        // VLAN 面板此前从不向设备查询(refreshVlan 是死代码)，恒显示缓存/默认值。
        // 已知 chipVendor 后从设备读取真实 VLAN 配置；refreshVlan 内部自带 qualcomm/已连接守卫。
        try { await refreshVlan(); } catch (_) {}

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
        // 状态页"刷新"按钮(standalone)此前只调 get_modem_status，CQI/上下行带宽/上下行流量
        // 会停留在连接时的缓存值；这里补刷，使页面数据全部来自实时 AT 查询。
        // (refreshAll 路径 standalone=false，其自身已分别刷新 QoS/流量，避免重复。)
        if (standalone) {
          try { await refreshQos(); } catch (_) {}
          try { await refreshTraffic(); } catch (_) {}
          showToast('刷新成功', 'ok');
        }

      } catch (e) {
        console.error('Failed to refresh modem status:', e);
        addTerminalLine('[状态] 刷新失败：' + e, 'err');
        showToast('刷新失败：' + e, 'err');
      } finally {
        if (standalone) hideLoading();
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
      const sel = document.getElementById('simSlotSwitch');
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
        'operator','networkType','band','pci','cellid','arfcn','bandwidth','rsrp','rsrq','sinr','txPower','rxLevel','cqi','scs',
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
      const cmd = $.atCommand.value.trim();
      if (!cmd) return;

      state.atHistory.unshift(cmd);
      if (state.atHistory.length > 100) state.atHistory.pop();
      state.atHistoryIdx = -1;
      $.atCommand.value = '';

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
      $.atCommand.value = cmd;
      sendAtCommand();
    }

    function handleAtKey(e) {
      if (e.key === 'Enter') { sendAtCommand(); return; }
      if (e.key === 'ArrowUp') {
        e.preventDefault();
        if (state.atHistoryIdx < state.atHistory.length - 1) {
          state.atHistoryIdx++;
          $.atCommand.value = state.atHistory[state.atHistoryIdx];
        }
      }
      if (e.key === 'ArrowDown') {
        e.preventDefault();
        if (state.atHistoryIdx > 0) {
          state.atHistoryIdx--;
          $.atCommand.value = state.atHistory[state.atHistoryIdx];
        } else {
          state.atHistoryIdx = -1;
          $.atCommand.value = '';
        }
      }
    }

    function addTerminalLine(text, cls) {
      const line = document.createElement('div');
      line.className = 'terminal-line' + (cls ? ' ' + cls : '');
      line.textContent = text;
      $.terminal.appendChild(line);
      while ($.terminal.children.length > 500) $.terminal.removeChild($.terminal.firstChild);
      $.terminal.scrollTop = $.terminal.scrollHeight;
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
      $.terminal.innerHTML = '';
    }

    // ── 蜂窝网络 Tab 切换 ──
    function switchCellularTab(tab, btn) {
      document.querySelectorAll('#page-cellular .tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('#page-cellular > .panel > .tab-panel').forEach(p => p.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById('ctab-' + tab).classList.add('active');
      // Load neighbor data every time tab is switched
      if (tab === 'neighbor' && state.connected) {
        loadNeighborCells();
      }
      // Load bands + network mode when switching to netlock tab
      if (tab === 'netlock' && state.connected) {
        loadNetlockData();
      }
      if (tab === 'apn' && state.connected) {
        refreshApnList();
      }
      if (tab === '5glan') {
        const cv = state.chipVendor;
        const targetTab = cv === 'qualcomm' ? 'qualcomm' : 'unisoc';
        const targetBtn = document.getElementById(
          targetTab === 'qualcomm' ? 'glanTabQualcomm' : 'glanTabUnisoc'
        );
        if (targetBtn) switch5GlanTab(targetTab, targetBtn);
        else if (state.connected) refresh5Glan();
      }
      // Query current cell lock when switching to celllock tab
      if (tab === 'celllock' && state.connected) {
        updateCellLockUI();
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
            <div class="apn-item-name">${escapeHtml(a.name)}</div>
            <div class="apn-item-meta">${escapeHtml(a.ip.toUpperCase())} · 鉴权: ${escapeHtml(a.auth.toUpperCase())}${a.user ? ' · 用户: ' + escapeHtml(a.user) : ''}</div>
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

    function switch5GlanTab(tab, btn) {
      const bar = document.getElementById('glanSubBar');
      bar.querySelectorAll('.sub-tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('#ctab-5glan > .tab-panel').forEach(p => p.classList.remove('active'));
      btn.classList.add('active');
      document.getElementById('glan-' + tab).classList.add('active');
      requestAnimationFrame(() => {
        const indicator = bar.querySelector('.tab-indicator');
        if (indicator) moveTabIndicator(indicator, btn);
      });
      if (tab === 'unisoc' && state.connected) refresh5Glan();
      if (tab === 'qualcomm' && state.connected) refresh5GlanQualcommStatus();
    }

    function render5Glan() {
      const container = document.getElementById('glanToggles');
      if (!glanData.length) {
        container.innerHTML = '<div style="color:var(--text-muted);font-size:12px;padding:8px 0;">暂无 5GLAN 配置，选择 CID 并保存</div>';
        return;
      }
      container.innerHTML =
        '<div style="font-size:11px;font-weight:700;color:var(--text-secondary);margin-bottom:6px;text-transform:uppercase;letter-spacing:0.6px;">当前配置</div>' +
        glanData.map(g => `
          <div style="display:flex;align-items:center;padding:6px 0;border-bottom:1px solid var(--border-color);gap:12px;cursor:pointer;"
               onclick="selectGlanEntry(${g.cid},${g.vlanId ?? 1},${g.enabled})">
            <div style="font-size:13px;font-weight:600;min-width:52px;">CID ${g.cid}</div>
            <div style="flex:1;color:var(--text-muted);font-size:12px;">VLAN ${g.vlanId ?? 1}</div>
            <span style="font-size:12px;font-weight:600;${g.enabled ? 'color:var(--ok,#10b981)' : 'color:var(--text-muted)'};">${g.enabled ? '已激活' : '已关闭'}</span>
          </div>
        `).join('');
    }

    function selectGlanEntry(cid, vlanId, enabled) {
      document.getElementById('glanUsCid').value = cid;
      document.getElementById('glanUsVlan').value = vlanId;
      document.getElementById('glanUsEnabled').checked = enabled;
    }

    function onGlanUsCidChange() {
      const cid = parseInt(document.getElementById('glanUsCid').value, 10);
      const existing = glanData.find(g => g.cid === cid);
      if (existing) {
        document.getElementById('glanUsVlan').value = existing.vlanId ?? 1;
        document.getElementById('glanUsEnabled').checked = existing.enabled;
      } else {
        document.getElementById('glanUsVlan').value = 1;
        document.getElementById('glanUsEnabled').checked = false;
      }
    }

    async function saveUnisoc5Glan() {
      if (!state.connected) { showToast('未连接模组', 'err'); return; }
      const cid     = parseInt(document.getElementById('glanUsCid').value, 10);
      const vlanId  = parseInt(document.getElementById('glanUsVlan').value, 10);
      const enabled = document.getElementById('glanUsEnabled').checked;
      if (!Number.isFinite(vlanId) || vlanId < 1 || vlanId > 255) {
        showToast('请输入有效 VLAN ID (1-255)', 'err'); return;
      }
      try {
        await invoke('set_5glan', { cid, enabled, vlanId });
        await flushAtLog();
        showToast(`CID${cid} 5GLAN ${enabled ? '已激活' : '已关闭'}`, 'ok');
        addTerminalLine(`[5GLAN] CID${cid} VLAN${vlanId} → ${enabled ? '开启' : '关闭'}`, 'ok');
        await refresh5Glan();
      } catch (e) {
        showToast('5GLAN 设置失败: ' + e, 'err');
        addTerminalLine('[5GLAN] 设置失败: ' + e, 'err');
      }
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

    function onGlanQcVlanToggle(checked) {
      const el = document.getElementById('glanQcVlanInputs');
      el.style.display = checked ? 'inline-flex' : 'none';
    }

    async function configureQualcomm5Glan() {
      if (!state.connected) { showToast('未连接模组', 'err'); return; }
      const cid = parseInt(document.getElementById('glanQcCid').value, 10);
      const apn = document.getElementById('glanQcApn').value.trim();
      const snssai = document.getElementById('glanQcSnssai').value.trim();
      const profileId = parseInt(document.getElementById('glanQcProfile').value, 10);
      const useVlan = document.getElementById('glanQcUseVlan').checked;
      const vlanStart = useVlan ? parseInt(document.getElementById('glanQcVlanStart').value, 10) : 65535;
      const vlanEnd   = useVlan ? parseInt(document.getElementById('glanQcVlanEnd').value, 10)   : 65535;
      if (!Number.isFinite(cid) || cid < 1 || cid > 15) { showToast('请输入有效 CID', 'err'); return; }
      if (!apn) { showToast('请输入 APN', 'err'); return; }
      if (!snssai) { showToast('请输入 S-NSSAI', 'err'); return; }
      try {
        await invoke('configure_qualcomm_5glan', { cid, apn, snssai, profileId, vlanStart, vlanEnd });
        await flushAtLog();
        showToast('参数配置成功', 'ok');
        addTerminalLine(`[5GLAN-QC] 配置参数 CID=${cid} APN=${apn} SNSSAI=${snssai}${useVlan ? ` VLAN=${vlanStart}-${vlanEnd}` : ''}`, 'ok');
      } catch (e) {
        showToast('配置失败: ' + e, 'err');
        addTerminalLine('[5GLAN-QC] 配置失败: ' + e, 'err');
      }
    }

    async function enableEthPdu() {
      if (!state.connected) { showToast('未连接模组', 'err'); return; }
      try {
        await invoke('enable_eth_pdu');
        await flushAtLog();
        showToast('ETH PDU 已启用，请重启模组后执行步骤 3', 'ok');
        addTerminalLine('[5GLAN-QC] ETH PDU 启用成功，等待模组重启', 'ok');
        await refresh5GlanQualcommStatus();
      } catch (e) {
        showToast('ETH PDU 启用失败: ' + e, 'err');
        addTerminalLine('[5GLAN-QC] ETH PDU 启用失败: ' + e, 'err');
      }
    }

    async function connectQualcomm5Glan() {
      if (!state.connected) { showToast('未连接模组', 'err'); return; }
      const ruleId = parseInt(document.getElementById('glanQcRuleId').value, 10);
      const cid    = parseInt(document.getElementById('glanQcCid').value, 10);
      if (!Number.isFinite(ruleId) || ruleId < 0 || ruleId > 7) { showToast('请输入有效 Rule ID (0-7)', 'err'); return; }
      if (!Number.isFinite(cid) || cid < 1 || cid > 15) { showToast('请输入有效 CID', 'err'); return; }
      try {
        await invoke('connect_qualcomm_5glan', { ruleId, cid });
        await flushAtLog();
        showToast('连接成功', 'ok');
        addTerminalLine(`[5GLAN-QC] Rule=${ruleId} CID=${cid} 连接成功`, 'ok');
        await refresh5GlanQualcommStatus();
      } catch (e) {
        showToast('连接失败: ' + e, 'err');
        addTerminalLine('[5GLAN-QC] 连接失败: ' + e, 'err');
      }
    }

    async function refresh5GlanQualcommStatus() {
      const el = document.getElementById('glanQcStatus');
      if (!el) return;
      if (!state.connected) { el.textContent = '—'; return; }
      try {
        const s = await invoke('query_qualcomm_5glan_status');
        const ethState = s.ethPduEnabled ? '<span style="color:var(--ok,#10b981)">已启用</span>' : '<span style="color:var(--text-muted)">未启用</span>';
        const connState = s.connected ? '<span style="color:var(--ok,#10b981)">已连接</span>' : '<span style="color:var(--text-muted)">未连接</span>';
        const cidStr = s.mpdnCid != null ? `CID ${s.mpdnCid}` : '—';
        el.innerHTML = `ETH PDU: ${ethState} &nbsp;|&nbsp; MPDN CID: ${cidStr} &nbsp;|&nbsp; 连接状态: ${connState}`;
      } catch (e) {
        el.textContent = '查询失败: ' + e;
      }
    }

    // ── 网络锁定 ──

    function getChipFamily(model) {
      if (!model) return 'unisoc';
      const m = model.toUpperCase();
      if (m.includes('RG520') || m.includes('RM520') || m.includes('RG525') ||
          m.includes('RG530') || m.includes('RM530') ||
          m.includes('RG500Q') || m.includes('RM500Q') || m.includes('RM501Q') || m.includes('RM551'))
        return 'qualcomm';
      if (m.includes('RG255')) return 'asr';
      return 'unisoc';
    }

    function buildNetworkModeOptions(sel) {
      const zh = state.lang === 'zh';
      const family = getChipFamily(state.model);
      // Qualcomm: AUTO WCDMA & LTE & 5G | NR5G 5G only | LTE LTE only
      const qualcommOpts = [
        { value: 'AUTO',     zh: '自动',       en: 'Auto' },
        { value: 'NR5G',     zh: '仅 5G',      en: '5G Only' },
        { value: 'LTE',      zh: '仅 LTE',     en: 'LTE Only' },
      ];
      // UniSoc: AUTO | NR5G SA+NSA | NR5G-SA SA only | NR5G-NSA NSA only | LTE
      const unisocOpts = [
        { value: 'AUTO',     zh: '自动',       en: 'Auto' },
        { value: 'NR5G',     zh: '5G (SA+NSA)', en: '5G (SA+NSA)' },
        { value: 'NR5G-SA',  zh: '仅 5G SA',   en: '5G SA Only' },
        { value: 'NR5G-NSA', zh: '仅 5G NSA',  en: '5G NSA Only' },
        { value: 'LTE',      zh: '仅 LTE',     en: 'LTE Only' },
      ];
      const opts = family === 'qualcomm' ? qualcommOpts : unisocOpts;
      sel.innerHTML = opts.map(o => `<option value="${o.value}">${zh ? o.zh : o.en}</option>`).join('');
    }

    async function loadNetlockData() {
      if (!state.connected) return;
      showLoading('正在读取网络配置...');
      // Load preferred network mode
      try {
        const sel = document.getElementById('preferredNetwork');
        buildNetworkModeOptions(sel);
        const mode = await invoke('get_network_mode');
        sel.value = mode;
      } catch (e) {
        console.warn('get_network_mode failed:', e);
      }
      // Load IMS state
      try {
        const isQualcomm = state.chipVendor === 'qualcomm';
        const resp = await invoke('send_raw_at', { command: 'AT+QCFG="ims"' });
        let imsVal = 0;
        if (isQualcomm) {
          // Qualcomm: +QCFG: "ims",<mode>,<enable>  — second param is the toggle
          const m = resp.match(/\+QCFG:\s*"ims",\d,\s*(\d)/i);
          if (m) imsVal = parseInt(m[1]);
        } else {
          // UniSoc: +QCFG: "ims",<enable>  — single value
          const m = resp.match(/\+QCFG:\s*"ims",(\d)/i);
          if (m) imsVal = parseInt(m[1]);
        }
        updateImsToggle(imsVal);
      } catch (e) {
        console.warn('get IMS failed:', e);
      }
      // Load bands independently
      await refreshBands();
      hideLoading();
    }

    function updateImsToggle(val) {
      document.getElementById('imsBtnOff').classList.toggle('active', val === 0);
      document.getElementById('imsBtnOn').classList.toggle('active', val === 1);
    }

    async function setIms(val) {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      try {
        showLoading('正在设置 IMS...');
        const isQualcomm = state.chipVendor === 'qualcomm';
        // Qualcomm: AT+QCFG="ims",<ims_conf>,<volte_cap>
        //   Enable:  AT+QCFG="ims",1,1
        //   Disable: AT+QCFG="ims",2,0
        // UniSoc:  AT+QCFG="ims",<enable>
        const cmd = isQualcomm
          ? `AT+QCFG="ims",${val === 1 ? '1,1' : '2,0'}`
          : `AT+QCFG="ims",${val}`;
        await invoke('send_raw_at', { command: cmd });
        await flushAtLog();
        hideLoading();
        updateImsToggle(val);
        showToast(`IMS 已${val ? '开启' : '关闭'}`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('IMS 设置失败: ' + e, 'err');
      }
    }

    async function applyPreferredNetwork() {
      const sel = document.getElementById('preferredNetwork');
      try {
        showLoading('正在保存首选网络...');
        await invoke('set_network_mode_cmd', { mode: sel.value });
        await flushAtLog();
        hideLoading();
        showToast('首选网络保存成功', 'ok');
        addTerminalLine(`[网络] 首选网络: ${sel.selectedOptions[0].text}`, 'ok');
        try {
          const mode = await invoke('get_network_mode');
          addTerminalLine(`[网络] 回读原始值: ${mode}`, 'info');
          sel.value = mode;
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
        // Use spec bands (static per model) as the base grid, fall back to supported if spec empty
        const lteBase = (cfg.lteSpec && cfg.lteSpec.length > 0) ? cfg.lteSpec : cfg.lteSupported;
        const nrBase = (cfg.nrSpec && cfg.nrSpec.length > 0) ? cfg.nrSpec : cfg.nrSupported;
        renderBandGrid(lteBase, cfg.lteLocked, 'bandGridLte', cfg.lteInvalid);
        renderBandGrid(nrBase, cfg.nrLocked, 'bandGridNr', cfg.nrInvalid);
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
      const password = document.getElementById('lockPassword').value.trim();
      if (!mcc || !mnc) { showToast('请输入 MCC 和 MNC', 'err'); return; }
      if (!isDigits(mcc, 3, 3)) { showToast('MCC 必须为 3 位数字', 'err'); return; }
      if (!isDigits(mnc, 2, 3)) { showToast('MNC 必须为 2-3 位数字', 'err'); return; }
      if (!password) { showToast('请输入供应商提供的锁定密码', 'err'); return; }
      const plmn = mcc + mnc;
      try {
        showLoading('正在锁定PLMN...');
        await invoke('set_plmn_lock', { plmn, password });
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
      const password = document.getElementById('lockPassword').value.trim();
      if (!password) { showToast('请输入供应商提供的锁定密码', 'err'); return; }
      try {
        showLoading('正在解锁PLMN...');
        await invoke('clear_plmn_lock', { password });
        await flushAtLog();
        hideLoading();
        showToast('PLMN 已解锁', 'ok');
        addTerminalLine('[PLMN锁定] 已解锁', 'ok');
        document.getElementById('lockMcc').value = '';
        document.getElementById('lockMnc').value = '';
        document.getElementById('lockPassword').value = '';
      } catch (e) {
        hideLoading();
        showToast('解锁失败: ' + e, 'err');
        addTerminalLine('[PLMN锁定] 解锁失败: ' + e, 'err');
      }
    }

    // ── 小区 / 频点锁定 ──

    async function queryCellLock() {
      if (!state.connected) return;
      showLoading('正在读取锁定配置...');
      const quota = document.getElementById('lockQuota');
      const list = document.getElementById('lockList');
      try {
        const items = await invoke('query_cell_lock');
        await flushAtLog();
        addTerminalLine(`[锁定查询] 共 ${items.length} 条锁定`, 'info');
        if (items.length > 0) {
          quota.textContent = `已锁定 ${items.length} 条`;
          list.innerHTML = items.map(e => `
            <div class="lock-item">
              <span class="lock-item-badge">${e.lockType === 'cell' ? '小区' : '频点'}</span>
              <span class="lock-item-info">频点 ${escapeHtml(e.arfcn)}${e.pci ? '  PCI ' + escapeHtml(e.pci) : ''}</span>
            </div>
          `).join('');
        } else {
          quota.textContent = '当前锁定：无';
          list.innerHTML = '';
        }
      } catch (e) {
        addTerminalLine('[锁定查询] 失败: ' + e, 'err');
      }
      hideLoading();
    }

    async function saveCellLock() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const arfcn = document.getElementById('lockArfcn').value.trim();
      const pci = document.getElementById('lockPci').value.trim();
      const scs = document.getElementById('lockScs')?.value || '';
      const band = document.getElementById('lockBand')?.value.trim() || '';
      if (!arfcn) { showToast('请输入频点', 'err'); return; }
      if (state.chipVendor === 'qualcomm') {
        if (!pci) { showToast('Qualcomm 模组需要填写 PCI', 'err'); return; }
        if (!band) { showToast('Qualcomm 模组需要填写 Band', 'err'); return; }
      }
      try {
        showLoading('正在保存锁定...');
        await invoke('set_cell_lock', { arfcn, pci, scs, band });
        await flushAtLog();
        hideLoading();
        showToast('锁定保存成功', 'ok');
        const extra = state.chipVendor === 'qualcomm' ? ` SCS=${scs} Band=${band}` : '';
        addTerminalLine(`[锁定] 频点=${arfcn}${pci ? ' PCI=' + pci : ''}${extra}`, 'ok');
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
        await invoke('clear_cell_lock');
        await flushAtLog();
        hideLoading();
        showToast('锁定已清除', 'ok');
        addTerminalLine('[锁定] 已清除全部', 'ok');
        document.getElementById('lockArfcn').value = '';
        document.getElementById('lockPci').value = '';
        const bandEl = document.getElementById('lockBand');
        if (bandEl) bandEl.value = state.currentBand || '';
        await queryCellLock();
      } catch (e) {
        hideLoading();
        showToast('清除失败: ' + e, 'err');
        addTerminalLine('[锁定] 清除失败: ' + e, 'err');
      }
    }

    function updateCellLockUI() {
      const isQualcomm = state.chipVendor === 'qualcomm';
      document.getElementById('lockScsGroup').style.display = isQualcomm ? '' : 'none';
      document.getElementById('lockBandGroup').style.display = isQualcomm ? '' : 'none';
      const pciLabel = document.getElementById('lockPciLabel');
      if (pciLabel) {
        pciLabel.setAttribute('data-i18n', isQualcomm ? 'label_pci_required' : 'label_pci_optional');
        pciLabel.textContent = t(isQualcomm ? 'label_pci_required' : 'label_pci_optional');
      }
      const bandEl = document.getElementById('lockBand');
      if (bandEl && isQualcomm && state.currentBand && !bandEl.value) {
        bandEl.value = state.currentBand;
      }
    }

    function updateVlanPanelAccess() {
      const isQualcomm = state.chipVendor === 'qualcomm';
      const hint = document.getElementById('vlanNotSupported');
      if (hint) hint.style.display = isQualcomm ? 'none' : '';
      const cb = document.getElementById('vlanEnabled');
      if (cb) cb.disabled = !isQualcomm;
    }

    function setVlanToggleUI(enabled) {
      const cb  = document.getElementById('vlanEnabled');
      const row = document.getElementById('vlanIdRow');
      cb.checked = enabled;
      row.style.display = enabled ? 'block' : 'none';
    }

    async function refreshVlan() {
      if (!state.connected) return;
      updateVlanPanelAccess();
      if (state.chipVendor !== 'qualcomm') return;
      try {
        const ids = await invoke('get_vlan');
        const hasVlan = ids && ids.length > 0;
        state.vlanEnabled = hasVlan;
        setVlanToggleUI(hasVlan);
        if (hasVlan) document.getElementById('vlanId').value = ids[0];
      } catch (_) { /* leave current state */ }
    }

    async function onVlanToggle(enabled) {
      const row = document.getElementById('vlanIdRow');
      row.style.display = enabled ? 'block' : 'none';
      if (!enabled && state.connected && state.vlanEnabled) {
        const vid = parseInt(document.getElementById('vlanId').value) || 1;
        try {
          await invoke('set_vlan', { vlanId: vid, enabled: false });
          await flushAtLog();
          state.vlanEnabled = false;
          showToast('VLAN 已禁用', 'ok');
        } catch (e) {
          showToast('VLAN 禁用失败: ' + e, 'err');
          setVlanToggleUI(true);
        }
      }
    }

    async function applyVlan() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const id = parseInt(document.getElementById('vlanId').value);
      if (isNaN(id) || id < 1 || id > 255) { showToast('VLAN ID 范围：1 ~ 255', 'err'); return; }
      try {
        showLoading('正在启用 VLAN...');
        await invoke('set_vlan', { vlanId: id, enabled: true });
        await flushAtLog();
        hideLoading();
        state.vlanEnabled = true;
        showToast('VLAN ' + id + ' 已启用', 'ok');
      } catch (e) {
        hideLoading();
        showToast('VLAN 启用失败: ' + e, 'err');
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
        console.log('[neighbor-cells]', JSON.stringify(result));
        populateLteNeighbors(result.lte || []);
        populateNrNeighbors(result.nr || []);
        addTerminalLine(`[邻区] LTE ${(result.lte || []).length} 个, NR ${(result.nr || []).length} 个`, 'ok');
      } catch (e) {
        console.error('[neighbor-cells] failed:', e);
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
        console.log('[neighbor-cells]', JSON.stringify(result));
        populateLteNeighbors(result.lte || []);
        populateNrNeighbors(result.nr || []);
        addTerminalLine(`[邻区] LTE ${(result.lte || []).length} 个, NR ${(result.nr || []).length} 个`, 'ok');
      } catch (e) {
        console.error('[neighbor-cells] failed:', e);
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
        `<tr><td>${escapeHtml(r.pci || '--')}</td><td>${escapeHtml(formatRsrpFrontend(r.rsrp) || '--')}</td><td>${escapeHtml(r.rsrq || '--')}</td><td>${escapeHtml(r.earfcn || '--')}</td></tr>`
      ).join('');
    }

    function populateNrNeighbors(rows) {
      if (!rows || rows.length === 0) {
        document.getElementById('nrNeighborBody').innerHTML = '<tr><td colspan="5" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
        return;
      }
      document.getElementById('nrNeighborBody').innerHTML = rows.map(r =>
        `<tr><td>${escapeHtml(r.pci || '--')}</td><td>${escapeHtml(formatRsrpFrontend(r.rsrp) || '--')}</td><td>${escapeHtml(r.rsrq || '--')}</td><td>${escapeHtml(r.sinr || '--')}</td><td>${escapeHtml(r.arfcn || '--')}</td></tr>`
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

    async function applyMtu() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const val = parseInt(document.getElementById('mtuValue').value, 10);
      if (!val || val < 576 || val > 9000) { showToast('MTU 范围：576 ~ 9000', 'err'); return; }
      try {
        showLoading('正在设置 MTU...');
        await invoke('send_raw_at', { command: `AT+QCFG="mtu",${val}` });
        await flushAtLog();
        hideLoading();
        showToast(`MTU 已设置为 ${val}，重启后生效`, 'ok');
        addTerminalLine(`[MTU] 已设置为 ${val}`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('MTU 设置失败: ' + e, 'err');
      }
    }

    async function applyDmz() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const ip = document.getElementById('dmzHost').value.trim();
      if (!ip) { showToast('请输入 DMZ 主机 IP 地址', 'err'); return; }
      if (!/^(\d{1,3}\.){3}\d{1,3}$/.test(ip)) { showToast('IP 地址格式无效', 'err'); return; }
      if (ip.split('.').some(o => parseInt(o) > 255)) { showToast('IP 地址格式无效', 'err'); return; }
      try {
        showLoading('正在设置 DMZ...');
        const isQualcomm = state.chipVendor === 'qualcomm';
        const cmd = isQualcomm
          ? `AT+QMAP="DMZ",1,4,"${ip}"`
          : `AT+QDMZ=1,4,${ip}`;
        await invoke('send_raw_at', { command: cmd });
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
        const isQualcomm = state.chipVendor === 'qualcomm';
        const cmd = isQualcomm
          ? `AT+QMAP="DMZ",0,4`
          : `AT+QDMZ=0,4`;
        await invoke('send_raw_at', { command: cmd });
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


    async function refreshLanConfig() {
      if (!state.connected) return;
      const loadingEl = document.getElementById('lanLoading');
      const formEl    = document.getElementById('lanForm');
      loadingEl.style.display = '';
      formEl.style.display    = 'none';
      const isQualcomm = state.chipVendor === 'qualcomm';
      try {
        if (isQualcomm) {
          // Qualcomm: AT+QMAP="LANIP" → +QMAP: "LANIP",<start>,<end>,<gw>
          const resp = await invoke('send_raw_at', { command: 'AT+QMAP="LANIP"' });
          for (const line of (resp || '').split('\n')) {
            const m = line.match(/\+QMAP:\s*"LANIP",\s*([^,]+),\s*([^,]+),\s*([^,\s]+)/i);
            if (m) {
              document.getElementById('dhcpStart').value = m[1].trim();
              document.getElementById('dhcpEnd').value   = m[2].trim();
              document.getElementById('lanGw').value     = m[3].trim();
              document.getElementById('lanMask').value   = '';  // QMAP has no mask field
              break;
            }
          }
        } else if (state.chipVendor === 'asr') {
          // ASR: AT+QCFG="lanip" → +QCFG: "lanip","<gw>","<mask>","<start>","<end>",<lease_time>
          const resp = await invoke('send_raw_at', { command: 'AT+QCFG="lanip"' });
          for (const line of (resp || '').split('\n')) {
            const m5 = line.match(/\+QCFG:\s*"lanip",\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?(?:,\s*"?([^",\s]+)"?)?/i);
            if (m5) {
              document.getElementById('lanGw').value     = m5[1];
              document.getElementById('lanMask').value   = m5[2];
              document.getElementById('dhcpStart').value = m5[3];
              document.getElementById('dhcpEnd').value   = m5[4];
              break;
            }
          }
        } else {
          // UniSoc: AT+QCFG="lanip_ex" → +QCFG: "lanip_ex","<gw>","<mask>","<start>","<end>"
          const resp = await invoke('send_raw_at', { command: 'AT+QCFG="lanip_ex"' });
          for (const line of (resp || '').split('\n')) {
            const m4 = line.match(/\+QCFG:\s*"lanip_ex",\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?/i);
            if (m4) {
              document.getElementById('lanGw').value     = m4[1];
              document.getElementById('lanMask').value   = m4[2];
              document.getElementById('dhcpStart').value = m4[3];
              document.getElementById('dhcpEnd').value   = m4[4];
              break;
            }
            const m3 = line.match(/\+QCFG:\s*"lanip_ex",\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?,\s*"?([^",\s]+)"?/i);
            if (m3) {
              document.getElementById('lanGw').value     = m3[1];
              document.getElementById('dhcpStart').value = m3[2];
              document.getElementById('dhcpEnd').value   = m3[3];
              break;
            }
          }
        }
      } catch (e) {
        addTerminalLine('[LAN] 查询失败: ' + e, 'err');
      } finally {
        loadingEl.style.display = 'none';
        formEl.style.display    = '';
        // Bind gateway IP change event to auto-update DHCP range
        const gwInput = document.getElementById('lanGw');
        if (gwInput && !gwInput._dhcpAutoUpdateBound) {
          gwInput.addEventListener('change', updateDhcpRangeFromGateway);
          gwInput._dhcpAutoUpdateBound = true;
        }
      }
    }

    // Helper: Parse IP to array of integers
    function parseIp(ip) {
      const parts = ip.split('.');
      if (parts.length !== 4) return null;
      const nums = parts.map(Number);
      if (nums.some(n => isNaN(n) || n < 0 || n > 255)) return null;
      return nums;
    }

    // Helper: Convert IP array back to string
    function ipToString(parts) {
      return parts.join('.');
    }

    // Auto-update DHCP range when gateway IP changes
    function updateDhcpRangeFromGateway() {
      const gwInput = document.getElementById('lanGw');
      const dhcpStartInput = document.getElementById('dhcpStart');
      const dhcpEndInput = document.getElementById('dhcpEnd');
      
      if (!gwInput || !dhcpStartInput || !dhcpEndInput) return;
      
      const gwParts = parseIp(gwInput.value.trim());
      if (!gwParts) return; // Invalid IP, don't update
      
      // Keep the same last octet pattern for DHCP range
      // If current DHCP start ends with .2, new start should be <gw_prefix>.2
      // If current DHCP end ends with .254, new end should be <gw_prefix>.254
      const currentStartParts = parseIp(dhcpStartInput.value.trim());
      const currentEndParts = parseIp(dhcpEndInput.value.trim());
      
      // Default: start at .2, end at .254
      let startLastOctet = 2;
      let endLastOctet = 254;
      
      if (currentStartParts && currentStartParts[3] !== undefined) {
        startLastOctet = currentStartParts[3];
      }
      if (currentEndParts && currentEndParts[3] !== undefined) {
        endLastOctet = currentEndParts[3];
      }
      
      // Build new DHCP addresses with same subnet as gateway
      const newStartParts = [gwParts[0], gwParts[1], gwParts[2], startLastOctet];
      const newEndParts = [gwParts[0], gwParts[1], gwParts[2], endLastOctet];
      
      dhcpStartInput.value = ipToString(newStartParts);
      dhcpEndInput.value = ipToString(newEndParts);
    }

    async function applyLanConfig() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const gw    = document.getElementById('lanGw').value.trim();
      const mask  = document.getElementById('lanMask').value.trim();
      const start = document.getElementById('dhcpStart').value.trim();
      const end   = document.getElementById('dhcpEnd').value.trim();
      if (!gw || !start || !end) { showToast('请填写完整的 LAN 配置', 'err'); return; }
      const isQualcomm = state.chipVendor === 'qualcomm';
      try {
        showLoading('正在设置 LAN 配置...');
        let cmd;
        if (isQualcomm) {
          cmd = `AT+QMAP="LANIP",${start},${end},${gw}`;
        } else if (state.chipVendor === 'asr') {
          cmd = mask
            ? `AT+QCFG="lanip","${gw}","${mask}","${start}","${end}"`
            : `AT+QCFG="lanip","${gw}","${start}","${end}"`;
        } else {
          cmd = mask
            ? `AT+QCFG="lanip_ex","${gw}","${mask}","${start}","${end}"`
            : `AT+QCFG="lanip_ex","${gw}","${start}","${end}"`;
        }
        await invoke('send_raw_at', { command: cmd });
        await flushAtLog();
        hideLoading();
        showToast('LAN 配置已保存，重启后生效', 'ok');
        addTerminalLine(`[LAN] GW=${gw} MASK=${mask || 'n/a'} DHCP=${start}-${end}`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('LAN 配置设置失败: ' + e, 'err');
        addTerminalLine('[LAN] 设置失败: ' + e, 'err');
      }
    }



    let atdbPlatform = 'unisoc';
    let atdbSelectedCmd = null;
    let atdbSearchText = '';

    function initAtdbPage() {
      renderAtdbIndex();
      const detail = document.getElementById('atdbDetail');
      const welcome = document.getElementById('atdbWelcome');
      if (detail) detail.style.display = 'none';
      if (welcome) welcome.style.display = '';
    }

    function switchAtdbPlatform(platform) {
      atdbPlatform = platform;
      atdbSelectedCmd = null;
      atdbSearchText = '';
      const searchEl = document.getElementById('atdbSearch');
      if (searchEl) searchEl.value = '';
      document.getElementById('atdbBtnUnisoc').classList.toggle('active', platform === 'unisoc');
      document.getElementById('atdbBtnQualcomm').classList.toggle('active', platform === 'qualcomm');
      const asrBtn = document.getElementById('atdbBtnAsr');
      if (asrBtn) asrBtn.classList.toggle('active', platform === 'asr');
      renderAtdbIndex();
      clearAtdbDetail();
    }

    function filterAtdb(text) {
      atdbSearchText = text.toLowerCase();
      renderAtdbIndex();
    }

    function highlightMatch(text, query) {
      if (!query) return escHtml(text);
      const idx = text.toLowerCase().indexOf(query);
      if (idx === -1) return escHtml(text);
      return escHtml(text.slice(0, idx)) + '<mark>' + escHtml(text.slice(idx, idx + query.length)) + '</mark>' + escHtml(text.slice(idx + query.length));
    }

    function escHtml(s) {
      return s.replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
    }

    function renderAtdbIndex() {
      const srcKey = atdbPlatform === 'asr' ? 'unisoc' : atdbPlatform;
      const cmds = AT_DB[srcKey] || [];
      const query = atdbSearchText;
      const categories = {};
      const catOrder = [];
      for (const c of cmds) {
        const matchCmd  = c.cmd.toLowerCase().includes(query);
        const matchDesc = c.desc.toLowerCase().includes(query);
        if (query && !matchCmd && !matchDesc) continue;
        if (!categories[c.category]) { categories[c.category] = []; catOrder.push(c.category); }
        categories[c.category].push(c);
      }
      const idx = document.getElementById('atdbIndex');
      idx.innerHTML = '';
      for (const cat of catOrder) {
        const items = categories[cat];
        const hdr = document.createElement('div');
        hdr.className = 'atdb-cat-header';
        hdr.innerHTML = `<span>${escHtml(cat)}</span><span class="atdb-count">${items.length}</span>`;
        idx.appendChild(hdr);
        const list = document.createElement('div');
        list.className = 'atdb-cmd-list';
        for (const entry of items) {
          const item = document.createElement('div');
          item.className = 'atdb-cmd-item' + (atdbSelectedCmd === entry ? ' active' : '');
          item.title = entry.desc;
          item.innerHTML = highlightMatch(entry.cmd, query);
          item.addEventListener('click', () => selectAtdbCmd(entry));
          list.appendChild(item);
        }
        idx.appendChild(list);
      }
      if (catOrder.length === 0) {
        idx.innerHTML = '<div style="padding:16px 14px;font-size:12px;color:var(--text-muted);">无匹配命令</div>';
      }
    }

    function selectAtdbCmd(entry) {
      atdbSelectedCmd = entry;
      renderAtdbIndex();
      renderAtdbDetail(entry);
    }

    function renderAtdbDetail(entry) {
      const welcome = document.getElementById('atdbWelcome');
      const detail = document.getElementById('atdbDetail');
      if (welcome) welcome.style.display = 'none';
      detail.style.display = '';

      const platform = atdbPlatform;
      const badgeClass = platform === 'qualcomm' ? 'qualcomm' : platform === 'asr' ? 'asr' : 'unisoc';
      const badgeLabel = platform === 'qualcomm' ? '高通 Qualcomm' : platform === 'asr' ? 'ASR' : '展锐 UniSoc';

      let paramsHtml = '';
      if (entry.params && entry.params.length > 0) {
        paramsHtml = `
          <div class="atdb-section-label">参数说明</div>
          <table class="atdb-param-table">
            <thead><tr><th>参数</th><th>说明</th><th>取值</th></tr></thead>
            <tbody>
              ${entry.params.map(p => `<tr><td>${escHtml(p.name)}</td><td>${escHtml(p.desc)}</td><td>${escHtml(p.values)}</td></tr>`).join('')}
            </tbody>
          </table>`;
      }

      const exampleHtml = entry.example ? `
        <div class="atdb-section-label">示例</div>
        <div class="atdb-code">${escHtml(entry.example)}</div>` : '';

      const noteHtml = entry.note ? `<div class="atdb-note">${escHtml(entry.note)}</div>` : '';

      detail.innerHTML = `
        <div class="atdb-detail-header">
          <div class="atdb-detail-title">${escHtml(entry.cmd)}</div>
          <span class="atdb-platform-badge ${badgeClass}">${badgeLabel}</span>
        </div>
        <div class="atdb-detail-desc">${escHtml(entry.desc)}</div>
        <div class="atdb-section-label">语法</div>
        <div class="atdb-code" data-cp="${escHtml(entry.syntax).replace(/\n/g,'&#10;')}">${escHtml(entry.syntax)}<button class="atdb-copy-btn" onclick="atdbCopyParent(this)">${t('atdb_copy_btn')}</button></div>
        <div class="atdb-section-label">响应</div>
        <div class="atdb-code">${escHtml(entry.response)}</div>
        ${paramsHtml}
        ${exampleHtml}
        ${noteHtml}
      `;
    }

    function clearAtdbDetail() {
      const welcome = document.getElementById('atdbWelcome');
      const detail = document.getElementById('atdbDetail');
      if (welcome) welcome.style.display = '';
      if (detail) detail.style.display = 'none';
      atdbSelectedCmd = null;
    }

    function atdbCopyParent(btn) {
      const raw = btn.parentElement.getAttribute('data-cp') || '';
      navigator.clipboard.writeText(raw).then(() => {
        const orig = btn.textContent;
        btn.textContent = t('atdb_copy_done');
        setTimeout(() => { btn.textContent = orig; }, 1500);
      }).catch(() => {});
    }

    // ── 系统操作 ──
    function isQualcommModel(model) {
      if (!model) return false;
      const m = model.toUpperCase();
      return m.includes('RG520') || m.includes('RM520') || m.includes('RG525') ||
             m.includes('RG530') || m.includes('RM530') ||
             m.includes('RG500Q') || m.includes('RM500Q') || m.includes('RM501Q') || m.includes('RM551');
    }

    function resetHardwareTabs() {
      const unisocBtn = document.getElementById('tabBtnUnisoc');
      const qualcommBtn = document.getElementById('tabBtnQualcomm');
      if (unisocBtn) unisocBtn.classList.remove('disabled');
      if (qualcommBtn) qualcommBtn.classList.remove('disabled');
    }

    function switchStatusTab(tab, btn) {
      document.querySelectorAll('#page-status .tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('#page-status .tab-panel').forEach(p => p.classList.remove('active'));
      
      btn.classList.add('active');
      const panel = document.getElementById('stab-' + tab);
      if (panel) panel.classList.add('active');

      if (state.connected) {
        if (tab === 'network') {
          refreshModemStatus();
        } else if (tab === 'ip') {
          refreshIpInfo();
        }
      }
    }

    function switchHardwareTab(tab, btn) {
      if (btn.classList.contains('disabled')) return;

      document.querySelectorAll('#page-hardware .tab-btn').forEach(b => b.classList.remove('active'));
      document.querySelectorAll('#page-hardware .tab-panel').forEach(p => p.classList.remove('active'));
      
      btn.classList.add('active');
      const panel = document.getElementById('hwtab-' + tab);
      if (panel) panel.classList.add('active');
      
      if (state.connected) {
        if (tab === 'unisoc') {
          refreshFeatureToggles();
        } else if (tab === 'qualcomm') {
          refreshQualcommConfig();
        }
      }
    }

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
        setTextData('hwSn', hw.serialNumber || '--');
        if (hw.model) {
          state.model = hw.model;
          document.getElementById('logoText').textContent = hw.model;
          const isQualcomm = isQualcommModel(hw.model);
          const isAsr = hw.model.toUpperCase().includes('RG255');

          const unisocBtn = document.getElementById('tabBtnUnisoc');
          const qualcommBtn = document.getElementById('tabBtnQualcomm');
          const asrBtn = document.getElementById('tabBtnAsr');
          const unisocPanel = document.getElementById('hwtab-unisoc');
          const qualcommPanel = document.getElementById('hwtab-qualcomm');
          const asrPanel = document.getElementById('hwtab-asr');

          // Disable all tabs first
          if (unisocBtn) unisocBtn.classList.add('disabled');
          if (qualcommBtn) qualcommBtn.classList.add('disabled');
          if (asrBtn) asrBtn.classList.add('disabled');

          if (isQualcomm) {
            if (qualcommBtn) qualcommBtn.classList.remove('disabled');
            switchHardwareTab('qualcomm', qualcommBtn);
          } else if (isAsr) {
            if (asrBtn) asrBtn.classList.remove('disabled');
            switchHardwareTab('asr', asrBtn);
          } else {
            if (unisocBtn) unisocBtn.classList.remove('disabled');
            switchHardwareTab('unisoc', unisocBtn);
          }
        }
        addTerminalLine(`[系统] 型号=${hw.model} 固件=${hw.firmware}`, 'info');
      } catch (e) {
        addTerminalLine('[系统] 读取失败: ' + e, 'err');
      }
    }

    async function refreshQualcommConfig() {
      if (!state.connected) return;
      try {
        const config = await invoke('get_qualcomm_config');
        state.qualcommConfig = config;

        document.getElementById('qcUsbnet').value = String(config.usbnet);
        document.getElementById('qcDataInterface').value = config.dataInterface;
        document.getElementById('qcPcieMode').value = String(config.pcieMode);
        document.getElementById('qcUsbspeed').value = config.usbspeed;
        document.getElementById('qcEthDriver').value = config.ethDriver;

        ['qcUsbnet_0', 'qcUsbnet_1', 'qcUsbnet_2'].forEach((id, i) => {
          document.getElementById(id).classList.toggle('active', i === config.usbnet);
        });
        document.getElementById('qcDataInterface_usb').classList.toggle('active', config.dataInterface === '0,0');
        document.getElementById('qcDataInterface_pcie').classList.toggle('active', config.dataInterface === '1,0');
        document.getElementById('qcPcieMode_0').classList.toggle('active', config.pcieMode === 0);
        document.getElementById('qcPcieMode_1').classList.toggle('active', config.pcieMode === 1);
        document.getElementById('qcUsbspeed_20').classList.toggle('active', config.usbspeed === '20');
        document.getElementById('qcUsbspeed_311').classList.toggle('active', config.usbspeed === '311');
        document.getElementById('qcUsbspeed_312').classList.toggle('active', config.usbspeed === '312');
        [0, 1, 3].forEach(i => {
          document.getElementById('qcIppt_' + i).classList.toggle('active', config.ipptMode === i);
        });
        const acBadge = document.getElementById('qcAutoConnectBadge');
        if (config.ipptMode !== 0 && config.autoConnect === 1) {
          acBadge.style.display = 'block';
        } else {
          acBadge.style.display = 'none';
        }
      } catch (e) {
        console.warn('refreshQualcommConfig failed:', e);
      }
    }

    function _syncQcUsbnetUI(val) {
      document.getElementById('qcUsbnet').value = String(val);
      ['qcUsbnet_0', 'qcUsbnet_1', 'qcUsbnet_2'].forEach((id, i) => {
        document.getElementById(id).classList.toggle('active', i === val);
      });
    }
    function _syncQcDataInterfaceUI(val) {
      document.getElementById('qcDataInterface').value = val;
      document.getElementById('qcDataInterface_usb').classList.toggle('active', val === '0,0');
      document.getElementById('qcDataInterface_pcie').classList.toggle('active', val === '1,0');
    }
    function _syncQcPcieModeUI(val) {
      document.getElementById('qcPcieMode').value = String(val);
      document.getElementById('qcPcieMode_0').classList.toggle('active', val === 0);
      document.getElementById('qcPcieMode_1').classList.toggle('active', val === 1);
    }
    function _syncQcUsbspeedUI(val) {
      document.getElementById('qcUsbspeed').value = val;
      document.getElementById('qcUsbspeed_20').classList.toggle('active', val === '20');
      document.getElementById('qcUsbspeed_311').classList.toggle('active', val === '311');
      document.getElementById('qcUsbspeed_312').classList.toggle('active', val === '312');
    }

    async function setQcUsbnetToggle(val) {
      if (!state.connected) return;
      const prev = parseInt(document.getElementById('qcUsbnet').value);
      _syncQcUsbnetUI(val);
      try {
        await invoke('set_qualcomm_config', { param: 'usbnet', value: val.toString() });
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.usbnet = val;
        showToast('USB 网卡协议已设置，重启后生效', 'ok');
      } catch (e) {
        _syncQcUsbnetUI(prev);
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function setQcDataInterfaceToggle(val) {
      if (!state.connected) return;
      const prev = document.getElementById('qcDataInterface').value;
      _syncQcDataInterfaceUI(val);
      try {
        await invoke('set_qualcomm_config', { param: 'dataInterface', value: val });
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.dataInterface = val;
        showToast('数据接口模式已设置，重启后生效', 'ok');
      } catch (e) {
        _syncQcDataInterfaceUI(prev);
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function setQcPcieModeToggle(val) {
      if (!state.connected) return;
      const prev = parseInt(document.getElementById('qcPcieMode').value);
      _syncQcPcieModeUI(val);
      try {
        await invoke('set_qualcomm_config', { param: 'pcieMode', value: val.toString() });
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.pcieMode = val;
        showToast('PCIe 工作模式已设置，重启后生效', 'ok');
      } catch (e) {
        _syncQcPcieModeUI(prev);
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function setQcUsbspeedToggle(val) {
      if (!state.connected) return;
      const prev = document.getElementById('qcUsbspeed').value;
      _syncQcUsbspeedUI(val);
      try {
        await invoke('set_qualcomm_config', { param: 'usbspeed', value: val });
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.usbspeed = val;
        showToast('USB 接口速度已设置，重启后生效', 'ok');
      } catch (e) {
        _syncQcUsbspeedUI(prev);
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function setQcIpptMode(mode) {
      if (!state.connected) return;
      const prev = state.qualcommConfig?.ipptMode ?? 0;
      if (prev === mode) return;
      [0, 1, 3].forEach(i => document.getElementById('qcIppt_' + i).classList.toggle('active', i === mode));
      const labels = { 0: 'IPPT 已关闭', 1: 'IPPT ETH 已设置', 3: 'IPPT USB 已设置' };
      try {
        showLoading(mode === 0 ? '正在关闭 IPPT...' : (prev !== 0 ? '正在切换 IPPT 模式...' : '正在配置 IPPT...'));
        await invoke('set_qualcomm_config', { param: 'ippt', value: String(mode) });
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.ipptMode = mode;
        const acBadge = document.getElementById('qcAutoConnectBadge');
        acBadge.style.display = (mode !== 0) ? 'block' : 'none';
        hideLoading();
        showToast((labels[mode] || 'IPPT 已设置') + '，重启后生效', 'ok');
      } catch (e) {
        [0, 1, 3].forEach(i => document.getElementById('qcIppt_' + i).classList.toggle('active', i === prev));
        hideLoading();
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function saveQcEthDriver() {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const newDriver = document.getElementById('qcEthDriver').value;
      const oldDriver = state.qualcommConfig?.ethDriver ?? 'none';
      if (newDriver === oldDriver) { showToast('驱动未更改', 'info'); return; }
      try {
        showLoading('正在设置网口驱动...');
        // Disable old driver before enabling new one
        if (oldDriver !== 'none') {
          await invoke('set_qualcomm_config', { param: 'ethDriver', value: '-' + oldDriver });
        }
        if (newDriver !== 'none') {
          await invoke('set_qualcomm_config', { param: 'ethDriver', value: newDriver });
        }
        await flushAtLog();
        if (state.qualcommConfig) state.qualcommConfig.ethDriver = newDriver;
        hideLoading();
        showToast('网口驱动已设置，重启后生效', 'ok');
      } catch (e) {
        hideLoading();
        document.getElementById('qcEthDriver').value = oldDriver;
        showToast('设置失败: ' + e, 'err');
      }
    }

    async function refreshFeatureToggles() {
      if (!state.connected) return;
      try {
        const t = await invoke('get_feature_toggles');
        const toggleBtn = (key, val) => {
          const on = document.getElementById('toggle' + key + '_on');
          const off = document.getElementById('toggle' + key + '_off');
          if (on) on.classList.toggle('active', val);
          if (off) off.classList.toggle('active', !val);
          // Also update ASR tab buttons
          const onAsr = document.getElementById('toggle' + key + 'Asr_on');
          const offAsr = document.getElementById('toggle' + key + 'Asr_off');
          if (onAsr) onAsr.classList.toggle('active', val);
          if (offAsr) offAsr.classList.toggle('active', !val);
        };
        toggleBtn('Pcie', t.pcieMode);
        toggleBtn('Ethernet', t.ethernet);
        toggleBtn('EthAt', t.ethAt);
        toggleBtn('UartAt', t.uartAt);
        toggleBtn('Adb', t.adb);
        toggleBtn('ProxyArp', t.proxyarp);
        toggleBtn('Napt', t.napt);
        toggleBtn('Netmask', t.netmask);
        toggleBtn('ArmLog', t.armLog ?? false);
        toggleBtn('CpLog', t.cpLog ?? false);
      } catch (e) {
        console.warn('refreshFeatureToggles failed:', e);
      }
      try {
        const mode = await invoke('get_usbnet_mode');
        const sel = document.getElementById('usbNetMode');
        if (sel) sel.value = String(mode);
        const selAsr = document.getElementById('usbNetModeAsr');
        if (selAsr) selAsr.value = String(mode);
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

    async function changeUsbNetMode(sel) {
      if (!state.connected) { showToast('请先连接模组', 'err'); return; }
      const label = sel.selectedOptions[0].text;
      const value = parseInt(sel.value);
      try {
        showLoading(`正在设置 USB 网卡模式...`);
        await invoke('set_usbnet_mode', { mode: value });
        await flushAtLog();
        hideLoading();
        showToast(`USB 网卡模式已设置为「${label}」，重启后生效`, 'ok');
      } catch (e) {
        hideLoading();
        showToast('USB 网卡模式设置失败: ' + e, 'err');
      }
    }

    async function applyToggle(feature, enabled) {
      const labels = { adb: 'ADB', ethAt: 'ETH AT', uartAt: 'UART AT', pcieMode: 'PCIe ↔ 以太网', ethernet: 'Ethernet', proxyArp: 'Proxy ARP', napt: 'NAPT 端口转换', netmask: '动态子网掩码', armLog: 'ARM LOG', cpLog: 'CP LOG' };
      const keyMap = { adb: 'Adb', ethAt: 'EthAt', uartAt: 'UartAt', pcieMode: 'Pcie', ethernet: 'Ethernet', proxyArp: 'ProxyArp', napt: 'Napt', netmask: 'Netmask', armLog: 'ArmLog', cpLog: 'CpLog' };
      if (!state.connected) {
        showToast('请先连接模组', 'err');
        return;
      }
      const key = keyMap[feature];
      const onBtn = document.getElementById('toggle' + key + '_on');
      const offBtn = document.getElementById('toggle' + key + '_off');
      // Optimistic UI: update active state immediately so user sees feedback
      if (onBtn) onBtn.classList.toggle('active', enabled);
      if (offBtn) offBtn.classList.toggle('active', !enabled);
      try {
        await invoke('set_feature_toggle', { feature, enabled });
        await flushAtLog();
        addTerminalLine(`[功能] ${labels[feature]} 已${enabled ? '开启' : '关闭'}`, enabled ? 'ok' : 'info');
        showToast(`${labels[feature]} 已${enabled ? '开启' : '关闭'}`, enabled ? 'ok' : 'info');
      } catch (e) {
        addTerminalLine(`[功能] ${labels[feature]} 设置失败: ${e}`, 'err');
        showToast(`${labels[feature]} 设置失败`, 'err');
        // Revert optimistic UI on failure
        if (onBtn) onBtn.classList.toggle('active', !enabled);
        if (offBtn) offBtn.classList.toggle('active', enabled);
      }
    }

    async function toggleRf(enabled) {
      if (!state.connected) {
        const cb = document.getElementById('rfToggle');
        cb.checked = !enabled;
        showToast('请先连接模组', 'err');
        return;
      }
      try {
        await invoke('set_cfun', { mode: enabled ? 1 : 0 });
        await flushAtLog();
        addTerminalLine(`[射频] AT+CFUN=${enabled ? 1 : 0} 已执行`, enabled ? 'ok' : 'info');
        // 先清空旧值，避免射频变化期间显示过期状态
        ['regStatus', 'connStatus', 'operator', 'networkType', 'band', 'pci', 'arfcn', 'bandwidth'].forEach(id => {
          const el = document.getElementById(id);
          if (el) { el.textContent = '--'; el.className = 'info-value'; }
        });
        if (!enabled) {
          state.dataConnected = false;
          updateDataConnectionUI();
        }
        // 射频状态变化后主动刷新网络状态：关闭立即刷新，开启稍等模组重新驻网
        const delay = enabled ? 2000 : 800;
        setTimeout(async () => {
          await refreshModemStatus(true);
          try { await refreshApnList(); } catch (_) {}
          try { await refreshIpInfo(); } catch (_) {}
        }, delay);
      } catch (e) {
        const cb = document.getElementById('rfToggle');
        cb.checked = !enabled;
        addTerminalLine(`[射频] 设置失败: ${e}`, 'err');
        showToast('射频设置失败', 'err');
      }
    }

    async function syncRfState() {
      if (!state.connected) return;
      try {
        const resp = await invoke('send_raw_at', { command: 'AT+CFUN?' });
        const m = resp.match(/\+CFUN:\s*(\d+)/);
        if (m) {
          const on = m[1] === '1';
          const cb = document.getElementById('rfToggle');
          if (cb) cb.checked = on;
        }
      } catch (e) {
        console.warn('[CFUN] query failed:', e);
      }
    }

    async function confirmAction(action) {
      const labels = { reboot: '重启模组', factory: '恢复出厂设置（所有配置将丢失）' };
      const tauriConfirm = window.__TAURI__?.dialog?.confirm;
      const doConfirm = tauriConfirm
        ? (msg) => tauriConfirm(msg, { title: '确认操作', kind: 'warning' })
        : (msg) => confirm(msg);
      const ok = await doConfirm(`确认要执行：${labels[action]}？`);
      if (!ok) return;
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
      showLoading('正在读取 APN 配置...');
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
      hideLoading();
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

        // Connected port was physically removed → force disconnect.
        // Match by exact name OR by basename (handles /dev/ttyUSB0 vs ttyUSB0
        // discrepancies across serialport versions / ASR platforms).
        const portBasename = state.connectedPort ? state.connectedPort.replace(/^.*[\\/]/, '') : '';
        const removedBasenames = removed.map(r => r.replace(/^.*[\\/]/, ''));
        const portRemoved = state.connected && state.connectedPort && (
          removed.includes(state.connectedPort) ||
          removedBasenames.includes(portBasename)
        );

        if (portRemoved) {
          console.log('[USB] Connected port removed, disconnecting');
          addTerminalLine('[USB] AT端口已拔出，断开连接', 'cmd');
          if (state.dataConnected) {
            invoke('disconnect_data').catch(() => {});
          }
          // 后端 force_shutdown 非阻塞，正常情况下 <500ms 返回；
          // 加 3s 超时保护防止 UI 永远挂起。
          Promise.race([
            invoke('disconnect'),
            new Promise((_, rj) => setTimeout(() => rj(new Error('断开超时')), 3000)),
          ]).catch(() => {});
          state.connected = false;
          state.dataConnected = false;
          state.dataApn = '';
          state.connectedPort = '';
          state.idle = true;
          state.transport = undefined;
          updateConnectionUI(false);
          updateDataConnectionUI();
          clearData();
          const label = document.getElementById('statusLabel');
          label.textContent = '待机中';
          label.style.color = 'var(--text-muted)';
        }

        // Always refresh port list to reflect actual hardware state.
        // This runs regardless of connection status — even when connected,
        // the user should see the port disappear from the dropdown so they
        // know the hardware changed. refreshPortList re-enables the dropdown
        // when state.connected is false (set above by portRemoved handling).
        if (added.length > 0 || removed.length > 0) {
          refreshPortList().catch(() => {});
        }

        // New port appeared and we're idle → auto-connect
        if (!state.connected && state.idle && added.length > 0) {
          console.log('[USB] New port detected, trying auto-connect');
          addTerminalLine('[USB] 检测到新端口，等待设备就绪后连接...', 'info');
          setTimeout(() => {
            if (!state.connected && state.idle) toggleConnection(true);
          }, 5000);
        }
      });
    }

    function moveTabIndicator(indicator, btn) {
      const barPad = 3;
      indicator.style.width = btn.offsetWidth + 'px';
      indicator.style.transform = `translateX(${btn.offsetLeft - barPad}px)`;
    }

    function initTabSliders() {
      document.querySelectorAll('.tab-bar, .sub-tab-bar').forEach(bar => {
        const indicator = document.createElement('span');
        indicator.className = 'tab-indicator';
        bar.insertBefore(indicator, bar.firstChild);
        const active = bar.querySelector('.tab-btn.active, .sub-tab-btn.active');
        if (active) moveTabIndicator(indicator, active);
        bar.addEventListener('click', e => {
          const btn = e.target.closest('.tab-btn, .sub-tab-btn');
          if (btn && !btn.classList.contains('disabled') && bar.contains(btn)) {
            requestAnimationFrame(() => {
              const nowActive = bar.querySelector('.tab-btn.active, .sub-tab-btn.active');
              if (nowActive) moveTabIndicator(indicator, nowActive);
            });
          }
        });
      });
    }

    initTabSliders();

    async function doInit() {
      cacheDom();
      initMqttSetting();
      try {
        const isAuto = localStorage.getItem('ui-scale-auto') !== 'false';
        updateUiScaleModeUI(isAuto ? 'auto' : 'manual');
        const savedScale = parseFloat(localStorage.getItem('ui-scale')) || 1.0;
        updateUiScaleToggle(savedScale);
        const manualGroup = document.getElementById('manualScaleRow');
        if (manualGroup) manualGroup.style.opacity = isAuto ? '0.4' : '1.0';
      } catch (_) {}
      $.statusLabel.textContent = '正在初始化...';
      showLoading('正在初始化...', '加载连接参数');
      try {
        const ver = await invoke('get_app_version');
        if ($.appVersion) $.appVersion.textContent = 'v' + ver;
        if ($.aboutVersion) $.aboutVersion.textContent = 'v' + ver;
      } catch (_) {}
      try {
        await refreshConnectionParams();
      } catch (e) {
        addTerminalLine('[初始化] 端口/网卡列表获取失败: ' + e, 'err');
      }
      // 自动连接（USB 模式）
      try {
        await toggleConnection(true);
      } catch (e) {
        hideLoading();
        addTerminalLine('[初始化] 连接失败: ' + e, 'err');
      }
      // 启动 USB 监控
      setupUsbMonitor();
      if (window.initDebugTerminal) {
        await window.initDebugTerminal();
      }

      // 定时轮询端口列表（兜底：即使 port-changed 事件丢失，也能在 5s 内
      // 检测到端口变化）。仅在未连接时轮询，避免干扰 AT 操作。
      setInterval(() => {
        if (!state.connected) {
          refreshPortList().catch(() => {});
        }
      }, 5000);
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

    async function refreshNetworkAdapters() {
      const select = document.getElementById('connectionParams');
      try {
        const adapters = await invoke('list_network_adapters');
        select.innerHTML = '<option value="">-- 选择网卡 --</option>';
        if (adapters.length === 0) {
          select.innerHTML += '<option value="" disabled>未找到可用网卡</option>';
        } else {
          adapters.forEach(adapter => {
            const option = document.createElement('option');
            option.value = adapter.gateway;
            option.textContent = `${adapter.name} (${adapter.ip_address}) -> 网关: ${adapter.gateway} [${adapter.description}]`;
            select.appendChild(option);
          });
          if (adapters.length > 0) {
            select.value = adapters[0].gateway;
          }
        }
      } catch (e) {
        console.error('Failed to list network adapters:', e);
        select.innerHTML = '<option value="">刷新网卡失败: ' + escapeHtml(String(e).slice(0, 60)) + '</option>';
      }
    }

    async function refreshConnectionParams() {
      if (state.connected) {
        addTerminalLine('[连接] 检测到连接模式切换，正在自动断开当前连接...', 'info');
        try {
          await toggleConnection();
        } catch (e) {
          console.error('Auto-disconnect failed:', e);
        }
      }
      const connType = document.getElementById('connectionType')?.value || 'serial';
      if ($.connectionAuthRow) {
        $.connectionAuthRow.style.display = connType === 'ethernet' ? '' : 'none';
      }
      if (connType === 'serial') {
        await refreshPortList();
      } else if (connType === 'ethernet') {
        await refreshNetworkAdapters();
      }
    }
    window.refreshConnectionParams = refreshConnectionParams;

    // ── 刷新串口列表 ──
    async function refreshPortList() {
      const select = document.getElementById('connectionParams');
      try {
        const ports = await invoke('list_ports');
        const portNames = ports.map(p => p.portName);

        // Fallback disconnect detection: if we think we're connected but the
        // connected port is no longer in the system port list, the USB device
        // was removed. This catches cases where the USB-monitor's `removed`
        // diff missed the port name (Windows COM enumeration delay / format
        // mismatch) but list_ports now reflects reality.
        if (state.connected && state.connectedPort && !portNames.includes(state.connectedPort)) {
          console.log('[USB] Connected port vanished from list_ports, force-disconnecting');
          addTerminalLine('[USB] 连接端口已消失，断开连接', 'cmd');
          if (state.dataConnected) invoke('disconnect_data').catch(() => {});
          Promise.race([
            invoke('disconnect'),
            new Promise((_, rj) => setTimeout(() => rj(new Error('断开超时')), 3000)),
          ]).catch(() => {});
          state.connected = false;
          state.dataConnected = false;
          state.dataApn = '';
          state.connectedPort = '';
          state.idle = true;
          state.transport = undefined;
          updateConnectionUI(false);
          updateDataConnectionUI();
          clearData();
        }

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
        select.innerHTML = '<option value="">刷新失败: ' + escapeHtml(String(e).slice(0, 60)) + '</option>';
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

    // ─── Firmware Download Page ────────────────────────────────────────────
    const { listen: fwListen } = window.__TAURI__.event;

    const fw = {
      pacPath: null,
      report: null,
      downloading: false,
    };

    const RISK_LABEL = {
      Safe: ['安全','ok'], NvWrite: ['NV','warn'], RfCalibration: ['RF校准','err'],
      Erase: ['擦除','err'], EraseAll: ['全盘擦除','err'], PhaseCheck: ['PhaseCheck','info'],
    };

    function fwLog(line) {
      const el = document.getElementById('fwLog');
      if (!el) return;
      if (el.dataset.empty !== 'false') { el.textContent = ''; el.dataset.empty = 'false'; }
      el.textContent += (el.textContent ? '\n' : '') + line;
      el.scrollTop = el.scrollHeight;
    }

    function fwRenderReport(report) {
      const alertEl = document.getElementById('fwPacAlert');
      alertEl.innerHTML = `<div class="alert alert-success">PAC 加载完成（${report.total_files} 个文件）</div>`;

      const chips = document.getElementById('fwSafetyChips');
      const eraseOn = report.erase_files.length > 0;
      const nvOn = report.nv_files.length > 0;
      const rfBlocked = report.touches_rf_calibration || report.rf_cali_files.length > 0;
      const pcBlocked = report.phasecheck_files.length > 0;
      chips.innerHTML =
        `<span class="chip ${eraseOn ? 'warn' : 'ok'}">擦除: ${eraseOn ? '将执行' : '无'}</span>` +
        `<span class="chip ${nvOn ? 'warn' : 'ok'}">NV写入: ${nvOn ? '将执行(保留校准)' : '无'}</span>` +
        `<span class="chip ${rfBlocked ? 'err' : 'ok'}">射频校准: ${rfBlocked ? '含校准·已禁止' : '已保护'}</span>` +
        (pcBlocked ? `<span class="chip err">PhaseCheck: 含数据·已禁止</span>` : '');

      const rows = [];
      report.safe_files.forEach(id => rows.push({ id, risk: 'Safe', reason: '' }));
      [...report.nv_files, ...report.rf_cali_files, ...report.erase_files, ...report.phasecheck_files]
        .forEach(f => rows.push({ id: f.file_id, risk: f.risk_level, reason: f.reason }));
      const body = rows.map(r => {
        const [label, cls] = RISK_LABEL[r.risk] || [escHtml(r.risk) + ' ?', 'warn'];
        return `<tr><td class="id">${escHtml(r.id)}</td><td><span class="chip ${cls}" title="${escHtml(r.reason)}">${label}</span></td></tr>`;
      }).join('');
      document.getElementById('fwRiskWrap').innerHTML =
        `<details><summary style="cursor:pointer; font-size:12.5px; color:var(--text-secondary);">文件列表 (${report.total_files})</summary>` +
        `<table class="risk-table"><thead><tr><th>文件 ID</th><th>风险</th></tr></thead><tbody>${body}</tbody></table></details>`;

      const blocked = rfBlocked || pcBlocked;
      document.getElementById('fwStartBtn').disabled = blocked || fw.downloading;
      if (blocked) {
        alertEl.innerHTML += `<div class="alert alert-error" style="margin-top:8px;">此 PAC 含受保护分区（射频校准 / PhaseCheck），出于保护已禁止刷写。</div>`;
      }
    }

    function fwSetDownloading(on) {
      fw.downloading = on;
      document.getElementById('fwStartBtn').disabled = on || !fw.report;
      document.getElementById('fwStopBtn').disabled = !on;
      document.getElementById('fwSelectPacBtn').disabled = on;
      document.getElementById('fwStartBtn').textContent = on ? '下载中…' : '开始下载';
    }

    function fwSetResult(html) {
      document.getElementById('fwResult').innerHTML = html;
    }

    function fwSetProgress(label, pct) {
      const wrap = document.getElementById('fwProgressWrap');
      if (pct == null) { wrap.style.display = 'none'; return; }
      wrap.style.display = 'block';
      document.getElementById('fwProgressLabel').textContent = label;
      document.getElementById('fwProgressPct').textContent = `${Math.round(pct)}%`;
      document.getElementById('fwProgressFill').style.width = `${Math.round(pct)}%`;
    }

    // Select + analyze a PAC.
    document.getElementById('fwSelectPacBtn').addEventListener('click', async () => {
      const selBtn = document.getElementById('fwSelectPacBtn');
      let path;
      try {
        path = await invoke('pick_pac_file');
      } catch (e) {
        document.getElementById('fwPacAlert').innerHTML = `<div class="alert alert-error">分析失败: ${escHtml(e)}</div>`;
        return;
      }
      if (!path) return;
      selBtn.disabled = true;
      try {
        fw.pacPath = path;
        fw.report = null;
        document.getElementById('fwPacPath').textContent = path;
        document.getElementById('fwPacAlert').innerHTML = '<div class="alert alert-info">正在分析 PAC…</div>';
        document.getElementById('fwRiskWrap').innerHTML = '';
        document.getElementById('fwStartBtn').disabled = true;
        const report = await invoke('pac_info', { path });
        fw.report = report;
        fwRenderReport(report);
      } catch (e) {
        document.getElementById('fwPacAlert').innerHTML = `<div class="alert alert-error">分析失败: ${escHtml(e)}</div>`;
      } finally {
        selBtn.disabled = fw.downloading;
      }
    });

    // Start download.
    document.getElementById('fwStartBtn').addEventListener('click', async () => {
      if (!fw.pacPath) { showToast('请先选择 PAC 文件', 'error'); return; }
      try {
        document.getElementById('fwLog').textContent = '';
        document.getElementById('fwLog').dataset.empty = 'false';
        fwSetResult('');
        fwSetDownloading(true);
        fwSetProgress('等待设备…', 0);
        await invoke('start_firmware_download', { path: fw.pacPath });
      } catch (e) {
        fwSetDownloading(false);
        fwSetProgress(null);
        fwSetResult(`<div class="alert alert-error">${escHtml(e)}</div>`);
      }
    });

    // Stop download.
    document.getElementById('fwStopBtn').addEventListener('click', async () => {
      try {
        await invoke('stop_firmware_download');
        fwSetDownloading(false);
        fwSetProgress(null);
        fwSetResult('<div class="alert alert-info">下载已手动停止</div>');
      } catch (e) {
        showToast(`停止失败: ${e}`, 'error');
      }
    });

    // Listen to forwarded sidecar events.
    fwListen('firmware-event', (ev) => {
      const p = ev.payload || {};
      if (p.Log) {
        fwLog(`[${p.Log.level}] ${p.Log.message}`);
      } else if (p.Progress) {
        fwSetDownloading(true);
        fwSetProgress(p.Progress.file_id, p.Progress.percent);
      } else if (p.PacLoadProgress) {
        fwSetProgress('加载 PAC…', p.PacLoadProgress.percent);
      } else if (p.StateChange) {
        fwLog(`[状态] ${p.StateChange.from} → ${p.StateChange.to}`);
      } else if (p.Completed) {
        const r = p.Completed.result;
        fwSetDownloading(false);
        fwSetProgress(null);
        fwSetResult(r.success
          ? `<div class="alert alert-success">下载完成！耗时 ${Math.round((r.duration_ms||0)/1000)} 秒</div>`
          : `<div class="alert alert-error">下载失败: ${escHtml(r.error || '未知错误')}</div>`);
      } else if (p.Error) {
        fwSetDownloading(false);
        fwSetProgress(null);
        fwSetResult(`<div class="alert alert-error">错误 [${p.Error.code}]: ${escHtml(p.Error.message)}</div>`);
      } else if (p.Terminated) {
        if (fw.downloading) {
          fwSetDownloading(false);
          fwSetProgress(null);
          if (p.Terminated.code && p.Terminated.code !== 0) {
            fwSetResult(`<div class="alert alert-error">刷机进程异常退出 (code ${p.Terminated.code})</div>`);
          } else {
            fwSetResult('<div class="alert alert-info">刷机进程已停止</div>');
          }
        }
      }
    });

    // ─── Online Signal Monitoring ───

    function getThemeColor(varName, fallback) {
      try {
        const val = getComputedStyle(document.documentElement).getPropertyValue(varName).trim();
        return val || fallback;
      } catch (_) {
        return fallback;
      }
    }

    function formatRsrpFrontend(rsrpStr) {
      if (!rsrpStr || rsrpStr === '--') return '--';
      const trimmed = rsrpStr.toString().trim();
      if (trimmed.startsWith('-')) return trimmed;
      const val = parseSignalValue(trimmed);
      if (val !== null && val > 0) {
        return '-' + trimmed;
      }
      return trimmed;
    }

    function parseSignalValue(str) {
      if (!str) return null;
      const match = str.match(/(-?\d+(?:\.\d+)?)/);
      return match ? parseFloat(match[1]) : null;
    }

    class SignalChart {
      constructor(canvasId, options = {}) {
        this.canvasId = canvasId;
        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) return;
        this.ctx = this.canvas.getContext('2d');
        this.options = Object.assign({
          minY: -140,
          maxY: -40,
          gridCount: 5,
          unit: 'dBm',
          lineColor: '#06b6d4',
          gradientStart: 'rgba(6, 182, 212, 0.25)',
          gradientStop: 'rgba(6, 182, 212, 0)'
        }, options);
        this.data = []; // Array of { value: number, time: string }
        this.resize();
      }

      resize() {
        this.canvas = document.getElementById(this.canvasId);
        if (!this.canvas) return;
        this.ctx = this.canvas.getContext('2d');
        const rect = this.canvas.parentNode.getBoundingClientRect();
        const dpr = window.devicePixelRatio || 1;
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
        this.ctx.resetTransform();
        this.ctx.scale(dpr, dpr);
        this.draw();
      }

      addData(value, timeStr) {
        this.data.push({ value, time: timeStr || new Date().toLocaleTimeString().slice(-8) });
        if (this.data.length > 25) {
          this.data.shift();
        }
        this.draw();
      }

      clear() {
        this.data = [];
        this.draw();
      }

      draw() {
        if (!this.canvas || !this.ctx) return;
        const ctx = this.ctx;
        const width = this.canvas.width / (window.devicePixelRatio || 1);
        const height = this.canvas.height / (window.devicePixelRatio || 1);

        ctx.clearRect(0, 0, width, height);

        // Padding around the drawing area
        const paddingLeft = 65;
        const paddingRight = 20;
        const paddingTop = 20;
        const paddingBottom = 25;

        const chartWidth = width - paddingLeft - paddingRight;
        const chartHeight = height - paddingTop - paddingBottom;

        if (chartWidth <= 0 || chartHeight <= 0) return;

        // Get current theme colors
        const theme = document.documentElement.getAttribute('data-theme') || 'dark';
        const textColor = getThemeColor('--text-secondary', '#94a3b8');
        const gridColor = getThemeColor('--border-color', 'rgba(30, 41, 69, 0.6)');
        
        // Draw Y-axis grid and labels
        const minY = this.options.minY;
        const maxY = this.options.maxY;
        const rangeY = maxY - minY;
        const gridCount = this.options.gridCount;

        ctx.font = '10px "JetBrains Mono", monospace';
        ctx.textAlign = 'right';
        ctx.textBaseline = 'middle';

        for (let i = 0; i <= gridCount; i++) {
          const yVal = minY + (rangeY * i) / gridCount;
          const yPos = paddingTop + chartHeight - (chartHeight * i) / gridCount;

          // Draw grid line
          ctx.beginPath();
          ctx.strokeStyle = gridColor;
          ctx.lineWidth = 1;
          ctx.moveTo(paddingLeft, yPos);
          ctx.lineTo(width - paddingRight, yPos);
          ctx.stroke();

          // Draw label
          ctx.fillStyle = textColor;
          ctx.fillText(`${Math.round(yVal)}${this.options.unit}`, paddingLeft - 8, yPos);
        }

        // Draw X-axis line (at the bottom)
        ctx.beginPath();
        ctx.strokeStyle = gridColor;
        ctx.lineWidth = 1;
        ctx.moveTo(paddingLeft, paddingTop + chartHeight);
        ctx.lineTo(width - paddingRight, paddingTop + chartHeight);
        ctx.stroke();

        // Check if there is data to draw
        if (this.data.length === 0) {
          ctx.textAlign = 'center';
          ctx.fillStyle = textColor;
          ctx.font = '13px "Inter", sans-serif';
          ctx.fillText(state.lang === 'zh' ? '暂无监控数据 (请开启开关)' : 'No data (turn on switch)', paddingLeft + chartWidth / 2, paddingTop + chartHeight / 2);
          return;
        }

        // Calculate positions of data points
        const points = [];
        for (let i = 0; i < this.data.length; i++) {
          const d = this.data[i];
          // X-coord distributed evenly
          const x = paddingLeft + (chartWidth * i) / Math.max(1, this.data.length - 1);
          
          // Y-coord based on value clamped to min/max
          const clampedVal = Math.max(minY, Math.min(maxY, d.value));
          const y = paddingTop + chartHeight - (chartHeight * (clampedVal - minY)) / rangeY;
          points.push({ x, y, val: d.value, time: d.time });
        }

        // Draw area under curve (filled gradient)
        ctx.beginPath();
        ctx.moveTo(points[0].x, paddingTop + chartHeight);
        for (let i = 0; i < points.length; i++) {
          ctx.lineTo(points[i].x, points[i].y);
        }
        ctx.lineTo(points[points.length - 1].x, paddingTop + chartHeight);
        ctx.closePath();

        const gradient = ctx.createLinearGradient(0, paddingTop, 0, paddingTop + chartHeight);
        gradient.addColorStop(0, this.options.gradientStart);
        gradient.addColorStop(1, this.options.gradientStop);
        ctx.fillStyle = gradient;
        ctx.fill();

        // Draw line connecting points
        ctx.beginPath();
        ctx.moveTo(points[0].x, points[0].y);
        for (let i = 1; i < points.length; i++) {
          ctx.lineTo(points[i].x, points[i].y);
        }
        ctx.strokeStyle = this.options.lineColor;
        ctx.lineWidth = 2.5;
        ctx.lineJoin = 'round';
        ctx.stroke();

        // Draw point circles and latest pulse
        for (let i = 0; i < points.length; i++) {
          const p = points[i];
          
          // Standard point dot
          ctx.beginPath();
          ctx.fillStyle = this.options.lineColor;
          ctx.arc(p.x, p.y, i === points.length - 1 ? 5 : 3.5, 0, Math.PI * 2);
          ctx.fill();

          // Outer border for dot to pop
          ctx.beginPath();
          ctx.strokeStyle = theme === 'light' ? '#ffffff' : '#101524';
          ctx.lineWidth = 1.5;
          ctx.arc(p.x, p.y, i === points.length - 1 ? 5 : 3.5, 0, Math.PI * 2);
          ctx.stroke();

          // If it's the last point, draw a beautiful glow pulse
          if (i === points.length - 1) {
            ctx.beginPath();
            ctx.strokeStyle = this.options.lineColor;
            ctx.lineWidth = 1;
            ctx.arc(p.x, p.y, 9, 0, Math.PI * 2);
            ctx.stroke();
          }
        }

        // Draw X-axis timestamps (limit count to avoid overlapping)
        ctx.fillStyle = textColor;
        ctx.font = '9px "JetBrains Mono", monospace';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'top';

        const step = Math.max(1, Math.floor(points.length / 5));
        for (let i = 0; i < points.length; i += step) {
          const p = points[i];
          ctx.fillText(p.time, p.x, paddingTop + chartHeight + 6);
        }
        // Always draw the last timestamp if not already drawn
        if ((points.length - 1) % step !== 0) {
          const p = points[points.length - 1];
          ctx.fillText(p.time, p.x, paddingTop + chartHeight + 6);
        }
      }
    }

    let rsrpChart = null;
    let sinrChart = null;
    let monitorTimer = null;

    window.redrawCharts = function() {
      if (!rsrpChart || !sinrChart) return;
      const isLight = document.documentElement.getAttribute('data-theme') === 'light';
      
      rsrpChart.options.lineColor = getThemeColor('--cyan', '#06b6d4');
      rsrpChart.options.gradientStart = isLight ? 'rgba(8, 145, 178, 0.2)' : 'rgba(6, 182, 212, 0.25)';
      
      sinrChart.options.lineColor = getThemeColor('--accent', '#f97316');
      sinrChart.options.gradientStart = isLight ? 'rgba(234, 88, 12, 0.15)' : 'rgba(249, 115, 22, 0.25)';
      
      rsrpChart.resize();
      sinrChart.resize();
    };

    window.initMonitorPage = function() {
      const isLight = document.documentElement.getAttribute('data-theme') === 'light';
      if (!rsrpChart) {
        rsrpChart = new SignalChart('rsrpChartCanvas', {
          minY: -140,
          maxY: -40,
          gridCount: 5,
          unit: ' dBm',
          lineColor: getThemeColor('--cyan', '#06b6d4'),
          gradientStart: isLight ? 'rgba(8, 145, 178, 0.2)' : 'rgba(6, 182, 212, 0.25)',
          gradientStop: 'rgba(6, 182, 212, 0)'
        });
      }
      if (!sinrChart) {
        sinrChart = new SignalChart('sinrChartCanvas', {
          minY: -20,
          maxY: 40,
          gridCount: 6,
          unit: ' dB',
          lineColor: getThemeColor('--accent', '#f97316'),
          gradientStart: isLight ? 'rgba(234, 88, 12, 0.15)' : 'rgba(249, 115, 22, 0.25)',
          gradientStop: 'rgba(249, 115, 22, 0)'
        });
      }
      
      window.redrawCharts();
    };

    async function fetchMonitorData() {
      if (!state.connected) {
        // Disconnected, auto shut down
        const sw = document.getElementById('monitorToggleSwitch');
        if (sw && sw.checked) {
          sw.checked = false;
          toggleMonitorState(false);
          showToast(state.lang === 'zh' ? '模组已断开，监控关闭' : 'Modem disconnected, monitoring stopped', 'err');
        }
        return;
      }

      try {
        const s = await invoke('get_modem_status');
        const timeStr = new Date().toLocaleTimeString().slice(-8);

        const formattedRsrp = formatRsrpFrontend(s.rsrp);
        const rsrpVal = parseSignalValue(formattedRsrp);
        const sinrVal = parseSignalValue(s.sinr);

        const rsrpBadge = document.getElementById('monitorRsrpBadge');
        const sinrBadge = document.getElementById('monitorSinrBadge');

        if (rsrpVal !== null) {
          if (rsrpChart) rsrpChart.addData(rsrpVal, timeStr);
          if (rsrpBadge) rsrpBadge.textContent = formattedRsrp;
        } else {
          if (rsrpBadge) rsrpBadge.textContent = '--';
        }

        if (sinrVal !== null) {
          if (sinrChart) sinrChart.addData(sinrVal, timeStr);
          if (sinrBadge) sinrBadge.textContent = s.sinr;
        } else {
          if (sinrBadge) sinrBadge.textContent = '--';
        }
      } catch (e) {
        console.error('Failed to query signal status:', e);
        addTerminalLine('[监控] 获取信号失败: ' + (e.message || String(e)), 'err');
      }
    }

    function toggleMonitorState(active) {
      if (monitorTimer) {
        clearInterval(monitorTimer);
        monitorTimer = null;
      }

      if (active) {
        const intervalSelect = document.getElementById('monitorIntervalSelect');
        const intervalSec = intervalSelect ? parseInt(intervalSelect.value) || 10 : 10;
        
        // Fetch once immediately
        fetchMonitorData();
        
        monitorTimer = setInterval(fetchMonitorData, intervalSec * 1000);
      }
    }

    function initMonitorEvents() {
      const sw = document.getElementById('monitorToggleSwitch');
      if (sw) {
        sw.addEventListener('change', (e) => {
          if (e.target.checked) {
            if (!state.connected) {
              showToast(state.lang === 'zh' ? '请先连接模组' : 'Please connect the modem first', 'err');
              e.target.checked = false;
              return;
            }
            toggleMonitorState(true);
          } else {
            toggleMonitorState(false);
          }
        });
      }

      const select = document.getElementById('monitorIntervalSelect');
      if (select) {
        select.addEventListener('change', () => {
          const sw = document.getElementById('monitorToggleSwitch');
          if (sw && sw.checked && state.connected) {
            // Restart timer with new interval
            toggleMonitorState(true);
          }
        });
      }
    }

    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', initMonitorEvents);
    } else {
      initMonitorEvents();
    }

    window.addEventListener('resize', () => {
      if (rsrpChart && sinrChart && document.getElementById('page-monitor').classList.contains('active')) {
        rsrpChart.resize();
        sinrChart.resize();
      }
    });

    // ── Expose all onclick-referenced functions to window ──
    // Tauri v2 WebView2 may not resolve inline onclick handlers to scoped
    // function declarations in all configurations. Explicitly binding them
    // to window ensures every onclick="fn()" in index.html works reliably.
    const _onclickFns = {
      applyBandLock, applyDmz, applyLanConfig, applyMtu,
      applyOperatorLock, applyPreferredNetwork, applyToggle, applyVlan,
      clearCellLock, clearOperatorLock, closeApnModal,
      configureQualcomm5Glan, confirmAction,
      connectQualcomm5Glan, dismissSceneReboot, enableEthPdu,
      hideAbout, openApnModal, quickAt,
      refresh5GlanQualcommStatus, refreshHardwareInfo, refreshIpInfo,
      refreshModemStatus, refreshNeighbors,
      resetBandLock, saveApn, saveCellLock, saveQcEthDriver, saveUnisoc5Glan,
      sendAtCommand, setIms, setLang, setMqttEnabled,
      setQcDataInterfaceToggle, setQcIpptMode, setQcPcieModeToggle,
      setQcUsbnetToggle, setQcUsbspeedToggle,
      setTheme, setUiScale, setUiScaleMode,
      switch5GlanTab, switchCellularTab, switchHardwareTab,
      switchNeighborTab, switchStatusTab,
      toggleConnection, toggleDataConnection, toggleRf,
      toggleSimSlotDropdown,
    };
    Object.keys(_onclickFns).forEach(fn => {
      if (typeof _onclickFns[fn] === 'function') {
        window[fn] = _onclickFns[fn];
      }
    });

