// scene module — extracted from app.js
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


