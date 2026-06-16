// core module — invoke wrapper + global state (extracted from app.js)
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
      licenseStatus: null,
    };
