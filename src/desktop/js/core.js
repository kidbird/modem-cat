// core module — invoke wrapper + global state (extracted from app.js)
    const _rawInvoke = window.__TAURI__?.core?.invoke;
    const invoke = (cmd, args) => {
      if (!_rawInvoke) {
        console.warn(`Tauri IPC not available: invoke('${cmd}')`);
        return Promise.reject(new Error('Tauri IPC not available'));
      }
      return _rawInvoke(cmd, args).catch(err => {
        // CODE_MAP.md §6: invoke 失败时 catch 拿到的对象是 { message: string }。
        // 归一化为 Error.message，让全部 30+ 个 catch 块都能安全使用字符串拼接，
        // 避免 `[object Object]`.
        const message = (err && typeof err === 'object' && err.message)
          ? String(err.message)
          : String(err);
        throw new Error(message);
      });
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
