# 页面布局调整实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 将IP信息页面整合到蜂窝网络页面作为第五个Tab，同时确保硬件信息页面的PCIe开关和USB网络模式功能完整

**Architecture:** 修改index.html中的页面结构，将page-ip的内容迁移到page-cellular作为新Tab，保持现有CSS样式和功能逻辑不变

**Tech Stack:** HTML, CSS, JavaScript (原生实现)

---

### Task 1: 将IP信息内容迁移到蜂窝网络页面

**Files:**
- Modify: `src/desktop/index.html:732-910` (page-cellular)
- Modify: `src/desktop/index.html:1062-1147` (page-ip)

- [ ] **Step 1: 在蜂窝网络Tab栏添加IP信息Tab**

定位到蜂窝网络页面的Tab栏（第736-741行），在现有四个Tab后添加IP信息Tab：

```html
<button class="tab-btn" onclick="switchCellularTab('ipinfo', this)">IP 信息</button>
```

- [ ] **Step 2: 在蜂窝网络页面添加IP信息Tab面板**

在`</div><!-- /panel -->`（第909行）之前，添加IP信息Tab面板：

```html
<!-- ─ Tab 5: IP 信息 ─ -->
<div class="tab-panel" id="ctab-ipinfo">
  <div class="panel">
    <div class="panel-header">
      <div class="panel-title">IP 地址</div>
      <button class="btn btn-secondary btn-sm" onclick="refreshIpInfo()">刷新</button>
    </div>
    <div class="info-grid-2">
      <div class="info-item">
        <div class="info-label">IPv4 地址</div>
        <div class="info-value muted" id="ipv4Addr" style="font-size:13px;">--</div>
      </div>
      <div class="info-item">
        <div class="info-label">IPv4 子网掩码</div>
        <div class="info-value muted" id="ipv4Mask" style="font-size:13px;">--</div>
      </div>
      <div class="info-item">
        <div class="info-label">IPv4 网关</div>
        <div class="info-value muted" id="ipv4Gw" style="font-size:13px;">--</div>
      </div>
      <div class="info-item">
        <div class="info-label">IPv4 DNS</div>
        <div class="info-value muted" id="ipv4Dns" style="font-size:13px;">--</div>
      </div>
      <div class="info-item" style="grid-column: span 2;">
        <div class="info-label">IPv6 地址</div>
        <div class="info-value muted" id="ipv6Addr" style="font-size:12px;letter-spacing:0.3px;">--</div>
      </div>
      <div class="info-item">
        <div class="info-label">IPv6 网关</div>
        <div class="info-value muted" id="ipv6Gw" style="font-size:12px;">--</div>
      </div>
      <div class="info-item">
        <div class="info-label">IPv6 DNS</div>
        <div class="info-value muted" id="ipv6Dns" style="font-size:12px;">--</div>
      </div>
    </div>
  </div>

  <div class="panel">
    <div class="panel-title">MTU 配置</div>
    <div class="form-row">
      <div class="form-group narrow" style="min-width:160px;">
        <label>MTU 值（字节）</label>
        <input type="number" id="mtuValue" value="1500" min="576" max="9000" placeholder="1500">
      </div>
      <div class="form-group auto" style="display:flex;align-items:flex-end;">
        <button class="btn btn-primary btn-sm" onclick="applyMtu()">应用</button>
      </div>
      <div class="form-group" style="display:flex;align-items:flex-end;padding-bottom:1px;">
        <span style="font-size:11px;color:var(--text-muted);">推荐值：1500（以太网）/ 1480（PPPoE）/ 1400（VPN）</span>
      </div>
    </div>
  </div>

  <div class="panel">
    <div class="panel-title">功能配置</div>
    <div class="toggle-row">
      <div>
        <div class="toggle-label">Proxy ARP</div>
        <div class="toggle-desc">开启后模组代替局域网设备响应 ARP 请求</div>
      </div>
      <label class="toggle-switch">
        <input type="checkbox" id="toggleProxyArp" onchange="applyToggleIp('proxyArp', this.checked)">
        <span class="toggle-track"></span>
      </label>
    </div>
  </div>

  <div class="panel">
    <div class="panel-title">DMZ 主机</div>
    <div class="form-row">
      <div class="form-group">
        <label>DMZ 主机 IP 地址</label>
        <input type="text" id="dmzHost" placeholder="如 192.168.1.100">
      </div>
      <div class="form-group auto" style="display:flex;gap:8px;align-items:flex-end;">
        <button class="btn btn-primary btn-sm" onclick="applyDmz()">应用</button>
        <button class="btn btn-secondary btn-sm" onclick="clearDmz()">清除</button>
      </div>
    </div>
    <div style="font-size:11px;color:var(--text-muted);margin-top:4px;">DMZ 主机将接收所有未映射的入站流量，请确保主机具备必要的安全防护。</div>
  </div>
</div>
```

- [ ] **Step 3: 更新switchCellularTab函数支持IP信息Tab**

在JavaScript部分的`switchCellularTab`函数中添加对'ipinfo'的支持：

```javascript
function switchCellularTab(tab, btn) {
  document.querySelectorAll('#page-cellular .tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('#page-cellular > .panel > .tab-panel').forEach(p => p.classList.remove('active'));
  btn.classList.add('active');
  document.getElementById('ctab-' + tab).classList.add('active');
}
```

该函数已支持动态处理任何tab名称，无需修改。

- [ ] **Step 4: 删除独立的IP信息页面**

删除整个`page-ip` div（第1062-1147行）：

```html
<!-- ══ IP 信息 ══ -->
<div class="page" id="page-ip">
  ... 整个内容 ...
</div><!-- /page-ip -->
```

- [ ] **Step 5: 从导航栏移除IP信息菜单项**

在导航栏中删除IP信息菜单项（第563-565行）：

```html
<div class="nav-item" data-page="ip">
  <span class="nav-icon">⌥</span>IP 信息
</div>
```

- [ ] **Step 6: 验证实现**

在浏览器中打开index.html，验证：
1. 蜂窝网络页面出现第五个Tab"IP 信息"
2. 点击"IP 信息"Tab显示IP地址、MTU配置、Proxy ARP、DMZ主机
3. 其他Tab（APN配置、网络锁定、小区/频点锁定、邻区信息）正常工作
4. 原始的独立IP信息页面已被移除

- [ ] **Step 7: 提交**

```bash
git add src/desktop/index.html
git commit -m "feat: 将IP信息整合为蜂窝网络页面的第五个Tab"
```

---

### Task 2: 验证硬件信息页面功能完整

**Files:**
- Modify: `src/desktop/index.html:939-1059` (page-hardware)

- [ ] **Step 1: 验证PCIe ↔ 以太网开关已存在**

检查功能开关区域（第978-1030行）确认PCIe开关已存在：

```html
<div class="toggle-row">
  <div>
    <div class="toggle-label">PCIe ↔ 以太网</div>
    <div class="toggle-desc">切换 PCIe 接口与以太网之间的数据通路</div>
  </div>
  <label class="toggle-switch">
    <input type="checkbox" id="togglePcie" onchange="applyToggle('pcie', this.checked)">
    <span class="toggle-track"></span>
  </label>
</div>
```

- [ ] **Step 2: 验证USB网络模式下拉选项已存在**

检查USB网络模式区域（第1032-1048行）确认下拉选项和保存按钮已存在：

```html
<div class="panel">
  <div class="panel-title">USB 网络模式</div>
  <div class="form-row" style="align-items:flex-end;">
    <div class="form-group">
      <label>网络模式</label>
      <select id="usbNetMode">
        <option value="ncm">NCM 网卡</option>
        <option value="rndis">RNDIS 网卡</option>
        <option value="mbim">MBIM 移动宽带</option>
      </select>
    </div>
    <div class="form-group auto">
      <button class="btn btn-danger btn-sm" onclick="saveUsbNetMode()">保存并重启</button>
    </div>
  </div>
  <div style="font-size:11px;color:var(--text-muted);margin-top:4px;">保存后模组将自动重启，USB 连接会短暂中断。</div>
</div>
```

- [ ] **Step 3: 验证saveUsbNetMode函数已实现**

检查JavaScript部分（第1609-1615行）确认函数已实现：

```javascript
function saveUsbNetMode() {
  const mode = document.getElementById('usbNetMode');
  const label = mode.selectedOptions[0].text;
  if (!confirm(`切换 USB 网络模式为「${label}」？\n模组将自动重启，连接会短暂中断。`)) return;
  addTerminalLine(`[USB] 网络模式切换为: ${label}，正在重启...`, 'cmd');
  setTimeout(() => addTerminalLine('[USB] 重启完成', 'ok'), 1200);
}
```

- [ ] **Step 4: 验证applyToggle函数已支持PCIe**

检查JavaScript部分（第1617-1620行）确认PCIe功能已支持：

```javascript
function applyToggle(feature, enabled) {
  const labels = { adb: 'ADB', lanAt: 'LAN AT', uartAt: 'UART AT', apLog: 'AP Log', pcie: 'PCIe ↔ 以太网' };
  addTerminalLine(`[功能] ${labels[feature]} 已${enabled ? '开启' : '关闭'}`, enabled ? 'ok' : 'info');
}
```

- [ ] **Step 5: 提交**

```bash
git add src/desktop/index.html
git commit -m "chore: 确认硬件信息页面功能完整（PCIe开关、USB网络模式）"
```

---

## 实现总结

| 需求 | 状态 | 实现位置 |
|------|------|----------|
| IP信息挪到蜂窝网络下面 | ✅ 需实现 | Task 1: 将IP信息作为第五个Tab加入蜂窝网络页面 |
| PCIe ↔ 以太网开关 | ✅ 已存在 | src/desktop/index.html:1020-1029 |
| USB网络模式下拉选项 | ✅ 已存在 | src/desktop/index.html:1032-1048 |
| 保存并重启按钮 | ✅ 已存在 | src/desktop/index.html:1044 ("保存并重启") |