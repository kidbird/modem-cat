    const _rawInvoke = window.__TAURI__?.core?.invoke;
    const invoke = (cmd, args) => {
      if (!_rawInvoke) {
        console.warn(`Tauri IPC not available: invoke('${cmd}')`);
        return Promise.reject(new Error('Tauri IPC not available'));
      }
      return _rawInvoke(cmd, args);
    };

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
      lang: localStorage.getItem('lang') || 'zh',
      model: '',
      chipVendor: '',
      currentBand: '',
      vlanEnabled: false,
    };

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

    // ── Theme ──
    (function () {
      const saved = localStorage.getItem('theme');
      if (saved === 'light') setTheme('light');
      else if (saved === 'blue-light') setTheme('blue-light');
      else setTheme('dark');
    })();

    function toggleTheme() {
      const current = localStorage.getItem('theme') || 'dark';
      if (current === 'dark') setTheme('light');
      else if (current === 'light') setTheme('blue-light');
      else setTheme('dark');
    }

    function setTheme(theme) {
      // state.isDark was a 2-state flag that no longer reflects the 3 themes
      // (dark/light/blue-light) — toggleTheme + updateThemeToggle both read
      // localStorage directly. Drop the dead-write; the `isDark: true` field
      // in the state literal above is harmless initial seed.
      document.documentElement.setAttribute('data-theme', theme);
      localStorage.setItem('theme', theme);
      updateThemeToggle(theme);
    }

    // Pass theme as a parameter (single source of truth = the argument to setTheme).
    // Previously this re-read localStorage, which created 3 sources of truth
    // (setTheme arg / data-theme attribute / localStorage) and could desync.
    function updateThemeToggle(theme) {
      const darkBtn = document.getElementById('themeDark');
      const lightBtn = document.getElementById('themeLight');
      const blueLightBtn = document.getElementById('themeBlueLight');
      if (darkBtn) darkBtn.classList.toggle('active', theme === 'dark');
      if (lightBtn) lightBtn.classList.toggle('active', theme === 'light');
      if (blueLightBtn) blueLightBtn.classList.toggle('active', theme === 'blue-light');
    }

    const LANG = {
      zh: {
        nav_status: '模组状态', nav_cellular: '蜂窝网络', nav_ip: 'IP 配置',
        nav_at: 'AT 调试', nav_hardware: '模组配置', nav_settings: '系统设置',
        nav_scene: '情景模式', nav_atmanual: 'AT手册',
        atdb_search_ph: '搜索 AT 命令...', atdb_welcome: '从左侧选择命令查看详情',
        atdb_copy_btn: '复制', atdb_copy_done: '已复制',
        scene_active_badge: '已激活', scene_inactive_badge: '未激活',
        scene_activate_btn: '激活此模式',
        scene_reboot_notice: '配置已应用，请手动重启模组使设置生效',
        scene_qualcomm_placeholder: '高通情景模式即将推出',
        btn_reboot: '重启模组', btn_factory: '恢复出厂',
        status_modem: '模组', status_init: '初始化中...', status_data: '数据',
        status_disconnected: '未连接', btn_data_connect: '连接',
        panel_interface: '接口配置', label_conn_type: '连接类型',
        opt_usb_serial: 'USB / TTL Serial', opt_eth_tcp: 'Ethernet TCP',
        label_port_addr: '端口 / 地址', opt_select_port: '-- 选择端口 --',
        btn_connect: '连接', panel_net_status: '网络状态',
        label_sim: 'SIM 状态', label_reg: '注册状态', label_conn_status: '连接状态',
        label_imei: 'IMEI', label_iccid: 'ICCID', label_operator: '运营商',
        label_net_type: '网络类型', label_band: 'Band', label_pci: 'PCI', label_cell_id: 'Cell ID',
        label_arfcn: 'ARFCN（频点）', label_bandwidth: '频宽',
        label_rsrp: 'RSRP', label_rsrq: 'RSRQ', label_sinr: 'SINR',
        label_tx_power: 'TX Power', label_rx_level: 'RX Level',
        label_cqi: 'CQI', label_scs: 'SCS',
        panel_antenna: '天线信号', label_ant0: 'ANT 0', label_ant1: 'ANT 1',
        label_ant2: 'ANT 2', label_ant3: 'ANT 3',
        panel_traffic: '流量统计', label_ul_bw: '签约上行带宽',
        label_dl_bw: '签约下行带宽', label_ul_traffic: '上行流量',
        label_dl_traffic: '下行流量',
        tab_apn: 'APN 配置', tab_network: '网络配置',
        tab_lock: '小区 / 频点锁定', tab_neighbor: '邻区信息', tab_5glan: '5GLAN',
        label_5glan_cid: 'CID', label_5glan_vlan: 'VLAN ID', btn_add_5glan: '添加 / 应用',
        btn_add_apn: '新增 APN', panel_pref_net: '首选网络', btn_save: '保存',
        opt_auto: '自动', opt_nr5g_only: '仅 5G', opt_lte_only: '仅 LTE', opt_wcdma: '仅 WCDMA',
        panel_band: '频段设置', btn_reset: '重置', panel_plmn: 'PLMN 锁定',
        btn_lock: '锁定', btn_unlock: '解锁', label_mcc: 'MCC', label_mnc: 'MNC',
        label_plmn_password: '锁定密码',
        lock_none: '当前锁定：无',
        label_arfcn_input: '频点（EARFCN / NR-ARFCN）',
        label_pci_optional: 'PCI（可选，不填则只锁频点）',
        label_pci_required: 'PCI（必填）',
        label_scs: 'SCS (kHz)', label_band_nr: 'Band',
        btn_save_lock: '保存', btn_clear_lock: '清除锁定',
        btn_neighbor_refresh: '刷新',
        subtab_lte: 'LTE 邻区', subtab_nr: 'NR 邻区', no_data: '暂无数据',
        panel_at: 'AT 命令终端', btn_send: '发送', btn_clear_at: '清除',
        panel_hw_info: '模组信息', btn_hw_refresh: '刷新',
        label_model: '模组型号', label_manufacturer: '生产厂家',
        label_firmware: '固件版本', label_ap_baseline: 'AP 基线版本',
        label_cp_baseline: 'CP 基线版本', label_soc_temp: 'SOC 温度',
        label_pa_temp: 'PA 温度', panel_toggles: '功能开关',
        toggle_pcie: 'PCIe ↔ 以太网', toggle_ethernet: 'Ethernet',
        toggle_eth_at: 'ETH AT', toggle_uart_at: 'UART AT', toggle_adb: 'ADB',
        toggle_proxy_arp: 'Proxy ARP', toggle_napt: 'NAPT 端口转换',
        toggle_netmask: '动态子网掩码', toggle_arm_log: 'ARM LOG',
        toggle_arm_log_desc: 'ARM 侧日志输出', toggle_cp_log: 'CP LOG',
        toggle_cp_log_desc: 'CP 侧日志输出',
        panel_usb_mode: 'USB 网卡模式', label_usb_proto: 'USB 网卡协议',
        opt_ecm: 'ECM 网卡', opt_mbim: 'MBIM 移动宽带',
        opt_rndis: 'RNDIS 网卡', opt_ncm: 'NCM 网卡',
        btn_save_reboot: '保存并重启', panel_ip: 'IP 地址', btn_ip_refresh: '刷新',
        label_ipv4: 'IPv4 地址', label_ipv4_mask: 'IPv4 子网掩码',
        label_ipv4_gw: 'IPv4 网关', label_ipv4_dns: 'IPv4 DNS',
        label_ipv6: 'IPv6 地址', label_ipv6_dns: 'IPv6 DNS',
        panel_mtu: 'MTU 配置', label_mtu: 'MTU 值（字节）', btn_apply: '应用',
        mtu_hint: '推荐值：1500（以太网）/ 1480（PPPoE）/ 1400（VPN）',
        label_dmz: 'DMZ 主机 IP 地址', btn_apply_dmz: '应用', btn_clear_dmz: '清除',
        dmz_warn: 'DMZ 主机将接收所有未映射的入站流量，请确保主机具备必要的安全防护。',
        panel_vlan: 'VLAN 配置', label_vlan_off: '已关闭', label_vlan_on: '已启用',
        vlan_not_supported: '仅 Qualcomm 模组支持 VLAN 配置',
        label_vlan_id: 'VLAN ID', btn_vlan_apply: '应用', btn_vlan_disable: '禁用当前',
        label_apn_name: 'APN 名称', label_apn_user: '用户名', label_apn_pass: '密码',
        label_auth_type: '鉴权类型', label_ip_type: 'IP 类型',
        opt_ipv4v6: 'IPv4v6（双栈）', btn_cancel: '取消', btn_confirm_save: '确认保存',
        about_desc: '5G Modem 调试桌面工具，支持远程/本地 AT 串口连接、实时信号监控、频段配置、网络状态查询等功能。',
        about_os: '支持的操作系统',
        os_linux_note: 'Linux 需预装：libgtk-3, libwebkit2gtk-4.1, libudev1, libappindicator3 等运行库。',
        btn_ok: '确定',
        ph_apn_name: 'cmnet', ph_arfcn: '如 500300', ph_at_cmd: '输入 AT 命令...',
        ph_dmz: '如 192.168.1.100', ph_optional: '（可选）', ph_pci: '如 123',
        ph_plmn_password: '供应商提供',
        label_language: '语言 / Language', label_theme: '外观主题',
        theme_dark: '深色', theme_light: '浅色', theme_blue_light: '科技蓝', label_app_version: '当前版本',
      },
      en: {
        nav_status: 'Modem Status', nav_cellular: 'Cellular', nav_ip: 'IP Config',
        nav_at: 'AT Debug', nav_hardware: 'Module Config', nav_settings: 'Settings',
        nav_scene: 'Scene Mode', nav_atmanual: 'AT Reference',
        atdb_search_ph: 'Search AT commands...', atdb_welcome: 'Select a command from the left',
        atdb_copy_btn: 'Copy', atdb_copy_done: 'Copied',
        scene_active_badge: 'Active', scene_inactive_badge: 'Inactive',
        scene_activate_btn: 'Activate',
        scene_reboot_notice: 'Settings applied. Please reboot the modem manually to take effect.',
        scene_qualcomm_placeholder: 'Qualcomm scene mode coming soon',
        btn_reboot: 'Reboot', btn_factory: 'Factory Reset',
        status_modem: 'Modem', status_init: 'Initializing...', status_data: 'Data',
        status_disconnected: 'Disconnected', btn_data_connect: 'Connect',
        panel_interface: 'Interface', label_conn_type: 'Connection Type',
        opt_usb_serial: 'USB / TTL Serial', opt_eth_tcp: 'Ethernet TCP',
        label_port_addr: 'Port / Address', opt_select_port: '-- Select Port --',
        btn_connect: 'Connect', panel_net_status: 'Network Status',
        label_sim: 'SIM Status', label_reg: 'Registration',
        label_conn_status: 'Conn. Status', label_imei: 'IMEI', label_iccid: 'ICCID',
        label_operator: 'Operator', label_net_type: 'Network Type', label_band: 'Band',
        label_pci: 'PCI', label_cell_id: 'Cell ID', label_arfcn: 'ARFCN',
        label_bandwidth: 'Bandwidth', label_rsrp: 'RSRP', label_rsrq: 'RSRQ',
        label_sinr: 'SINR', label_tx_power: 'TX Power', label_rx_level: 'RX Level',
        label_cqi: 'CQI', label_scs: 'SCS',
        panel_antenna: 'Antenna Signal', label_ant0: 'ANT 0', label_ant1: 'ANT 1',
        label_ant2: 'ANT 2', label_ant3: 'ANT 3',
        panel_traffic: 'Traffic Stats', label_ul_bw: 'UL Bandwidth',
        label_dl_bw: 'DL Bandwidth', label_ul_traffic: 'Upload',
        label_dl_traffic: 'Download',
        tab_apn: 'APN Config', tab_network: 'Network Config',
        tab_lock: 'Cell / Freq Lock', tab_neighbor: 'Neighbor Cells', tab_5glan: '5GLAN',
        label_5glan_cid: 'CID', label_5glan_vlan: 'VLAN ID', btn_add_5glan: 'Add / Apply',
        btn_add_apn: 'Add APN', panel_pref_net: 'Preferred Network', btn_save: 'Save',
        opt_auto: 'Auto', opt_nr5g_only: '5G Only', opt_lte_only: 'LTE Only', opt_wcdma: 'WCDMA Only',
        panel_band: 'Band Settings', btn_reset: 'Reset', panel_plmn: 'PLMN Lock',
        btn_lock: 'Lock', btn_unlock: 'Unlock', label_mcc: 'MCC', label_mnc: 'MNC',
        label_plmn_password: 'Lock Password',
        lock_none: 'Current Lock: None',
        label_arfcn_input: 'Frequency (EARFCN / NR-ARFCN)',
        label_pci_optional: 'PCI (optional, freq-only if blank)',
        label_pci_required: 'PCI (required)',
        label_scs: 'SCS (kHz)', label_band_nr: 'Band',
        btn_save_lock: 'Save', btn_clear_lock: 'Clear Lock',
        btn_neighbor_refresh: 'Refresh',
        subtab_lte: 'LTE Neighbors', subtab_nr: 'NR Neighbors', no_data: 'No Data',
        panel_at: 'AT Terminal', btn_send: 'Send', btn_clear_at: 'Clear',
        panel_hw_info: 'Module Info', btn_hw_refresh: 'Refresh',
        label_model: 'Model', label_manufacturer: 'Manufacturer',
        label_firmware: 'Firmware', label_ap_baseline: 'AP Baseline',
        label_cp_baseline: 'CP Baseline', label_soc_temp: 'SOC Temp',
        label_pa_temp: 'PA Temp', panel_toggles: 'Feature Toggles',
        toggle_pcie: 'PCIe ↔ Ethernet', toggle_ethernet: 'Ethernet',
        toggle_eth_at: 'ETH AT', toggle_uart_at: 'UART AT', toggle_adb: 'ADB',
        toggle_proxy_arp: 'Proxy ARP', toggle_napt: 'NAPT',
        toggle_netmask: 'Dynamic Netmask', toggle_arm_log: 'ARM LOG',
        toggle_arm_log_desc: 'ARM side log output', toggle_cp_log: 'CP LOG',
        toggle_cp_log_desc: 'CP side log output',
        panel_usb_mode: 'USB Mode', label_usb_proto: 'USB Protocol',
        opt_ecm: 'ECM', opt_mbim: 'MBIM', opt_rndis: 'RNDIS', opt_ncm: 'NCM',
        btn_save_reboot: 'Save & Reboot', panel_ip: 'IP Address',
        btn_ip_refresh: 'Refresh', label_ipv4: 'IPv4 Address',
        label_ipv4_mask: 'IPv4 Netmask', label_ipv4_gw: 'IPv4 Gateway',
        label_ipv4_dns: 'IPv4 DNS', label_ipv6: 'IPv6 Address',
        label_ipv6_dns: 'IPv6 DNS', panel_mtu: 'MTU Config',
        label_mtu: 'MTU (bytes)', btn_apply: 'Apply',
        mtu_hint: 'Recommended: 1500 (Ethernet) / 1480 (PPPoE) / 1400 (VPN)',
        label_dmz: 'DMZ Host IP', btn_apply_dmz: 'Apply', btn_clear_dmz: 'Clear',
        dmz_warn: 'DMZ host receives all unmapped inbound traffic. Ensure it is properly secured.',
        panel_vlan: 'VLAN Config', label_vlan_off: 'Disabled', label_vlan_on: 'Enabled',
        vlan_not_supported: 'VLAN configuration is only supported on Qualcomm modems',
        label_vlan_id: 'VLAN ID', btn_vlan_apply: 'Apply', btn_vlan_disable: 'Disable Current',
        label_apn_name: 'APN Name', label_apn_user: 'Username',
        label_apn_pass: 'Password', label_auth_type: 'Auth Type',
        label_ip_type: 'IP Type', opt_ipv4v6: 'IPv4v6 (Dual Stack)',
        btn_cancel: 'Cancel', btn_confirm_save: 'Save',
        about_desc: '5G Modem debugging desktop app. Supports AT serial/TCP, real-time signal monitoring, band configuration, and network diagnostics.',
        about_os: 'Supported OS',
        os_linux_note: 'Linux requires: libgtk-3, libwebkit2gtk-4.1, libudev1, libappindicator3 and other runtime libraries.',
        btn_ok: 'OK',
        ph_apn_name: 'e.g. internet', ph_arfcn: 'e.g. 500300',
        ph_at_cmd: 'Enter AT command...', ph_dmz: 'e.g. 192.168.1.100',
        ph_optional: '(optional)', ph_pci: 'e.g. 123',
        ph_plmn_password: 'Provided by vendor',
        label_language: 'Language / 语言', label_theme: 'Theme',
        theme_dark: 'Dark', theme_light: 'Light', theme_blue_light: 'Tech Blue', label_app_version: 'Version',
      },
    };

    function t(key) {
      return (LANG[state.lang] || LANG.zh)[key] || key;
    }

    function applyI18n() {
      document.querySelectorAll('[data-i18n]').forEach(el => {
        const text = t(el.getAttribute('data-i18n'));
        if (text) el.textContent = text;
      });
      document.querySelectorAll('[data-i18n-ph]').forEach(el => {
        const text = t(el.getAttribute('data-i18n-ph'));
        if (text) el.placeholder = text;
      });
      const btn = document.getElementById('langBtn');
      if (btn) btn.textContent = state.lang === 'zh' ? 'EN' : '中';
      document.documentElement.lang = state.lang === 'zh' ? 'zh-CN' : 'en';
      const zhBtn = document.getElementById('langZh');
      const enBtn = document.getElementById('langEn');
      if (zhBtn) zhBtn.classList.toggle('active', state.lang === 'zh');
      if (enBtn) enBtn.classList.toggle('active', state.lang === 'en');
    }

    function toggleLang() {
      state.lang = state.lang === 'zh' ? 'en' : 'zh';
      localStorage.setItem('lang', state.lang);
      applyI18n();
    }

    function setLang(lang) {
      if (state.lang === lang) return;
      state.lang = lang;
      localStorage.setItem('lang', state.lang);
      applyI18n();
    }

    applyI18n();

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
      document.querySelectorAll('.nav-item').forEach(n => n.classList.remove('active'));
      item.classList.add('active');
      document.querySelectorAll('.page').forEach(p => p.classList.remove('active'));
      document.getElementById('page-' + item.dataset.page).classList.add('active');
      if (item.dataset.page === 'status' && state.connected) {
        const activeTabBtn = document.querySelector('#page-status .tab-btn.active');
        if (activeTabBtn) activeTabBtn.click();
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
    async function toggleConnection() {
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
        state.model = '';
        state.chipVendor = '';
        state.currentBand = '';
        updateConnectionUI(false);
        updateDataConnectionUI();
        clearData();
        updateVlanPanelAccess();
        updateCellLockUI();
        addTerminalLine('[连接] 已断开', 'cmd');
      } else {
        // 连接
        $.connectBtn.textContent = '连接中...';
        $.connectBtn.disabled = true;
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
          // No AT port — go idle gracefully
          state.connected = false;
          state.idle = true;
          updateConnectionUI(false);
          $.statusLabel.textContent = '待机中';
          $.statusLabel.style.color = 'var(--text-muted)';
        }
      }
    }

    function updateConnectionUI(connected) {
      const connType = document.getElementById('connectionType');
      connType.disabled = connected;
      $.connectionParams.disabled = connected;
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
        setTextData('imei', noSim ? '--' : (s.imei || '--'));
        setTextData('iccid', noSim ? '--' : (s.iccid || '--'));
        setTextData('operator', noSim ? '--' : (s.operator || '--'));
        setTextData('networkType', noSim ? '--' : (s.networkType || '--'));
        setTextData('pci', noSim ? '--' : (s.pci || '--'));
        setTextData('cellid', noSim ? '--' : (s.cellId || '--'));
        setTextData('arfcn', noSim ? '--' : (s.arfcn || '--'));
        setTextData('band', noSim ? '--' : (s.band || '--'));
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
        const targetTab = cv === 'qualcomm' ? 'qualcomm' : cv === 'tdtech' ? 'tdtech' : 'unisoc';
        const targetBtn = document.getElementById(
          targetTab === 'qualcomm' ? 'glanTabQualcomm' : targetTab === 'tdtech' ? 'glanTabTdtech' : 'glanTabUnisoc'
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
      if (m.includes('MT5700')) return 'tdtech';
      if (m.includes('RG520') || m.includes('RM520') || m.includes('RG525') ||
          m.includes('RG530') || m.includes('RM530') ||
          m.includes('RG500Q') || m.includes('RM500Q') || m.includes('RM501Q') || m.includes('RM551'))
        return 'qualcomm';
      return 'unisoc';
    }

    function buildNetworkModeOptions(sel) {
      const zh = state.lang === 'zh';
      const family = getChipFamily(state.model);
      // Qualcomm / TdTech: AUTO WCDMA & LTE & 5G | NR5G 5G only | LTE LTE only
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
      const opts = family === 'unisoc' ? unisocOpts : qualcommOpts;
      sel.innerHTML = opts.map(o => `<option value="${o.value}">${zh ? o.zh : o.en}</option>`).join('');
    }

    // TdTech query_network_mode returns "NR5G-SA" for what it labels as general 5G (code "08").
    // For UniSoc, NR5G-SA is a real distinct option — do not normalize it.
    function normalizeNetworkMode(mode, family) {
      if (family === 'tdtech' && mode === 'NR5G-SA') return 'NR5G';
      return mode;
    }

    async function loadNetlockData() {
      if (!state.connected) return;
      showLoading('正在读取网络配置...');
      // Load preferred network mode
      try {
        const sel = document.getElementById('preferredNetwork');
        const family = getChipFamily(state.model);
        buildNetworkModeOptions(sel);
        const mode = await invoke('get_network_mode');
        sel.value = normalizeNetworkMode(mode, family);
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
        // Qualcomm: AT+QCFG="ims",<mode>,<enable>
        // UniSoc:  AT+QCFG="ims",<enable>
        const cmd = isQualcomm
          ? `AT+QCFG="ims",0,${val}`
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
          sel.value = normalizeNetworkMode(mode, getChipFamily(state.model));
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
        `<tr><td>${escapeHtml(r.pci || '--')}</td><td>${escapeHtml(r.rsrp || '--')}</td><td>${escapeHtml(r.rsrq || '--')}</td><td>${escapeHtml(r.earfcn || '--')}</td></tr>`
      ).join('');
    }

    function populateNrNeighbors(rows) {
      if (!rows || rows.length === 0) {
        document.getElementById('nrNeighborBody').innerHTML = '<tr><td colspan="5" style="color:var(--text-muted);text-align:center;padding:20px">暂无数据</td></tr>';
        return;
      }
      document.getElementById('nrNeighborBody').innerHTML = rows.map(r =>
        `<tr><td>${escapeHtml(r.pci || '--')}</td><td>${escapeHtml(r.rsrp || '--')}</td><td>${escapeHtml(r.rsrq || '--')}</td><td>${escapeHtml(r.sinr || '--')}</td><td>${escapeHtml(r.arfcn || '--')}</td></tr>`
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
      }
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
        // Qualcomm QMAP order: start,end,gw  (no mask field)
        // UniSoc QCFG order:  gw,mask,start,end
        const cmd = isQualcomm
          ? `AT+QMAP="LANIP",${start},${end},${gw}`
          : (mask
              ? `AT+QCFG="lanip_ex","${gw}","${mask}","${start}","${end}"`
              : `AT+QCFG="lanip_ex","${gw}","${start}","${end}"`);
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

    // ── 情景模式 ──
    const UNISOC_SCENES = [
      { id: 'router_eth_on',  name: '路由模式+以太网开启', nameEn: 'Router + Ethernet On',    req: { nat: 2, ethernet: true,  pcieMode: true  } },
      { id: 'router_eth_off', name: '路由模式+以太网关闭', nameEn: 'Router + Ethernet Off',   req: { nat: 2, ethernet: false } },
      { id: 'eth_bridge',     name: '以太网桥接模式',       nameEn: 'Ethernet Bridge',         req: { nat: 0, ethernet: true,  pcieMode: true  } },
      { id: 'usb_bridge',     name: 'USB桥接模式',          nameEn: 'USB Bridge',              req: { nat: 0, ethernet: false, pcieMode: false } },
      { id: 'usb_broadband',  name: 'USB移动宽带模式',      nameEn: 'USB Mobile Broadband',    req: { usbnet: 2 } },
    ];

    const SCENE_AT_LABELS = {
      nat:      (v) => v === 2 ? 'AT+QCFG="nat",2 (路由)' : 'AT+QCFG="nat",0 (桥接)',
      ethernet: (v) => v ? 'AT+QCFG="ethernet",1' : 'AT+QCFG="ethernet",0',
      pcieMode: (v) => v ? 'AT+QCFG="pcie/mode",1' : 'AT+QCFG="pcie/mode",0',
      usbnet:   (v) => v === 2 ? 'AT+QCFG="usbnet",2 (MBIM)' : `AT+QCFG="usbnet",${v}`,
    };

    let sceneCurrentState = null;

    async function loadScenePage() {
      const notConnected  = document.getElementById('sceneNotConnected');
      const loadingEl     = document.getElementById('sceneLoading');
      const unisocPanel   = document.getElementById('sceneUnisocPanel');
      const qualcommPanel = document.getElementById('sceneQualcommPanel');

      if (!state.connected) {
        notConnected.style.display  = '';
        loadingEl.style.display     = 'none';
        unisocPanel.style.display   = 'none';
        qualcommPanel.style.display = 'none';
        return;
      }
      notConnected.style.display = 'none';

      const isQualcomm = isQualcommModel(state.model);
      qualcommPanel.style.display = isQualcomm ? '' : 'none';
      if (isQualcomm) {
        loadingEl.style.display   = 'none';
        unisocPanel.style.display = 'none';
        return;
      }

      loadingEl.style.display   = '';
      unisocPanel.style.display = 'none';

      try {
        const [toggles, natMode, usbnetMode] = await Promise.all([
          invoke('get_feature_toggles'),
          invoke('get_nat_mode').catch(() => 0),
          invoke('get_usbnet_mode').catch(() => 3),
        ]);
        sceneCurrentState = {
          nat:      natMode,
          ethernet: toggles.ethernet,
          pcieMode: toggles.pcieMode,
          usbnet:   usbnetMode,
        };
        loadingEl.style.display   = 'none';
        unisocPanel.style.display = '';
        renderSceneCards(sceneCurrentState);
      } catch (e) {
        loadingEl.style.display = 'none';
        showToast('加载情景模式失败: ' + e, 'err');
      }
    }

    function renderSceneCards(currentState) {
      const grid = document.getElementById('sceneGrid');
      grid.innerHTML = '';
      UNISOC_SCENES.forEach(scene => {
        const isActive  = isSceneActive(scene, currentState);
        const card      = document.createElement('div');
        card.className  = 'scene-card' + (isActive ? ' scene-active' : '');

        const sceneName  = state.lang === 'en' ? scene.nameEn : scene.name;
        const badgeText  = isActive ? t('scene_active_badge') : t('scene_activate_btn');
        const badgeClass = isActive ? 'active' : 'inactive';
        const badgeClick = isActive ? '' : `onclick="activateScene('${scene.id}')"`;

        let atItems = '';
        for (const [key, expected] of Object.entries(scene.req)) {
          const matches = currentState[key] === expected;
          const symbol  = matches ? '<span class="check">✓</span>' : '<span class="cross">✗</span>';
          const labelFn = SCENE_AT_LABELS[key];
          const label   = labelFn ? labelFn(expected) : `${key}=${expected}`;
          atItems += `<li>${symbol}<span>${label}</span></li>`;
        }

        card.innerHTML = `
          <span class="scene-badge ${badgeClass}" ${badgeClick}>${badgeText}</span>
          <div class="scene-name">${sceneName}</div>
          <ul class="scene-at-list">${atItems}</ul>
        `;
        grid.appendChild(card);
      });
    }

    function isSceneActive(scene, curState) {
      for (const [key, expected] of Object.entries(scene.req)) {
        if (curState[key] !== expected) return false;
      }
      return true;
    }

    async function activateScene(sceneId) {
      if (!sceneCurrentState) return;
      const scene = UNISOC_SCENES.find(s => s.id === sceneId);
      if (!scene) return;

      const ops = [];
      const req = scene.req;
      if (req.nat      !== undefined && sceneCurrentState.nat      !== req.nat)
        ops.push(invoke('set_nat_mode', { mode: req.nat }));
      if (req.ethernet !== undefined && sceneCurrentState.ethernet !== req.ethernet)
        ops.push(invoke('set_feature_toggle', { feature: 'ethernet', enabled: req.ethernet }));
      if (req.pcieMode !== undefined && sceneCurrentState.pcieMode !== req.pcieMode)
        ops.push(invoke('set_feature_toggle', { feature: 'pcieMode', enabled: req.pcieMode }));
      if (req.usbnet   !== undefined && sceneCurrentState.usbnet   !== req.usbnet)
        ops.push(invoke('set_usbnet_mode', { mode: req.usbnet }));

      if (ops.length === 0) return;

      try {
        await Promise.all(ops);
        document.getElementById('sceneRebootNotice').style.display = '';
        await loadScenePage();
      } catch (e) {
        showToast('激活失败: ' + e, 'err');
      }
    }

    function dismissSceneReboot() {
      document.getElementById('sceneRebootNotice').style.display = 'none';
    }

    // ── AT手册速查 ──
    const AT_DB = {
      unisoc: [
        { cmd:'AT', category:'通用指令', desc:'测试 AT 串口通信是否正常',
          syntax:'AT',
          response:'OK',
          params:[],
          example:'AT\nOK', note:'最基本的 AT 指令，用于确认模组串口连接正常、模组已开机。若无响应请检查波特率、串口号、接线。' },
        { cmd:'ATE', category:'通用指令', desc:'设置 AT 命令回显开关',
          syntax:'ATE<value>',
          response:'OK',
          params:[
            {name:'value',desc:'回显开关',values:'0=关闭回显, 1=开启回显'}],
          example:'ATE1\nOK', note:'关闭回显后发送的指令不会原样返回，可减少串口数据量，便于程序解析响应。' },
        { cmd:'AT+GMR', category:'通用指令', desc:'查询模组固件版本号',
          syntax:'AT+GMR',
          response:'<firmware_version>\nOK',
          params:[],
          example:'AT+GMR\nRG500U_EU_5G_SA01A06V01\nOK', note:'返回完整的固件版本字符串，包含平台、区域、SA/NSA、版本号等信息。升级固件前后务必确认版本号。' },
        { cmd:'AT+CGMR', category:'通用指令', desc:'查询固件版本（3GPP 标准格式，等效 AT+GMR）',
          syntax:'AT+CGMR',
          response:'<firmware_version>\nOK',
          params:[],
          example:'AT+CGMR\nRG500U_EU_5G_SA01A06V01\nOK', note:'3GPP 标准指令，返回内容与 AT+GMR 相同。部分第三方工具仅识别此格式。' },
        { cmd:'AT+CGSN', category:'设备信息', desc:'查询模组 IMEI（国际移动设备识别码）',
          syntax:'AT+CGSN',
          response:'<imei>\nOK',
          params:[],
          example:'AT+CGSN\n861234567890123\nOK', note:'IMEI 为 15 位数字，是全球唯一的模组标识。网络注册、运营商白名单均依赖 IMEI。' },
        { cmd:'AT+CCID', category:'设备信息', desc:'查询 SIM 卡 ICCID（展锐平台首选）',
          syntax:'AT+CCID',
          response:'+CCID: <iccid>\nOK',
          params:[],
          example:'AT+CCID\n+CCID: 89860318640012345678\nOK', note:'ICCID 为 SIM 卡唯一识别号（20 位）。展锐平台响应前缀为 +CCID:，高通平台使用 AT+ICCID（前缀 +ICCID:），注意区分。' },
        { cmd:'AT+CIMI', category:'设备信息', desc:'查询 SIM 卡 IMSI（国际移动用户识别码）',
          syntax:'AT+CIMI',
          response:'<imsi>\nOK',
          params:[],
          example:'AT+CIMI\n460001234567890\nOK', note:'IMSI 由 MCC（国家码）+ MNC（网络码）+ MSIN（用户号）组成，用于网络侧识别用户身份。SIM 未插入时返回 ERROR。' },
        { cmd:'AT+CGMM', category:'设备信息', desc:'查询模组型号（用于厂商/芯片平台检测）',
          syntax:'AT+CGMM',
          response:'<model>\nOK',
          params:[],
          example:'AT+CGMM\nRM500U\nOK', note:'返回值用于判断芯片平台：RM500U/RG200U 为展锐平台，RM520N/RM500Q 为高通平台。本工具据此自动切换 AT 指令集。' },
        { cmd:'AT+CGMI', category:'设备信息', desc:'查询模组制造商名称',
          syntax:'AT+CGMI',
          response:'Quectel\nOK',
          params:[],
          example:'AT+CGMI\nQuectel\nOK', note:'通常返回 "Quectel"。用于确认模组品牌。' },
        { cmd:'AT+CCLK', category:'设备信息', desc:'查询或设置模组实时时钟',
          syntax:'AT+CCLK?\nAT+CCLK="<time>"',
          response:'+CCLK: "<yy/MM/dd,hh:mm:ss±tz>"\nOK',
          params:[
            {name:'time',desc:'日期时间',values:'格式 yy/MM/dd,hh:mm:ss±tz，tz 为时区偏移（单位：15分钟）'}],
          example:'AT+CCLK?\n+CCLK: "25/05/26,14:30:00+32"\nOK', note:'时区偏移 tz：+32 表示 UTC+8（32×15min=480min=8h）。设置时钟需在 SIM 卡就绪后操作。' },
        { cmd:'AT+QBASELINE', category:'设备信息', desc:'查询 AP/CP 基线版本号',
          syntax:'AT+QBASELINE',
          response:'+QBASELINE: <ap_version>,<cp_version>\nOK',
          params:[],
          example:'AT+QBASELINE\n+QBASELINE: MDM9x07_ES2.0_SEC5G_SVN_SEC5G_21.141.21.01,SR510M_BD_R01_V02_220525\nOK', note:'AP 为应用处理器版本，CP 为通信处理器版本。排查固件兼容性问题时需同时提供两者。' },
        { cmd:'AT+QTEMP', category:'设备信息', desc:'查询 SoC 芯片温度及 PA 功放温度',
          syntax:'AT+QTEMP',
          response:'+QTEMP: <soc_temp>,<pa_temp>\nOK',
          params:[
            {name:'soc_temp',desc:'SoC 温度',values:'摄氏度整数，正常 30~60°C'},
            {name:'pa_temp',desc:'PA 温度',values:'摄氏度整数，正常 30~70°C'}],
          example:'AT+QTEMP\n+QTEMP: 42,38\nOK', note:'温度超过 85°C 可能触发降频或关断保护。长时间高负载（持续速率测试）时应关注温度。' },
        { cmd:'AT+CPIN?', category:'SIM与注册', desc:'查询 SIM 卡当前状态',
          syntax:'AT+CPIN?',
          response:'+CPIN: <status>\nOK',
          params:[
            {name:'status',desc:'SIM 状态',values:'READY=正常 | SIM PIN=需要 PIN | SIM PUK=需要 PUK | NOT INSERTED=未插入'}],
          example:'AT+CPIN?\n+CPIN: READY\nOK', note:'SIM PIN/PUK 状态下需先解锁才能执行网络操作。连续输错 PIN（3次）将锁卡，需 PUK 解锁。' },
        { cmd:'AT+CEREG?', category:'SIM与注册', desc:'查询 LTE/5G EPS 网络注册状态',
          syntax:'AT+CEREG?',
          response:'+CEREG: <n>,<stat>[,<tac>,<ci>,<acst>]\nOK',
          params:[
            {name:'stat',desc:'注册状态',values:'0=未注册, 1=已注册(本地), 2=搜索中, 3=注册被拒, 5=已注册(漫游)'}],
          example:'AT+CEREG?\n+CEREG: 0,1,"0B01","1A000101",7\nOK', note:'5G/LTE 核心注册状态。stat=3（被拒）通常是 SIM 卡未开通对应业务或频段不匹配，需检查 SIM 套餐和网络覆盖。' },
        { cmd:'AT+CREG?', category:'SIM与注册', desc:'查询 GSM/WCDMA 网络注册状态',
          syntax:'AT+CREG?',
          response:'+CREG: <n>,<stat>\nOK',
          params:[
            {name:'stat',desc:'注册状态',values:'0=未注册, 1=已注册(本地), 2=搜索中, 3=被拒, 5=已注册(漫游)'}],
          example:'AT+CREG?\n+CREG: 0,1\nOK', note:'用于 2G/3G 网络注册诊断。5G 模组驻留 4G/5G 时此值通常为 0 或 5。' },
        { cmd:'AT+CGREG?', category:'SIM与注册', desc:'查询 GPRS 分组域注册状态',
          syntax:'AT+CGREG?',
          response:'+CGREG: <n>,<stat>\nOK',
          params:[
            {name:'stat',desc:'注册状态',values:'0=未注册, 1=已注册(本地), 2=搜索中, 3=被拒, 5=已注册(漫游)'}],
          example:'AT+CGREG?\n+CGREG: 0,1\nOK', note:'GPRS 数据业务注册状态。与 CEREG 互补：CEREG 对应 LTE/5G，CGREG 对应 2G/3G 分组域。' },
        { cmd:'AT+COPS?', category:'SIM与注册', desc:'查询当前运营商名称和接入技术',
          syntax:'AT+COPS?',
          response:'+COPS: <mode>[,<format>,<oper>[,<AcT>]]\nOK',
          params:[
            {name:'mode',desc:'选网模式',values:'0=自动选网, 1=手动选网, 4=关闭选网'},
            {name:'format',desc:'名称格式',values:'0=长名称, 1=短名称, 2=数字(MCC+MNC)'},
            {name:'AcT',desc:'接入技术',values:'7=LTE, 9=NR5G, 12=NR5G-SA'}],
          example:'AT+COPS?\n+COPS: 0,0,"CHINA MOBILE",9\nOK', note:'用于确认当前驻留的运营商和网络制式。若返回空值表示尚未驻网。' },
        { cmd:'AT+CGATT?', category:'SIM与注册', desc:'查询 GPRS 附着状态',
          syntax:'AT+CGATT?',
          response:'+CGATT: <state>\nOK',
          params:[
            {name:'state',desc:'附着状态',values:'0=未附着, 1=已附着'}],
          example:'AT+CGATT?\n+CGATT: 1\nOK', note:'GPRS 附着是数据通信的前提。state=0 时无法建立数据连接，需检查 SIM 套餐和注册状态。' },
        { cmd:'AT+QENG="servingcell"', category:'网络与信号', desc:'查询当前服务小区详细信息（频段、PCI、ARFCN、信号强度等）',
          syntax:'AT+QENG="servingcell"',
          response:'+QENG: "servingcell","CONNECT","NR5G-SA","SA",<MCC>,<MNC>,<CellID>,<PCI>,<ARFCN>,<BW>,<RSRP>,<RSRQ>,<SINR>,...\nOK',
          params:[
            {name:'state',desc:'连接状态',values:'CONNECT=已连接 | SEARCH=搜索中 | LIMSRV=受限服务'},
            {name:'tech',desc:'接入技术',values:'NR5G-SA | NR5G-NSA | LTE'},
            {name:'BW',desc:'带宽',values:'展锐为直接 MHz 值（如 100）'},
            {name:'RSRP',desc:'参考信号接收功率',values:'dBm 负值，-80以上优秀，-100以下较差'},
            {name:'RSRQ',desc:'参考信号接收质量',values:'dB 负值，-10以上优秀'},
            {name:'SINR',desc:'信干噪比',values:'dB，正值越大越好'}],
          example:'AT+QENG="servingcell"\n+QENG: "servingcell","CONNECT","NR5G-SA","SA",460,00,1A000101,123,630000,100,-85,-8,22\nOK', note:'展锐平台带宽字段为直接 MHz 值（如 100），与高通索引值（0-12）不同。这是最核心的网络诊断指令。' },
        { cmd:'AT+QENG="neighbourcell"', category:'网络与信号', desc:'查询邻近小区列表（用于网络优化分析）',
          syntax:'AT+QENG="neighbourcell"',
          response:'+QENG: "neighbourcell intra",<tech>,<pci>,<arfcn>,<rsrp>,...\n+QENG: "neighbourcell inter",<tech>,...\nOK',
          params:[
            {name:'intra',desc:'同频邻区',values:'与当前服务小区同频段的邻近小区'},
            {name:'inter',desc:'异频邻区',values:'与当前服务小区不同频段的邻近小区'}],
          example:'AT+QENG="neighbourcell"\n+QENG: "neighbourcell intra",...\nOK', note:'返回同频(intra)和异频(inter)邻区。邻区信息可用于判断是否应切换频段或小区。' },
        { cmd:'AT+CSQ', category:'网络与信号', desc:'查询信号强度 RSSI 和误码率 BER',
          syntax:'AT+CSQ',
          response:'+CSQ: <rssi>,<ber>\nOK',
          params:[
            {name:'rssi',desc:'信号强度',values:'0-31（越大越强），99=未知或未检测'},
            {name:'ber',desc:'误码率',values:'0-7（越大越差），99=未知'}],
          example:'AT+CSQ\n+CSQ: 22,0\nOK', note:'RSSI 取值 0~31 约对应 -113dBm ~ -51dBm，99 表示未知。这是最简单的信号查询，建议配合 QENG 获取更精确的 RSRP/RSRQ。' },
        { cmd:'AT+QANTRSSI?', category:'网络与信号', desc:'查询各路天线 RSSI（展锐 4 天线专用）',
          syntax:'AT+QANTRSSI?',
          response:'+QANTRSSI: <ant0>,<ant1>,<ant2>,<ant3>\nOK',
          params:[
            {name:'ant0~ant3',desc:'各天线 RSSI',values:'dBm 负值，4路天线分别显示'}],
          example:'AT+QANTRSSI?\n+QANTRSSI: -85,-87,-90,-92\nOK', note:'展锐平台 4x4 MIMO 天线诊断。若某路天线值明显偏低（如 -120），可能天线未接或馈线故障。高通平台使用 AT+QRSRP。' },
        { cmd:'AT+QSNR', category:'网络与信号', desc:'查询 5G 信噪比',
          syntax:'AT+QSNR',
          response:'+QSNR: <snr>\nOK',
          params:[
            {name:'snr',desc:'信噪比',values:'0-100，越大越好，>20 为优秀'}],
          example:'AT+QSNR\n+QSNR: 35\nOK', note:'展锐平台专用 5G SNR 查询。SNR>20 表示信号质量良好，<10 可能导致速率严重下降。' },
        { cmd:'AT+C5GQOSRDP=', category:'网络与信号', desc:'查询指定 CID 的 5G QoS 参数（5QI、上下行带宽等）',
          syntax:'AT+C5GQOSRDP=<cid>',
          response:'+C5GQOSRDP: <cid>,<5QI>,<ul_bw>,<dl_bw>,...\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'5QI',desc:'5G QoS 标识符',values:'整数，如 9=默认承载'},
            {name:'ul_bw',desc:'上行保证带宽',values:'kbps'},
            {name:'dl_bw',desc:'下行保证带宽',values:'kbps'}],
          example:'AT+C5GQOSRDP=1\n+C5GQOSRDP: 1,9,100000,200000,...\nOK', note:'5QI 值反映运营商分配的 QoS 等级。需先建立数据连接才能查询。' },
        { cmd:'AT+QNWPREFCFG=?', category:'网络与信号', desc:'查询模组硬件支持的 LTE 和 NR 频段列表',
          syntax:'AT+QNWPREFCFG=?',
          response:'+QNWPREFCFG: "lte_band",<supported_bands>\n+QNWPREFCFG: "nr5g_band",<supported_bands>\nOK',
          params:[],
          example:'AT+QNWPREFCFG=?\n+QNWPREFCFG: "lte_band",...\nOK', note:'返回模组射频硬件实际支持的频段，用于确认是否支持目标运营商的频段。设置频段时不应超出此范围。' },
        { cmd:'AT+QNETDEVCTL=', category:'数据连接', desc:'连接/断开数据（展锐平台专用）',
          syntax:'AT+QNETDEVCTL=<action>,<cid>,<flag>',
          response:'OK',
          params:[
            {name:'action',desc:'操作',values:'1=连接, 0=断开'},
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'flag',desc:'标志',values:'通常为 1'}],
          example:'AT+QNETDEVCTL=1,1,1\nOK', note:'展锐平台数据拨号指令。高通平台使用 AT+QMAP="connect" 代替。连接前需确保 PDP 上下文已配置（QICSGP/CGDCONT）。' },
        { cmd:'AT+QNETDEVSTATUS=', category:'数据连接', desc:'查询 IP 地址、子网掩码、网关、DNS（展锐平台专用）',
          syntax:'AT+QNETDEVSTATUS=<cid>',
          response:'<ipv4>,<mask>,<gw>,,<dns1>,<dns2>,<ipv6>,,,,<v6dns1>,<v6dns2>\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'}],
          example:'AT+QNETDEVSTATUS=1\n10.100.50.1,255.255.255.240,10.100.50.14,,114.114.114.114,8.8.8.8,...\nOK', note:'响应无前缀标签，字段按逗号分隔的位置解析。[0]=IPv4,[1]=掩码,[2]=网关,[4]=DNS1,[5]=DNS2。' },
        { cmd:'AT+CGACT', category:'数据连接', desc:'激活或去激活 PDP 上下文',
          syntax:'AT+CGACT=<state>,<cid>\nAT+CGACT?',
          response:'+CGACT: <cid>,<state>\nOK',
          params:[
            {name:'state',desc:'激活状态',values:'0=去激活, 1=激活'},
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'}],
          example:'AT+CGACT=1,1\nOK', note:'CGACT 是 3GPP 标准的 PDP 激活指令。展锐平台实际拨号使用 QNETDEVCTL，高通使用 QMAP。查询格式 AT+CGACT? 返回所有上下文状态。' },
        { cmd:'AT+QICSGP=', category:'数据连接', desc:'配置 PDP 上下文的 APN、协议类型和认证信息',
          syntax:'AT+QICSGP=<cid>,<context_type>,"<apn>","<username>","<password>",<auth_type>\nAT+QICSGP=<cid>',
          response:'+QICSGP: <cid>,<context_type>,"<apn>","<username>","<password>",<auth_type>\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'context_type',desc:'PDP 协议类型',values:'1=IPv4, 2=IPv6, 3=IPv4v6'},
            {name:'apn',desc:'APN 接入点名称',values:'字符串，如 "cmnet"(移动)/"3gnet"(联通)/"ctnet"(电信)'},
            {name:'username',desc:'认证用户名',values:'通常为空字符串'},
            {name:'password',desc:'认证密码',values:'通常为空字符串'},
            {name:'auth_type',desc:'认证方式',values:'0=无认证, 1=PAP, 2=CHAP, 3=PAP或CHAP'}],
          example:'AT+QICSGP=1,1,"cmnet","","",0\nOK', note:'查询时只传 cid。国内运营商 APN 通常无需认证（auth_type=0）。APN 配置错误是最常见的无法上网原因。' },
        { cmd:'AT+CGDCONT=', category:'数据连接', desc:'定义或删除 PDP 上下文',
          syntax:'AT+CGDCONT=<cid>[,<PDP_type>[,"<apn>"]]\nAT+CGDCONT=<cid>（删除）',
          response:'OK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'PDP_type',desc:'PDP 类型',values:'"IP" | "IPV6" | "IPV4V6"'},
            {name:'apn',desc:'APN 名称',values:'字符串'}],
          example:'AT+CGDCONT=2,"IP","internet"\nOK', note:'只传 cid（如 AT+CGDCONT=2）时删除该上下文。通常 cid=1 为主上下文。与 QICSGP 配合使用：先 CGDCONT 定义类型，再 QICSGP 配置 APN。' },
        { cmd:'AT+CGDCONT?', category:'数据连接', desc:'查询所有已定义的 PDP 上下文配置',
          syntax:'AT+CGDCONT?',
          response:'+CGDCONT: <cid>,<PDP_type>,<apn>,<ip_addr>,...\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'PDP_type',desc:'协议类型',values:'IP | IPV6 | IPV4V6'},
            {name:'apn',desc:'APN',values:'字符串'},
            {name:'ip_addr',desc:'已分配 IP',values:'如 10.x.x.x，未激活时为空'}],
          example:'AT+CGDCONT?\n+CGDCONT: 1,"IP","cmnet",10.100.50.1,...\nOK', note:'可查看所有 PDP 上下文的配置和当前 IP。多 APN 场景下每个 cid 对应不同的数据连接。' },
        { cmd:'AT+CGPADDR', category:'数据连接', desc:'查询 PDP 上下文已分配的 IP 地址',
          syntax:'AT+CGPADDR=<cid>\nAT+CGPADDR',
          response:'+CGPADDR: <cid>,<ipv4>[,<ipv6>]\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16，省略则查询全部'}],
          example:'AT+CGPADDR\n+CGPADDR: 1,10.100.50.1\nOK', note:'不传参数时查询所有已激活的 PDP 地址。IP 为 0.0.0.0 表示上下文已定义但未激活。' },
        { cmd:'AT+QGDCNT?', category:'流量统计', desc:'查询上下行流量累计字节数（展锐平台）',
          syntax:'AT+QGDCNT?',
          response:'+QGDCNT: <tx_bytes>,<rx_bytes>\nOK',
          params:[
            {name:'tx_bytes',desc:'上行（发送）流量',values:'字节数，累计值'},
            {name:'rx_bytes',desc:'下行（接收）流量',values:'字节数，累计值'}],
          example:'AT+QGDCNT?\n+QGDCNT: 1024000,5120000\nOK', note:'高通平台使用 AT+QGDNRCNT?，两者命令名不同，不可互换。数值为累计值，断电不清除，需手动重置。' },
        { cmd:'AT+QGDCNT=0', category:'流量统计', desc:'重置流量计数器归零',
          syntax:'AT+QGDCNT=0',
          response:'OK',
          params:[],
          example:'AT+QGDCNT=0\nOK', note:'执行后 tx/rx 字节数归零。通常在开始新的流量测试前重置。' },
        { cmd:'AT+QNWPREFCFG="lte_band"', category:'频段配置', desc:'查询或设置 LTE 频段锁定',
          syntax:'AT+QNWPREFCFG="lte_band"\nAT+QNWPREFCFG="lte_band","<bands>"',
          response:'+QNWPREFCFG: "lte_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'LTE 频段列表',values:'纯数字冒号分隔，如 1:3:8:41（不带引号/B 前缀）。设为空字符串恢复默认'}],
          example:'AT+QNWPREFCFG="lte_band",1:3:8:41\nOK', note:'锁定频段后模组仅扫描指定频段，可加快驻网速度但可能遗漏可用频段。设为空字符串恢复搜索所有支持频段。' },
        { cmd:'AT+QNWPREFCFG="nr5g_band"', category:'频段配置', desc:'查询或设置 5G NR 频段锁定',
          syntax:'AT+QNWPREFCFG="nr5g_band"\nAT+QNWPREFCFG="nr5g_band","<bands>"',
          response:'+QNWPREFCFG: "nr5g_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'NR 频段列表',values:'冒号分隔，如 "n41:n78:n79"。中国 5G 主要使用 n78/n79'}],
          example:'AT+QNWPREFCFG="nr5g_band",78:79\nOK', note:'中国电信/联通主要 n78，移动主要 n41/n79。只保留本运营商频段可显著加快 5G 驻网。' },
        { cmd:'AT+QNWPREFCFG="mode_pref"', category:'频段配置', desc:'查询或设置首选网络模式',
          syntax:'AT+QNWPREFCFG="mode_pref"\nAT+QNWPREFCFG="mode_pref",<mode>',
          response:'+QNWPREFCFG: "mode_pref",<mode>\nOK',
          params:[
            {name:'mode',desc:'网络模式',values:'AUTO=自动 | LTE=仅4G | NR5G=仅5G | LTE:NR5G=4G+5G'}],
          example:'AT+QNWPREFCFG="mode_pref","NR5G"\nOK', note:'设为 NR5G 可强制仅驻留 5G，用于 5G 覆盖测试。设为 AUTO 让模组自动选择最优制式。' },
        { cmd:'AT+QNWPREFCFG="all_band_reset"', category:'频段配置', desc:'重置所有频段锁定为出厂默认值',
          syntax:'AT+QNWPREFCFG="all_band_reset"',
          response:'OK',
          params:[],
          example:'AT+QNWPREFCFG="all_band_reset"\nOK', note:'清除所有 LTE 和 NR 频段锁定，恢复为搜索全部支持频段。修改频段后若无法驻网可执行此命令重置。' },
        { cmd:'AT+QCFG="ethernet"', category:'功能开关', desc:'查询或设置以太网接口启用状态',
          syntax:'AT+QCFG="ethernet"\nAT+QCFG="ethernet",<state>',
          response:'+QCFG: "ethernet",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="ethernet",1\nOK', note:'开启以太网功能的前提是 pcie/mode 已开启。通常用于有线 LAN 口输出。' },
        { cmd:'AT+QCFG="pcie/mode"', category:'功能开关', desc:'查询或设置 PCIe 接口模式（以太网功能依赖此项）',
          syntax:'AT+QCFG="pcie/mode"\nAT+QCFG="pcie/mode",<mode>',
          response:'+QCFG: "pcie/mode",<mode>\nOK',
          params:[
            {name:'mode',desc:'PCIe 模式',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="pcie/mode",1\nOK', note:'以太网功能的前置条件。开启后模组通过 PCIe 接口提供以太网数据通道。' },
        { cmd:'AT+QCFG="napt"', category:'功能开关', desc:'查询或设置 NAPT 网络地址端口转换功能',
          syntax:'AT+QCFG="napt"\nAT+QCFG="napt",<state>',
          response:'+QCFG: "napt",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="napt",1\nOK', note:'NAPT（Network Address Port Translation）允许多个内网设备共享一个公网 IP。路由模式下通常需开启。' },
        { cmd:'AT+QCFG="nat"', category:'功能开关', desc:'查询或设置 NAT 工作模式（桥接或路由）',
          syntax:'AT+QCFG="nat"\nAT+QCFG="nat",<mode>',
          response:'+QCFG: "nat",<mode>\nOK',
          params:[
            {name:'mode',desc:'NAT 模式',values:'0=桥接模式（透传）, 2=路由模式（NAT）'}],
          example:'AT+QCFG="nat",2\nOK', note:'桥接模式(0)下模组作为透明桥，路由模式(2)下模组作为路由器进行 NAT 转换。与 napt 不同：napt 控制是否启用 NAPT 功能，nat 控制桥接/路由模式。' },
        { cmd:'AT+QCFG="proxyarp"', category:'功能开关', desc:'查询或设置 Proxy ARP 代理功能',
          syntax:'AT+QCFG="proxyarp"\nAT+QCFG="proxyarp",<state>',
          response:'+QCFG: "proxyarp",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="proxyarp",1\nOK', note:'Proxy ARP 让模组代理响应内网设备的 ARP 请求，用于桥接模式下内网设备可达性。' },
        { cmd:'AT+QCFG="uartat"', category:'功能开关', desc:'查询或设置 UART 串口 AT 命令接口',
          syntax:'AT+QCFG="uartat"\nAT+QCFG="uartat",<state>',
          response:'+QCFG: "uartat",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭串口AT, 1=开启串口AT'}],
          example:'AT+QCFG="uartat",1\nOK', note:'关闭后无法通过串口发送 AT 指令。生产环境可关闭以释放串口资源。' },
        { cmd:'AT+QCFG="usbnet"', category:'功能开关', desc:'查询或设置 USB 网卡工作模式',
          syntax:'AT+QCFG="usbnet"\nAT+QCFG="usbnet",<mode>',
          response:'+QCFG: "usbnet",<mode>\nOK',
          params:[
            {name:'mode',desc:'USB 网卡模式',values:'2=MBIM（移动宽带，Windows 原生支持）, 3=RNDIS/ECM（通用虚拟网卡）'}],
          example:'AT+QCFG="usbnet",2\nOK', note:'MBIM(2) 适用于 Windows 即插即用；RNDIS(3) 兼容性更广（Linux/旧 Windows）。切换后需重启模组。' },
        { cmd:'AT+QCFG="usbcfg"', category:'功能开关', desc:'查询或设置 USB 功能配置（含 ADB 调试开关）',
          syntax:'AT+QCFG="usbcfg"\nAT+QCFG="usbcfg",<vid>,<pid>,...,<adb>,<flag>',
          response:'+QCFG: "usbcfg",<vid>,<pid>,...\nOK',
          params:[
            {name:'adb',desc:'ADB 调试开关',values:'倒数第二个字段：0=关闭 ADB, 1=开启 ADB'},
            {name:'vid/pid',desc:'USB 厂商/产品 ID',values:'通常保持不变'}],
          example:'AT+QCFG="usbcfg"\n+QCFG: "usbcfg",0x2C7C,0x0801,...\nOK', note:'修改 ADB 开关时：先查询完整值，仅修改倒数第二个字段（0→1），其余字段保持不变。ADB 开启后可通过 adb shell 访问模组 Android 系统。' },
        { cmd:'AT+QCFG="lanip_ex"', category:'功能开关', desc:'查询或设置 LAN 网关 IP 及 DHCP 地址池',
          syntax:'AT+QCFG="lanip_ex"\nAT+QCFG="lanip_ex","<gw>","<start>","<end>"',
          response:'+QCFG: "lanip_ex","<gw>","<start>","<end>"\nOK',
          params:[
            {name:'gw',desc:'网关 IP',values:'如 "192.168.8.1"'},
            {name:'start',desc:'地址池起始 IP',values:'如 "192.168.8.2"'},
            {name:'end',desc:'地址池结束 IP',values:'如 "192.168.8.254"'}],
          example:'AT+QCFG="lanip_ex"\n+QCFG: "lanip_ex","192.168.8.1","192.168.8.2","192.168.8.254"\nOK', note:'展锐平台可能支持 4 字段（含子网掩码）。修改后需重启生效，且需确保不与其他网段冲突。' },
        { cmd:'AT+QCFG="5glan"', category:'功能开关', desc:'查询或设置 5GLAN（5G 局域网）功能',
          syntax:'AT+QCFG="5glan"\nAT+QCFG="5glan",<cid>,<state>,1',
          response:'+QCFG: "5glan",<cid>,<state>\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'state',desc:'启用状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="5glan",1,1,1\nOK', note:'5GLAN 允许通过 5G 网络建立局域网，用于工业物联网场景。需先建立 PDP 上下文。' },
        { cmd:'AT+QCFG="netmask"', category:'功能开关', desc:'查询或设置子网掩码模式',
          syntax:'AT+QCFG="netmask"\nAT+QCFG="netmask",<mode>',
          response:'+QCFG: "netmask",<mode>\nOK',
          params:[
            {name:'mode',desc:'掩码模式',values:'0=标准（/24 子网）, 1=扩展（更小子网）'}],
          example:'AT+QCFG="netmask",0\nOK', note:'标准模式使用 255.255.255.0（/24），扩展模式允许更精细的子网划分。' },
        { cmd:'AT+QDMZ', category:'功能开关', desc:'查询或设置 DMZ 主机（全端口转发）',
          syntax:'AT+QDMZ?\nAT+QDMZ=<status>[,<IP_version>[,<IP_address>]]',
          response:'+QDMZ: <status>,4[,<IP_address>]\n+QDMZ: <status>,6[,<IP_address>]\nOK',
          params:[
            {name:'status',desc:'DMZ 开关',values:'0=关闭, 1=开启'},
            {name:'IP_version',desc:'IP 版本',values:'4=IPv4, 6=IPv6'},
            {name:'IP_address',desc:'DMZ 主机 IP',values:'内网 IP 如 192.168.8.100'}],
          example:'AT+QDMZ?\n+QDMZ: 0,4\n+QDMZ: 0,6\nOK\nAT+QDMZ=1,4,192.168.8.100\nOK', note:'展锐平台 DMZ 指令。开启后所有未映射入站流量转发到指定 IP。修改 DMZ 地址前需先关闭。' },
        { cmd:'AT+QNWPREFCFG="srv_domain"', category:'网络与信号', desc:'查询或设置服务域（CS 语音 / PS 数据 / 混合）',
          syntax:'AT+QNWPREFCFG="srv_domain"\nAT+QNWPREFCFG="srv_domain",<domain>',
          response:'+QNWPREFCFG: "srv_domain",<domain>\nOK',
          params:[
            {name:'domain',desc:'服务域',values:'1=仅 CS（语音）, 2=仅 PS（数据）, 3=CS+PS（混合）'}],
          example:'AT+QNWPREFCFG="srv_domain"\n+QNWPREFCFG: "srv_domain",2\nOK', note:'纯数据模组通常设为 2（仅 PS）。设为 3 可支持 VoLTE 语音（需运营商和 SIM 支持）。' },
        { cmd:'AT+QNWPREFCFG="roam_pref"', category:'网络与信号', desc:'查询或设置漫游偏好',
          syntax:'AT+QNWPREFCFG="roam_pref"\nAT+QNWPREFCFG="roam_pref",<pref>',
          response:'+QNWPREFCFG: "roam_pref",<pref>\nOK',
          params:[
            {name:'pref',desc:'漫游偏好',values:'0=不漫游（仅本网）, 1=允许漫游, 2=拒绝漫游'}],
          example:'AT+QNWPREFCFG="roam_pref"\n+QNWPREFCFG: "roam_pref",0\nOK', note:'跨运营商漫游场景需设为 1。0 和 2 均不允许漫游。' },
        { cmd:'AT+QNWPREFCFG="voice_domain"', category:'网络与信号', desc:'查询或设置语音域偏好',
          syntax:'AT+QNWPREFCFG="voice_domain"\nAT+QNWPREFCFG="voice_domain",<domain>',
          response:'+QNWPREFCFG: "voice_domain",<domain>\nOK',
          params:[
            {name:'domain',desc:'语音域',values:'0=CS voice only, 1=IMS voice over PS preferred, 2=CS voice preferred, 3=IMS voice over PS only'}],
          example:'AT+QNWPREFCFG="voice_domain"\n+QNWPREFCFG: "voice_domain",1\nOK', note:'设为 1 优先使用 IMS（VoLTE/VoNR），需运营商支持。设为 0 仅 CS 语音。' },
        { cmd:'AT+QNWPREFCFG="gw_band"', category:'频段配置', desc:'查询或设置 WCDMA (3G) 频段',
          syntax:'AT+QNWPREFCFG="gw_band"\nAT+QNWPREFCFG="gw_band",<bands>',
          response:'+QNWPREFCFG: "gw_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'WCDMA 频段',values:'冒号分隔数字，如 1:2:5:8'}],
          example:'AT+QNWPREFCFG="gw_band"\n+QNWPREFCFG: "gw_band",1:8\nOK', note:'WCDMA 频段锁定。5G 模组较少使用 3G，通常保持默认。' },
        { cmd:'AT+QNWPREFCFG="ue_usage_setting"', category:'网络与信号', desc:'查询或设置 UE 使用场景',
          syntax:'AT+QNWPREFCFG="ue_usage_setting"\nAT+QNWPREFCFG="ue_usage_setting",<setting>',
          response:'+QNWPREFCFG: "ue_usage_setting",<setting>\nOK',
          params:[
            {name:'setting',desc:'使用场景',values:'0=通用, 1=语音优先, 2=数据优先'}],
          example:'AT+QNWPREFCFG="ue_usage_setting"\n+QNWPREFCFG: "ue_usage_setting",0\nOK', note:'影响网络侧资源分配策略。数据模组建议设为 2（数据优先）。' },
        { cmd:'AT+QCFG="autoapn"', category:'功能开关', desc:'查询或设置 APN 自动获取功能',
          syntax:'AT+QCFG="autoapn"\nAT+QCFG="autoapn",<enable>',
          response:'+QCFG: "autoapn",<enable>\nOK',
          params:[
            {name:'enable',desc:'自动 APN',values:'0=关闭（手动配置 APN）, 1=开启（自动从网络获取 APN）'}],
          example:'AT+QCFG="autoapn"\n+QCFG: "autoapn",1\nOK', note:'开启后模组自动从网络侧获取 APN，无需手动配置。部分运营商不支持自动获取，需手动设为 0 并通过 QICSGP 配置。' },
        { cmd:'AT+CFUN?', category:'系统控制', desc:'查询当前射频功能状态',
          syntax:'AT+CFUN?',
          response:'+CFUN: <fun>\nOK',
          params:[
            {name:'fun',desc:'功能等级',values:'0=最小功能（射频关）, 1=全功能（射频开）, 4=飞行模式'}],
          example:'AT+CFUN?\n+CFUN: 1\nOK', note:'fun=0 时射频关闭但 AT 指令仍可用；fun=4 飞行模式关闭射频但保留 SIM 检测。' },
        { cmd:'AT+CFUN=1', category:'系统控制', desc:'开启射频（全功能模式）',
          syntax:'AT+CFUN=1',
          response:'OK',
          params:[],
          example:'AT+CFUN=1\nOK', note:'从最小功能模式或飞行模式恢复为全功能模式，射频开启后开始搜网注册。' },
        { cmd:'AT+CFUN=0', category:'系统控制', desc:'关闭射频（最小功能模式）',
          syntax:'AT+CFUN=0',
          response:'OK',
          params:[],
          example:'AT+CFUN=0\nOK', note:'关闭射频后模组停止搜网注册，但 AT 指令仍可用。用于省电或屏蔽信号场景。' },
        { cmd:'AT+CFUN=1,1', category:'系统控制', desc:'软重启模组',
          syntax:'AT+CFUN=1,1',
          response:'OK',
          params:[],
          example:'AT+CFUN=1,1\nOK', note:'模组执行软重启，约 10-30 秒后重新上线。重启期间串口不可用。修改大部分配置后需重启生效。' },
        { cmd:'AT+QFACT=0', category:'系统控制', desc:'恢复出厂设置',
          syntax:'AT+QFACT=0',
          response:'OK',
          params:[],
          example:'AT+QFACT=0\nOK', note:'清除所有用户配置（APN、频段、功能开关等），恢复出厂默认值。执行后需 AT+CFUN=1,1 重启生效。此操作不可撤销！' },
        { cmd:'AT+QUIMSLOT?', category:'系统控制', desc:'查询当前激活的 SIM 卡槽',
          syntax:'AT+QUIMSLOT?',
          response:'+QUIMSLOT: <slot>\nOK',
          params:[
            {name:'slot',desc:'当前卡槽',values:'1=卡槽 1, 2=卡槽 2'}],
          example:'AT+QUIMSLOT?\n+QUIMSLOT: 1\nOK', note:'双 SIM 卡模组支持两个卡槽。切换卡槽后需重新注册网络。' },
        { cmd:'AT+QUIMSLOT=', category:'系统控制', desc:'切换到指定 SIM 卡槽',
          syntax:'AT+QUIMSLOT=<slot>',
          response:'OK',
          params:[
            {name:'slot',desc:'目标卡槽',values:'1=卡槽 1, 2=卡槽 2'}],
          example:'AT+QUIMSLOT=2\nOK', note:'切换后模组会重新检测 SIM 卡并注册网络。建议切换后等待 10 秒再查询状态。' },
        { cmd:'AT+QNWLOCK', category:'系统控制', desc:'锁定到指定 NR5G 小区（ARFCN + PCI 精确锁定）',
          syntax:'AT+QNWLOCK="common/5g",1,<arfcn>,<pci>\nAT+QNWLOCK="common/5g",0（解锁）',
          response:'OK',
          params:[
            {name:'arfcn',desc:'NR ARFCN 频点号',values:'整数，如 630000（n78 频段）'},
            {name:'pci',desc:'物理小区 ID',values:'0-1007'}],
          example:'AT+QNWLOCK="common/5g",1,630000,123\nOK', note:'锁定后模组仅驻留指定频点和 PCI 的小区。用于定点测试特定基站。设为 0 解除锁定。' },
        { cmd:'AT+QNWLOCKFREQ', category:'系统控制', desc:'仅锁定到指定频点（不限 PCI）',
          syntax:'AT+QNWLOCKFREQ="common/5g",1,<arfcn>\nAT+QNWLOCKFREQ="common/5g",0（解锁）',
          response:'OK',
          params:[
            {name:'arfcn',desc:'NR ARFCN 频点号',values:'整数'}],
          example:'AT+QNWLOCKFREQ="common/5g",1,630000\nOK', note:'与 QNWLOCK 的区别：不限定 PCI，允许驻留该频点上任意小区。设为 0 解除锁定。' },
        { cmd:'AT+QSIMLOCK', category:'系统控制', desc:'PLMN（运营商）锁定/解锁（需密码授权）',
          syntax:'AT+QSIMLOCK="PN","<password>",2,"<plmn>"（锁定）\nAT+QSIMLOCK="PN","<password>"（解锁）',
          response:'OK',
          params:[
            {name:'password',desc:'锁定密码',values:'字符串，通常由供应商提供'},
            {name:'plmn',desc:'运营商代码',values:'MCC+MNC 格式，如 "46000"（中国移动）'}],
          example:'AT+QSIMLOCK="PN","12345678",2,"46000"\nOK', note:'锁定后仅允许指定运营商的 SIM 卡使用。需要供应商提供的密码。解锁时不传 plmn 参数。' },
      ],
      qualcomm: [
        { cmd:'AT', category:'通用指令', desc:'测试 AT 串口通信是否正常',
          syntax:'AT',
          response:'OK',
          params:[],
          example:'AT\nOK', note:'最基本的 AT 指令，用于确认模组串口连接正常、模组已开机。若无响应请检查波特率、串口号、接线。' },
        { cmd:'ATE', category:'通用指令', desc:'设置 AT 命令回显开关',
          syntax:'ATE<value>',
          response:'OK',
          params:[
            {name:'value',desc:'回显开关',values:'0=关闭回显, 1=开启回显'}],
          example:'ATE1\nOK', note:'关闭回显后发送的指令不会原样返回，可减少串口数据量，便于程序解析响应。' },
        { cmd:'AT+GMR', category:'通用指令', desc:'查询模组固件版本号',
          syntax:'AT+GMR',
          response:'<firmware_version>\nOK',
          params:[],
          example:'AT+GMR\nRM520NGLAAR03A03M4G\nOK', note:'返回完整的固件版本字符串。高通平台版本号含芯片代号（如 RM520N）、基线版本、编译日期等信息。升级固件前后务必确认。' },
        { cmd:'AT+CGSN', category:'设备信息', desc:'查询模组 IMEI（国际移动设备识别码）',
          syntax:'AT+CGSN',
          response:'<imei>\nOK',
          params:[],
          example:'AT+CGSN\n861234567890456\nOK', note:'IMEI 为 15 位数字，是全球唯一的模组标识。网络注册、运营商白名单均依赖 IMEI。' },
        { cmd:'AT+ICCID', category:'设备信息', desc:'查询 SIM 卡 ICCID（高通平台首选）',
          syntax:'AT+ICCID',
          response:'+ICCID: <iccid>\nOK',
          params:[],
          example:'AT+ICCID\n+ICCID: 89860318640012345678\nOK', note:'高通平台响应前缀为 +ICCID:，展锐平台使用 AT+CCID（前缀 +CCID:），两者命令和前缀均不同。' },
        { cmd:'AT+CGMM', category:'设备信息', desc:'查询模组型号（用于厂商/芯片平台检测）',
          syntax:'AT+CGMM',
          response:'<model>\nOK',
          params:[],
          example:'AT+CGMM\nRM520N\nOK', note:'返回值用于判断芯片平台：RM520N/RM500Q 为高通平台，RM500U/RG200U 为展锐平台。本工具据此自动切换 AT 指令集。' },
        { cmd:'AT+CGMI', category:'设备信息', desc:'查询模组制造商名称',
          syntax:'AT+CGMI',
          response:'Quectel\nOK',
          params:[],
          example:'AT+CGMI\nQuectel\nOK', note:'通常返回 "Quectel"。用于确认模组品牌。' },
        { cmd:'AT+CIMI', category:'设备信息', desc:'查询 SIM 卡 IMSI（国际移动用户识别码）',
          syntax:'AT+CIMI',
          response:'<imsi>\nOK',
          params:[],
          example:'AT+CIMI\n460001234567890\nOK', note:'IMSI 由 MCC（国家码）+ MNC（网络码）+ MSIN（用户号）组成，用于网络侧识别用户身份。' },
        { cmd:'AT+CCLK', category:'设备信息', desc:'查询或设置模组实时时钟',
          syntax:'AT+CCLK?\nAT+CCLK="<time>"',
          response:'+CCLK: "<yy/MM/dd,hh:mm:ss±tz>"\nOK',
          params:[
            {name:'time',desc:'日期时间',values:'格式 yy/MM/dd,hh:mm:ss±tz，tz 为时区偏移（单位：15分钟）'}],
          example:'AT+CCLK?\n+CCLK: "25/05/26,14:30:00+32"\nOK', note:'时区偏移 tz：+32 表示 UTC+8（32×15min=480min=8h）。' },
        { cmd:'AT+QBASELINE', category:'设备信息', desc:'查询 AP/CP 基线版本号',
          syntax:'AT+QBASELINE',
          response:'+QBASELINE: <ap_version>,<cp_version>\nOK',
          params:[],
          example:'AT+QBASELINE\n+QBASELINE: MDM9x07_ES2.0_SEC5G_SVN_SEC5G_21.141.21.01,CP_SVN_SEC5G\nOK', note:'AP 为应用处理器版本，CP 为通信处理器版本。高通平台 AP 通常含 MDM 芯片型号。' },
        { cmd:'AT+QTEMP', category:'设备信息', desc:'查询 SoC 芯片温度及 PA 功放温度',
          syntax:'AT+QTEMP',
          response:'+QTEMP: <soc_temp>,<pa_temp>\nOK',
          params:[
            {name:'soc_temp',desc:'SoC 温度',values:'摄氏度整数，正常 30~60°C'},
            {name:'pa_temp',desc:'PA 温度',values:'摄氏度整数，正常 30~70°C'}],
          example:'AT+QTEMP\n+QTEMP: 45,40\nOK', note:'温度超过 85°C 可能触发降频或关断保护。长时间高负载（持续速率测试）时应关注温度。' },
        { cmd:'AT+CPIN?', category:'SIM与注册', desc:'查询 SIM 卡当前状态',
          syntax:'AT+CPIN?',
          response:'+CPIN: <status>\nOK',
          params:[
            {name:'status',desc:'SIM 状态',values:'READY=正常 | SIM PIN=需要 PIN | SIM PUK=需要 PUK | NOT INSERTED=未插入'}],
          example:'AT+CPIN?\n+CPIN: READY\nOK', note:'SIM PIN/PUK 状态下需先解锁才能执行网络操作。连续输错 PIN（3次）将锁卡，需 PUK 解锁。' },
        { cmd:'AT+CEREG?', category:'SIM与注册', desc:'查询 LTE/5G EPS 网络注册状态',
          syntax:'AT+CEREG?',
          response:'+CEREG: <n>,<stat>[,<tac>,<ci>,<acst>]\nOK',
          params:[
            {name:'stat',desc:'注册状态',values:'0=未注册, 1=已注册(本地), 2=搜索中, 3=注册被拒, 5=已注册(漫游)'}],
          example:'AT+CEREG?\n+CEREG: 0,1,"0B01","1A000101",7\nOK', note:'5G/LTE 核心注册状态。stat=3（被拒）通常是 SIM 卡未开通对应业务或频段不匹配。' },
        { cmd:'AT+COPS?', category:'SIM与注册', desc:'查询当前运营商名称和接入技术',
          syntax:'AT+COPS?',
          response:'+COPS: <mode>[,<format>,<oper>[,<AcT>]]\nOK',
          params:[
            {name:'mode',desc:'选网模式',values:'0=自动选网, 1=手动选网, 4=关闭选网'},
            {name:'format',desc:'名称格式',values:'0=长名称, 1=短名称, 2=数字(MCC+MNC)'},
            {name:'AcT',desc:'接入技术',values:'7=LTE, 9=NR5G, 12=NR5G-SA'}],
          example:'AT+COPS?\n+COPS: 0,0,"CHINA MOBILE",9\nOK', note:'用于确认当前驻留的运营商和网络制式。AcT=9 为 NSA，AcT=12 为 SA。' },
        { cmd:'AT+CGATT?', category:'SIM与注册', desc:'查询 GPRS 附着状态',
          syntax:'AT+CGATT?',
          response:'+CGATT: <state>\nOK',
          params:[
            {name:'state',desc:'附着状态',values:'0=未附着, 1=已附着'}],
          example:'AT+CGATT?\n+CGATT: 1\nOK', note:'GPRS 附着是数据通信的前提。state=0 时无法建立数据连接。' },
        { cmd:'AT+QENG="servingcell"', category:'网络与信号', desc:'查询当前服务小区详细信息（频段、PCI、ARFCN、信号等）',
          syntax:'AT+QENG="servingcell"',
          response:'+QENG: "servingcell","CONNECT","NR5G-SA","SA",<MCC>,<MNC>,<CellID>,<PCI>,<ARFCN>,<BW_IDX>,<RSRP>,<RSRQ>,<SINR>,<TxPwr>,<RxLev>,<SCS>\nOK',
          params:[
            {name:'state',desc:'连接状态',values:'CONNECT | SEARCH | LIMSRV'},
            {name:'tech',desc:'接入技术',values:'NR5G-SA | NR5G-NSA | LTE'},
            {name:'BW_IDX',desc:'带宽索引',values:'高通专用索引：0=5MHz, 7=50MHz, 12=100MHz'},
            {name:'RSRP',desc:'参考信号接收功率',values:'dBm 负值，-80以上优秀'},
            {name:'RSRQ',desc:'参考信号接收质量',values:'dB 负值，-10以上优秀'},
            {name:'SCS',desc:'子载波间隔',values:'0=15kHz, 1=30kHz, 2=60kHz, 3=120kHz, 4=240kHz'}],
          example:'AT+QENG="servingcell"\n+QENG: "servingcell","CONNECT","NR5G-SA","SA",460,00,...,123,630000,12,-85,-8,22,...\nOK', note:'高通带宽为索引值（0-12），展锐为直接 MHz 值。NR5G-SA 响应最多 18 个字段，含 TxPwr（发射功率）、RxLev（接收电平）、SCS（子载波间隔）。' },
        { cmd:'AT+QENG="neighbourcell"', category:'网络与信号', desc:'查询邻近小区列表',
          syntax:'AT+QENG="neighbourcell"',
          response:'+QENG: "neighbourcell intra",<tech>,<pci>,<arfcn>,<rsrp>,...\n+QENG: "neighbourcell inter",<tech>,...\nOK',
          params:[
            {name:'intra',desc:'同频邻区',values:'与当前服务小区同频段的邻近小区'},
            {name:'inter',desc:'异频邻区',values:'与当前服务小区不同频段的邻近小区'}],
          example:'AT+QENG="neighbourcell"\n+QENG: "neighbourcell intra",...\nOK', note:'返回同频(intra)和异频(inter)邻区。邻区信息可用于判断是否应切换频段或小区。' },
        { cmd:'AT+CSQ', category:'网络与信号', desc:'查询信号强度 RSSI 和误码率 BER',
          syntax:'AT+CSQ',
          response:'+CSQ: <rssi>,<ber>\nOK',
          params:[
            {name:'rssi',desc:'信号强度',values:'0-31（越大越强），99=未知'},
            {name:'ber',desc:'误码率',values:'0-7, 99=未知'}],
          example:'AT+CSQ\n+CSQ: 22,0\nOK', note:'最简单的信号查询。建议配合 QENG 获取更精确的 RSRP/RSRQ。' },
        { cmd:'AT+QRSRP', category:'网络与信号', desc:'查询各路天线 RSRP/RSRQ（高通 4 天线专用）',
          syntax:'AT+QRSRP',
          response:'+QRSRP: <ant0_rsrp>,<ant1_rsrp>,<ant2_rsrp>,<ant3_rsrp>,<ant0_rsrq>,<ant1_rsrq>,<ant2_rsrq>,<ant3_rsrq>\nOK',
          params:[
            {name:'ant0~ant3_rsrp',desc:'各天线 RSRP',values:'dBm 负值，4路天线分别显示'},
            {name:'ant0~ant3_rsrq',desc:'各天线 RSRQ',values:'dB 负值'}],
          example:'AT+QRSRP\n+QRSRP: -85,-87,-90,-92,-8,-9,-10,-11\nOK', note:'高通平台 4x4 MIMO 天线诊断。前 4 个值为 RSRP，后 4 个为 RSRQ。若某路明显偏低，可能天线未接或馈线故障。展锐使用 AT+QANTRSSI?。' },
        { cmd:'AT+C5GQOSRDP=', category:'网络与信号', desc:'查询指定 CID 的 5G QoS 参数',
          syntax:'AT+C5GQOSRDP=<cid>',
          response:'+C5GQOSRDP: <cid>,<5QI>,<ul_bw>,<dl_bw>,...\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'5QI',desc:'5G QoS 标识符',values:'整数，9=默认承载'},
            {name:'ul_bw',desc:'上行保证带宽',values:'kbps'},
            {name:'dl_bw',desc:'下行保证带宽',values:'kbps'}],
          example:'AT+C5GQOSRDP=1\n+C5GQOSRDP: 1,9,100000,200000,...\nOK', note:'需先建立数据连接。5QI 值反映运营商分配的 QoS 等级，影响上下行速率保障。' },
        { cmd:'AT+QNWPREFCFG=?', category:'网络与信号', desc:'查询模组硬件支持的 LTE 和 NR 频段列表',
          syntax:'AT+QNWPREFCFG=?',
          response:'+QNWPREFCFG: "lte_band",<supported_bands>\n+QNWPREFCFG: "nr5g_band",<supported_bands>\nOK',
          params:[],
          example:'AT+QNWPREFCFG=?\n+QNWPREFCFG: "lte_band",...\nOK', note:'返回模组射频硬件实际支持的频段。设置频段时不应超出此范围。' },
        { cmd:'AT+QMAP="connect"', category:'数据连接', desc:'激活数据连接（高通平台专用）',
          syntax:'AT+QMAP="connect",<cid>',
          response:'OK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'}],
          example:'AT+QMAP="connect",1\nOK', note:'高通平台数据拨号指令。展锐平台使用 AT+QNETDEVCTL=1,<cid>,1 代替。连接前需确保 PDP 上下文已配置。' },
        { cmd:'AT+QMAP="disconnect"', category:'数据连接', desc:'断开数据连接',
          syntax:'AT+QMAP="disconnect",<cid>',
          response:'OK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'}],
          example:'AT+QMAP="disconnect",1\nOK', note:'断开后 IP 地址释放。展锐平台使用 AT+QNETDEVCTL=0,<cid>,1。' },
        { cmd:'AT+QMAP="WWAN"', category:'数据连接', desc:'查询 IP 地址、子网掩码、网关（高通平台专用）',
          syntax:'AT+QMAP="WWAN"',
          response:'+QMAP: "WWAN",<cid>,<state>,<ipv4>/<prefix>,<gw>,...\nOK',
          params:[
            {name:'state',desc:'连接状态',values:'0=未连接, 1=已连接'},
            {name:'ipv4',desc:'IPv4 地址',values:'含 CIDR 前缀长度'},
            {name:'gw',desc:'网关',values:'IP 地址'}],
          example:'AT+QMAP="WWAN"\n+QMAP: "WWAN",1,1,"10.100.50.1/28","10.100.50.14",...\nOK', note:'若无结果可降级使用 AT+CGPADDR 获取 IP 地址。展锐平台使用 AT+QNETDEVSTATUS=<cid>。' },
        { cmd:'AT+CGACT', category:'数据连接', desc:'激活或去激活 PDP 上下文',
          syntax:'AT+CGACT=<state>,<cid>\nAT+CGACT?',
          response:'+CGACT: <cid>,<state>\nOK',
          params:[
            {name:'state',desc:'激活状态',values:'0=去激活, 1=激活'},
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'}],
          example:'AT+CGACT=1,1\nOK', note:'3GPP 标准指令。高通平台实际拨号使用 QMAP，此命令用于控制 PDP 上下文激活状态。' },
        { cmd:'AT+QICSGP=', category:'数据连接', desc:'配置 PDP 上下文的 APN、协议类型和认证信息',
          syntax:'AT+QICSGP=<cid>,<context_type>,"<apn>","<username>","<password>",<auth_type>\nAT+QICSGP=<cid>',
          response:'+QICSGP: <cid>,<context_type>,"<apn>","<username>","<password>",<auth_type>\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'context_type',desc:'PDP 协议类型',values:'1=IPv4, 2=IPv6, 3=IPv4v6'},
            {name:'apn',desc:'APN 接入点名称',values:'如 "cmnet"(移动)/"3gnet"(联通)/"ctnet"(电信)'},
            {name:'auth_type',desc:'认证方式',values:'0=无认证, 1=PAP, 2=CHAP, 3=PAP或CHAP'}],
          example:'AT+QICSGP=1,1,"cmnet","","",0\nOK', note:'国内运营商 APN 通常无需认证。APN 配置错误是最常见的无法上网原因。' },
        { cmd:'AT+CGDCONT=', category:'数据连接', desc:'定义或删除 PDP 上下文',
          syntax:'AT+CGDCONT=<cid>[,<PDP_type>[,"<apn>"]]\nAT+CGDCONT=<cid>（删除）',
          response:'OK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'PDP_type',desc:'PDP 类型',values:'"IP" | "IPV6" | "IPV4V6"'},
            {name:'apn',desc:'APN 名称',values:'字符串'}],
          example:'AT+CGDCONT=1,"IP","cmnet"\nOK', note:'只传 cid 时删除该上下文。与 QICSGP 配合使用。' },
        { cmd:'AT+CGDCONT?', category:'数据连接', desc:'查询所有已定义的 PDP 上下文配置',
          syntax:'AT+CGDCONT?',
          response:'+CGDCONT: <cid>,<PDP_type>,<apn>,<ip_addr>,...\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'PDP_type',desc:'协议类型',values:'IP | IPV6 | IPV4V6'},
            {name:'ip_addr',desc:'已分配 IP',values:'未激活时为空'}],
          example:'AT+CGDCONT?\n+CGDCONT: 1,"IP","cmnet",10.100.50.1,...\nOK', note:'可查看所有 PDP 上下文的配置和当前 IP。' },
        { cmd:'AT+CGPADDR', category:'数据连接', desc:'查询 PDP 上下文已分配的 IP 地址',
          syntax:'AT+CGPADDR=<cid>\nAT+CGPADDR',
          response:'+CGPADDR: <cid>,<ipv4>[,<ipv6>]\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16，省略则查询全部'}],
          example:'AT+CGPADDR=1\n+CGPADDR: 1,10.100.50.1\nOK', note:'当 QMAP="WWAN" 无结果时的降级方案。IP 为 0.0.0.0 表示上下文已定义但未激活。' },
        { cmd:'AT+QGDNRCNT?', category:'流量统计', desc:'查询上下行流量累计字节数（高通平台）',
          syntax:'AT+QGDNRCNT?',
          response:'+QGDNRCNT: <tx_bytes>,<rx_bytes>\nOK',
          params:[
            {name:'tx_bytes',desc:'上行（发送）流量',values:'字节数，累计值'},
            {name:'rx_bytes',desc:'下行（接收）流量',values:'字节数，累计值'}],
          example:'AT+QGDNRCNT?\n+QGDNRCNT: 2048000,10240000\nOK', note:'展锐平台使用 AT+QGDCNT?，命令名不同，不可互换。数值为累计值，断电不清除。' },
        { cmd:'AT+QGDNRCNT=0', category:'流量统计', desc:'重置高通平台流量计数器',
          syntax:'AT+QGDNRCNT=0',
          response:'OK',
          params:[],
          example:'AT+QGDNRCNT=0\nOK', note:'执行后 tx/rx 字节数归零。通常在开始新的流量测试前重置。' },
        { cmd:'AT+QNWPREFCFG="lte_band"', category:'频段配置', desc:'查询或设置 LTE 频段锁定',
          syntax:'AT+QNWPREFCFG="lte_band"\nAT+QNWPREFCFG="lte_band","<bands>"',
          response:'+QNWPREFCFG: "lte_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'LTE 频段列表',values:'纯数字冒号分隔，如 1:3:8:41（不带引号/B 前缀）'}],
          example:'AT+QNWPREFCFG="lte_band",1:3:8:41\nOK', note:'锁定频段后仅扫描指定频段，可加快驻网速度。设为空字符串恢复默认。' },
        { cmd:'AT+QNWPREFCFG="nr5g_band"', category:'频段配置', desc:'查询或设置 5G NR 频段锁定',
          syntax:'AT+QNWPREFCFG="nr5g_band"\nAT+QNWPREFCFG="nr5g_band","<bands>"',
          response:'+QNWPREFCFG: "nr5g_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'NR 频段列表',values:'冒号分隔，如 "n41:n78:n79"'}],
          example:'AT+QNWPREFCFG="nr5g_band",78:79\nOK', note:'中国电信/联通主要 n78，移动主要 n41/n79。' },
        { cmd:'AT+QNWPREFCFG="mode_pref"', category:'频段配置', desc:'查询或设置首选网络模式',
          syntax:'AT+QNWPREFCFG="mode_pref"\nAT+QNWPREFCFG="mode_pref",<mode>',
          response:'+QNWPREFCFG: "mode_pref",<mode>\nOK',
          params:[
            {name:'mode',desc:'网络模式',values:'AUTO=自动 | LTE=仅4G | NR5G=仅5G | LTE:NR5G=4G+5G'}],
          example:'AT+QNWPREFCFG="mode_pref","NR5G"\nOK', note:'设为 NR5G 强制仅驻留 5G。设为 AUTO 让模组自动选择。' },
        { cmd:'AT+QNWPREFCFG="all_band_reset"', category:'频段配置', desc:'重置所有频段锁定为出厂默认值',
          syntax:'AT+QNWPREFCFG="all_band_reset"',
          response:'OK',
          params:[],
          example:'AT+QNWPREFCFG="all_band_reset"\nOK', note:'清除所有频段锁定。修改频段后若无法驻网可执行此命令。' },
        { cmd:'AT+QCFG="ethernet"', category:'功能开关', desc:'查询或设置以太网接口启用状态',
          syntax:'AT+QCFG="ethernet"\nAT+QCFG="ethernet",<state>',
          response:'+QCFG: "ethernet",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="ethernet",1\nOK', note:'开启以太网功能的前提是 pcie/mode 已开启。' },
        { cmd:'AT+QCFG="pcie/mode"', category:'功能开关', desc:'查询或设置 PCIe 接口模式',
          syntax:'AT+QCFG="pcie/mode"\nAT+QCFG="pcie/mode",<mode>',
          response:'+QCFG: "pcie/mode",<mode>\nOK',
          params:[
            {name:'mode',desc:'PCIe 模式',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="pcie/mode",1\nOK', note:'以太网功能的前置条件。开启后通过 PCIe 提供以太网数据通道。' },
        { cmd:'AT+QCFG="eth_at"', category:'功能开关', desc:'查询或设置通过以太网接口进行 AT 命令通信（高通专有）',
          syntax:'AT+QCFG="eth_at"\nAT+QCFG="eth_at",<state>',
          response:'+QCFG: "eth_at",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="eth_at",1\nOK', note:'开启后可通过以太网口发送 AT 指令，无需串口。仅高通平台支持。' },
        { cmd:'AT+QCFG="data_interface"', category:'高通专用', desc:'查询或设置数据接口通信方式（PCIe/USB）',
          syntax:'AT+QCFG="data_interface"\nAT+QCFG="data_interface",<net_port>,<diag_port>',
          response:'+QCFG: "data_interface",<net_port>,<diag_port>\nOK',
          params:[
            {name:'net_port',desc:'数据口方式',values:'0=USB, 1=PCIe'},
            {name:'diag_port',desc:'诊断口方式',values:'0=USB, 1=PCIe'}],
          example:'AT+QCFG="data_interface"\n+QCFG: "data_interface",0,0\nOK', note:'控制数据端口和诊断端口通过 USB 还是 PCIe 传输。修改后需重启。' },
        { cmd:'AT+QCFG="usbspeed"', category:'高通专用', desc:'查询或设置 USB 接口工作速度',
          syntax:'AT+QCFG="usbspeed"\nAT+QCFG="usbspeed",<speed>',
          response:'+QCFG: "usbspeed",<speed>\nOK',
          params:[
            {name:'speed',desc:'USB 速度',values:'0=USB 2.0（480Mbps）, 1=USB 3.0（5Gbps）'}],
          example:'AT+QCFG="usbspeed",1\nOK', note:'USB 3.0 可显著提升数据吞吐。切换后需重新枚举 USB 设备。' },
        { cmd:'AT+QETH="eth_driver"', category:'高通专用', desc:'查询或设置以太网控制器驱动',
          syntax:'AT+QETH="eth_driver"\nAT+QETH="eth_driver",<driver>',
          response:'+QETH: "eth_driver",<driver>\nOK',
          params:[
            {name:'driver',desc:'驱动模式',values:'整数，具体值取决于模组型号'}],
          example:'AT+QETH="eth_driver"\n+QETH: "eth_driver",0\nOK', note:'控制以太网控制器驱动加载方式。仅高通平台支持。' },
        { cmd:'AT+QCFG="napt"', category:'功能开关', desc:'查询或设置 NAPT 网络地址端口转换功能',
          syntax:'AT+QCFG="napt"\nAT+QCFG="napt",<state>',
          response:'+QCFG: "napt",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="napt",1\nOK', note:'NAPT 允许多个内网设备共享一个公网 IP。路由模式下通常需开启。' },
        { cmd:'AT+QCFG="nat"', category:'功能开关', desc:'查询或设置 NAT 工作模式（桥接或路由）',
          syntax:'AT+QCFG="nat"\nAT+QCFG="nat",<mode>',
          response:'+QCFG: "nat",<mode>\nOK',
          params:[
            {name:'mode',desc:'NAT 模式',values:'0=桥接模式（透传）, 2=路由模式（NAT）'}],
          example:'AT+QCFG="nat",2\nOK', note:'与 napt 不同：napt 控制是否启用，nat 控制桥接/路由模式。' },
        { cmd:'AT+QCFG="proxyarp"', category:'功能开关', desc:'查询或设置 Proxy ARP 代理功能',
          syntax:'AT+QCFG="proxyarp"\nAT+QCFG="proxyarp",<state>',
          response:'+QCFG: "proxyarp",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="proxyarp",1\nOK', note:'Proxy ARP 让模组代理响应内网设备的 ARP 请求。' },
        { cmd:'AT+QCFG="uartat"', category:'功能开关', desc:'查询或设置 UART 串口 AT 命令接口',
          syntax:'AT+QCFG="uartat"\nAT+QCFG="uartat",<state>',
          response:'+QCFG: "uartat",<state>\nOK',
          params:[
            {name:'state',desc:'状态',values:'0=关闭串口AT, 1=开启串口AT'}],
          example:'AT+QCFG="uartat",1\nOK', note:'关闭后无法通过串口发送 AT 指令。' },
        { cmd:'AT+QCFG="usbnet"', category:'功能开关', desc:'查询或设置 USB 网卡工作模式',
          syntax:'AT+QCFG="usbnet"\nAT+QCFG="usbnet",<mode>',
          response:'+QCFG: "usbnet",<mode>\nOK',
          params:[
            {name:'mode',desc:'USB 网卡模式',values:'2=MBIM（移动宽带，Windows 原生）, 3=RNDIS/ECM（通用虚拟网卡）'}],
          example:'AT+QCFG="usbnet",2\nOK', note:'MBIM(2) 适用于 Windows 即插即用；RNDIS(3) 兼容性更广。切换后需重启。' },
        { cmd:'AT+QCFG="usbcfg"', category:'功能开关', desc:'查询或设置 USB 功能配置（含 ADB 调试开关）',
          syntax:'AT+QCFG="usbcfg"\nAT+QCFG="usbcfg",<vid>,<pid>,...,<adb>,<flag>',
          response:'+QCFG: "usbcfg",<vid>,<pid>,...\nOK',
          params:[
            {name:'adb',desc:'ADB 调试开关',values:'倒数第二个字段：0=关闭 ADB, 1=开启 ADB'}],
          example:'AT+QCFG="usbcfg"\n+QCFG: "usbcfg",0x2C7C,0x0801,...\nOK', note:'修改 ADB 时：先查询完整值，仅改倒数第二个字段，其余保持不变。' },
        { cmd:'AT+QCFG="lanip_ex"', category:'功能开关', desc:'查询或设置 LAN 网关 IP 及 DHCP 地址池',
          syntax:'AT+QCFG="lanip_ex"\nAT+QCFG="lanip_ex","<gw>","<start>","<end>"',
          response:'+QCFG: "lanip_ex","<gw>","<start>","<end>"\nOK',
          params:[
            {name:'gw',desc:'网关 IP',values:'如 "192.168.8.1"'},
            {name:'start',desc:'地址池起始',values:'如 "192.168.8.2"'},
            {name:'end',desc:'地址池结束',values:'如 "192.168.8.254"'}],
          example:'AT+QCFG="lanip_ex"\n+QCFG: "lanip_ex","192.168.8.1","192.168.8.2","192.168.8.254"\nOK', note:'修改后需重启生效。' },
        { cmd:'AT+QCFG="5glan"', category:'功能开关', desc:'查询或设置 5GLAN（5G 局域网）功能',
          syntax:'AT+QCFG="5glan"\nAT+QCFG="5glan",<cid>,<state>,1',
          response:'+QCFG: "5glan",<cid>,<state>\nOK',
          params:[
            {name:'cid',desc:'PDP 上下文 ID',values:'1-16'},
            {name:'state',desc:'启用状态',values:'0=关闭, 1=开启'}],
          example:'AT+QCFG="5glan",1,1,1\nOK', note:'5GLAN 用于工业物联网场景，通过 5G 网络建立局域网。' },
        { cmd:'AT+QCFG="netmask"', category:'功能开关', desc:'查询或设置子网掩码模式',
          syntax:'AT+QCFG="netmask"\nAT+QCFG="netmask",<mode>',
          response:'+QCFG: "netmask",<mode>\nOK',
          params:[
            {name:'mode',desc:'掩码模式',values:'0=标准（/24）, 1=扩展（更小子网）'}],
          example:'AT+QCFG="netmask",0\nOK', note:'标准模式使用 255.255.255.0。' },
        { cmd:'AT+QMAP="DMZ"', category:'功能开关', desc:'查询或设置 DMZ 主机（全端口转发）',
          syntax:'AT+QMAP="DMZ"\nAT+QMAP="DMZ"[,<enable>,<IP_family>[,<IP_address>]]',
          response:'+QMAP: "DMZ",<enable>,<IP_family>[,<IP_address>]\nOK',
          params:[
            {name:'enable',desc:'DMZ 开关',values:'0=关闭, 1=开启'},
            {name:'IP_family',desc:'IP 版本',values:'4=IPv4, 6=IPv6'},
            {name:'IP_address',desc:'DMZ 主机 IP',values:'内网 IP 如 192.168.225.50'}],
          example:'AT+QMAP="DMZ"\n+QMAP: "DMZ",0,4\n+QMAP: "DMZ",0,6\nOK\nAT+QMAP="DMZ",1,4,192.168.225.50\nOK', note:'高通平台 DMZ 指令。修改 DMZ 地址前需先关闭（设 enable=0）。' },
        { cmd:'AT+QNWPREFCFG="srv_domain"', category:'网络与信号', desc:'查询或设置服务域（CS 语音 / PS 数据 / 混合）',
          syntax:'AT+QNWPREFCFG="srv_domain"\nAT+QNWPREFCFG="srv_domain",<domain>',
          response:'+QNWPREFCFG: "srv_domain",<domain>\nOK',
          params:[
            {name:'domain',desc:'服务域',values:'1=仅 CS（语音）, 2=仅 PS（数据）, 3=CS+PS（混合）'}],
          example:'AT+QNWPREFCFG="srv_domain"\n+QNWPREFCFG: "srv_domain",2\nOK', note:'纯数据模组通常设为 2（仅 PS）。设为 3 可支持 VoLTE。' },
        { cmd:'AT+QNWPREFCFG="roam_pref"', category:'网络与信号', desc:'查询或设置漫游偏好',
          syntax:'AT+QNWPREFCFG="roam_pref"\nAT+QNWPREFCFG="roam_pref",<pref>',
          response:'+QNWPREFCFG: "roam_pref",<pref>\nOK',
          params:[
            {name:'pref',desc:'漫游偏好',values:'0=不漫游（仅本网）, 1=允许漫游, 2=拒绝漫游'}],
          example:'AT+QNWPREFCFG="roam_pref"\n+QNWPREFCFG: "roam_pref",0\nOK', note:'跨运营商漫游场景需设为 1。' },
        { cmd:'AT+QNWPREFCFG="voice_domain"', category:'网络与信号', desc:'查询或设置语音域偏好',
          syntax:'AT+QNWPREFCFG="voice_domain"\nAT+QNWPREFCFG="voice_domain",<domain>',
          response:'+QNWPREFCFG: "voice_domain",<domain>\nOK',
          params:[
            {name:'domain',desc:'语音域',values:'0=CS voice only, 1=IMS PS preferred, 2=CS preferred, 3=IMS PS only'}],
          example:'AT+QNWPREFCFG="voice_domain"\n+QNWPREFCFG: "voice_domain",1\nOK', note:'设为 1 优先使用 IMS（VoLTE/VoNR）。' },
        { cmd:'AT+QNWPREFCFG="gw_band"', category:'频段配置', desc:'查询或设置 WCDMA (3G) 频段',
          syntax:'AT+QNWPREFCFG="gw_band"\nAT+QNWPREFCFG="gw_band",<bands>',
          response:'+QNWPREFCFG: "gw_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'WCDMA 频段',values:'冒号分隔数字，如 1:2:5:8'}],
          example:'AT+QNWPREFCFG="gw_band"\n+QNWPREFCFG: "gw_band",1:8\nOK', note:'WCDMA 频段锁定。5G 模组较少使用 3G。' },
        { cmd:'AT+QNWPREFCFG="ue_usage_setting"', category:'网络与信号', desc:'查询或设置 UE 使用场景',
          syntax:'AT+QNWPREFCFG="ue_usage_setting"\nAT+QNWPREFCFG="ue_usage_setting",<setting>',
          response:'+QNWPREFCFG: "ue_usage_setting",<setting>\nOK',
          params:[
            {name:'setting',desc:'使用场景',values:'0=通用, 1=语音优先, 2=数据优先'}],
          example:'AT+QNWPREFCFG="ue_usage_setting"\n+QNWPREFCFG: "ue_usage_setting",0\nOK', note:'数据模组建议设为 2（数据优先）。' },
        { cmd:'AT+QNWPREFCFG="nsa_nr5g_band"', category:'频段配置', desc:'查询或设置 5G NSA 频段（高通平台专用）',
          syntax:'AT+QNWPREFCFG="nsa_nr5g_band"\nAT+QNWPREFCFG="nsa_nr5g_band",<bands>',
          response:'+QNWPREFCFG: "nsa_nr5g_band",<bands>\nOK',
          params:[
            {name:'bands',desc:'NSA NR 频段',values:'冒号分隔，如 1:3:7:20:28:78:79'}],
          example:'AT+QNWPREFCFG="nsa_nr5g_band"\n+QNWPREFCFG: "nsa_nr5g_band",78:79\nOK', note:'NSA 模式下独立控制 5G 频段。与 nr5g_band（SA）分开设置。' },
        { cmd:'AT+QNWPREFCFG="rat_acq_order"', category:'网络与信号', desc:'查询或设置 RAT 优先级顺序（高通平台专用）',
          syntax:'AT+QNWPREFCFG="rat_acq_order"\nAT+QNWPREFCFG="rat_acq_order",<order>',
          response:'+QNWPREFCFG: "rat_acq_order",<order>\nOK',
          params:[
            {name:'order',desc:'RAT 优先级',values:'冒号分隔，如 NR5G:LTE:WCDMA（从高到低）'}],
          example:'AT+QNWPREFCFG="rat_acq_order"\n+QNWPREFCFG: "rat_acq_order",NR5G:LTE:WCDMA\nOK', note:'设置不同制式的搜索优先级。仅高通平台支持。' },
        { cmd:'AT+QMAP="VLAN"', category:'高通专用', desc:'查询或设置 VLAN 虚拟局域网',
          syntax:'AT+QMAP="VLAN"\nAT+QMAP="VLAN"[,<VLAN_ID>,<enable>[,<VLAN_type>]]',
          response:'+QMAP: "VLAN",<VLAN_ID>,<enable>\nOK',
          params:[
            {name:'VLAN_ID',desc:'VLAN ID',values:'1-4094'},
            {name:'enable',desc:'启用状态',values:'"enable"=启用, "disable"=禁用'},
            {name:'VLAN_type',desc:'VLAN 类型',values:'1=ETH, 2=USB'}],
          example:'AT+QMAP="VLAN"\n+QMAP: "VLAN",0\n+QMAP: "VLAN",2,1\nOK\nAT+QMAP="VLAN",4,"enable",1\nOK', note:'VLAN 功能仅高通平台支持，通过 AT+QMAP="VLAN" 配置。' },
        { cmd:'AT+QMAP="LANIP"', category:'高通专用', desc:'查询或设置 LAN DHCP 地址池',
          syntax:'AT+QMAP="LANIP"\nAT+QMAP="LANIP"[,<start_ip>,<end_ip>,<gw_ip>[,<effect>]]',
          response:'+QMAP: "LANIP",<start_ip>,<end_ip>,<gw_ip>\nOK',
          params:[
            {name:'start_ip',desc:'地址池起始 IP',values:'如 "192.168.225.20"'},
            {name:'end_ip',desc:'地址池结束 IP',values:'如 "192.168.225.60"'},
            {name:'gw_ip',desc:'网关 IP',values:'如 "192.168.225.1"'},
            {name:'effect',desc:'生效方式',values:'1=立即生效, 0或省略=重启后生效'}],
          example:'AT+QMAP="LANIP"\n+QMAP: "LANIP",192.168.225.20,192.168.225.60,192.168.225.1\nOK', note:'高通平台通过 QMAP 配置 DHCP 地址池。展锐平台使用 AT+QCFG="lanip_ex"。' },
        { cmd:'AT+CFUN?', category:'系统控制', desc:'查询当前射频功能状态',
          syntax:'AT+CFUN?',
          response:'+CFUN: <fun>\nOK',
          params:[
            {name:'fun',desc:'功能等级',values:'0=最小功能（射频关）, 1=全功能（射频开）, 4=飞行模式'}],
          example:'AT+CFUN?\n+CFUN: 1\nOK', note:'fun=0 射频关闭但 AT 可用；fun=4 飞行模式。' },
        { cmd:'AT+CFUN=1', category:'系统控制', desc:'开启射频（全功能模式）',
          syntax:'AT+CFUN=1',
          response:'OK',
          params:[],
          example:'AT+CFUN=1\nOK', note:'从最小功能/飞行模式恢复，射频开启后开始搜网。' },
        { cmd:'AT+CFUN=0', category:'系统控制', desc:'关闭射频（最小功能模式）',
          syntax:'AT+CFUN=0',
          response:'OK',
          params:[],
          example:'AT+CFUN=0\nOK', note:'关闭射频，AT 指令仍可用。用于省电或屏蔽信号。' },
        { cmd:'AT+CFUN=1,1', category:'系统控制', desc:'软重启模组',
          syntax:'AT+CFUN=1,1',
          response:'OK',
          params:[],
          example:'AT+CFUN=1,1\nOK', note:'约 10-30 秒后重新上线。大部分配置修改后需重启生效。' },
        { cmd:'AT+QFACT=0', category:'系统控制', desc:'恢复出厂设置',
          syntax:'AT+QFACT=0',
          response:'OK',
          params:[],
          example:'AT+QFACT=0\nOK', note:'清除所有用户配置，恢复出厂默认值。此操作不可撤销！执行后需 AT+CFUN=1,1 重启。' },
        { cmd:'AT+QUIMSLOT?', category:'系统控制', desc:'查询当前激活的 SIM 卡槽',
          syntax:'AT+QUIMSLOT?',
          response:'+QUIMSLOT: <slot>\nOK',
          params:[
            {name:'slot',desc:'当前卡槽',values:'1=卡槽 1, 2=卡槽 2'}],
          example:'AT+QUIMSLOT?\n+QUIMSLOT: 1\nOK', note:'双 SIM 模组支持两个卡槽。' },
        { cmd:'AT+QUIMSLOT=', category:'系统控制', desc:'切换到指定 SIM 卡槽',
          syntax:'AT+QUIMSLOT=<slot>',
          response:'OK',
          params:[
            {name:'slot',desc:'目标卡槽',values:'1=卡槽 1, 2=卡槽 2'}],
          example:'AT+QUIMSLOT=2\nOK', note:'切换后需等待 10 秒再查询状态。' },
        { cmd:'AT+QNWLOCK', category:'系统控制', desc:'锁定到指定 NR5G 小区（ARFCN + PCI 精确锁定）',
          syntax:'AT+QNWLOCK="common/5g",<pci>,<arfcn>,<scs>,<band>\nAT+QNWLOCK="common/5g",0（解锁）',
          response:'OK',
          params:[
            {name:'pci',desc:'物理小区 ID',values:'0-1007'},
            {name:'arfcn',desc:'NR ARFCN 频点号',values:'整数，如 630000（n78 频段）'},
            {name:'scs',desc:'子载波间隔 (kHz)',values:'15, 30, 60, 120, 240'},
            {name:'band',desc:'NR 频段号',values:'整数，如 78'}],
          example:'AT+QNWLOCK="common/5g",123,630000,30,78\nOK', note:'锁定后模组仅驻留指定小区。设为 0 解除。' },
        { cmd:'AT+QSIMLOCK', category:'系统控制', desc:'PLMN（运营商）锁定/解锁（需密码）',
          syntax:'AT+QSIMLOCK="PN","<password>",2,"<plmn>"（锁定）\nAT+QSIMLOCK="PN","<password>"（解锁）',
          response:'OK',
          params:[
            {name:'password',desc:'锁定密码',values:'由供应商提供'},
            {name:'plmn',desc:'运营商代码',values:'MCC+MNC，如 "46000"（移动）'}],
          example:'AT+QSIMLOCK="PN","12345678",2,"46000"\nOK', note:'锁定后仅允许指定运营商 SIM。解锁时不传 plmn。' },
      ],
    };


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
      const cmds = AT_DB[atdbPlatform];
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
      const badgeClass = platform === 'unisoc' ? 'unisoc' : 'qualcomm';
      const badgeLabel = platform === 'unisoc' ? '展锐 UniSoc' : '高通 Qualcomm';

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
        if (hw.model) {
          state.model = hw.model;
          document.getElementById('logoText').textContent = hw.model;
          const isQualcomm = isQualcommModel(hw.model);

          const unisocBtn = document.getElementById('tabBtnUnisoc');
          const qualcommBtn = document.getElementById('tabBtnQualcomm');
          const unisocPanel = document.getElementById('hwtab-unisoc');
          const qualcommPanel = document.getElementById('hwtab-qualcomm');

          if (isQualcomm) {
            if (unisocBtn) unisocBtn.classList.add('disabled');
            if (qualcommBtn) qualcommBtn.classList.remove('disabled');
            if (unisocPanel) unisocPanel.classList.remove('active');
            switchHardwareTab('qualcomm', qualcommBtn);
          } else {
            if (qualcommBtn) qualcommBtn.classList.add('disabled');
            if (unisocBtn) unisocBtn.classList.remove('disabled');
            if (qualcommPanel) qualcommPanel.classList.remove('active');
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
        [0, 1, 2].forEach(i => {
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
      [0, 1, 2].forEach(i => document.getElementById('qcIppt_' + i).classList.toggle('active', i === mode));
      const labels = { 0: 'IPPT 已关闭', 1: 'IPPT 路由已设置', 2: 'IPPT 桥接已设置' };
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
        [0, 1, 2].forEach(i => document.getElementById('qcIppt_' + i).classList.toggle('active', i === prev));
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
      if (!state.connected) return;
      try {
        await invoke('set_feature_toggle', { feature, enabled });
        await flushAtLog();
        addTerminalLine(`[功能] ${labels[feature]} 已${enabled ? '开启' : '关闭'}`, enabled ? 'ok' : 'info');
      } catch (e) {
        addTerminalLine(`[功能] ${labels[feature]} 设置失败: ${e}`, 'err');
        const key = keyMap[feature];
        const onBtn = document.getElementById('toggle' + key + '_on');
        const offBtn = document.getElementById('toggle' + key + '_off');
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

        // Connected port was physically removed → force disconnect
        if (state.connected && state.connectedPort && removed.includes(state.connectedPort)) {
          console.log('[USB] Connected port removed, disconnecting');
          addTerminalLine('[USB] AT端口已拔出，断开连接', 'cmd');
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

        // Always refresh port list to reflect actual hardware state
        if (added.length > 0 || removed.length > 0) {
          refreshPortList().catch(() => {});
        }

        // New port appeared and we're idle → auto-connect
        if (!state.connected && state.idle && added.length > 0) {
          console.log('[USB] New port detected, trying auto-connect');
          addTerminalLine('[USB] 检测到新端口，等待设备就绪后连接...', 'info');
          setTimeout(() => {
            if (!state.connected && state.idle) toggleConnection();
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
      $.statusLabel.textContent = '正在初始化...';
      showLoading('正在初始化...', '扫描串口端口');
      try {
        const ver = await invoke('get_app_version');
        if ($.appVersion) $.appVersion.textContent = 'v' + ver;
        if ($.aboutVersion) $.aboutVersion.textContent = 'v' + ver;
      } catch (_) {}
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
