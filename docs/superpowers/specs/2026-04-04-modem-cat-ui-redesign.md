# Modem Cat UI 重设计

**日期:** 2026-04-04
**分支:** 001-modem-debug-tool
**文件:** `src/desktop/index.html`（单文件改造，无新增文件）

---

## 目标

将现有 5G 模组调试界面从布局错乱的原型，改造为结构清晰、功能完整的调试工具。保持现有深色/浅色主题、monospace 字体、橙色 accent 的视觉风格。

---

## 导航结构

| 菜单项 | 图标前缀 | 状态 |
|--------|----------|------|
| 模组状态 | ● | 可用 |
| 蜂窝网络 | ◎ | 可用 |
| AT调试   | > | 可用 |
| 硬件信息 | ⬡ | 可用 |
| 工程模式 | ⚙ | 占位（置灰，不可点击） |

侧边栏底部固定显示连接状态指示器（彩色 dot + 文字）。

---

## 页面设计

### 1. 模组状态

**接口配置面板**
- 连接类型（USB Serial / Ethernet TCP / TTL UART）
- 端口/地址输入
- 波特率输入
- 连接 / 断开按钮

**核心指标面板**（4列 grid）
- 运营商、网络类型、RSRP、SINR、PCI、CellID、RSRQ、TX Power

**天线信号面板**（4列 grid）
- ANT0、ANT1、ANT2、ANT3

**流量统计面板**（4列 grid）
- 签约上行带宽、签约下行带宽、上行流量、下行流量

---

### 2. 蜂窝网络

**APN 配置面板**
- APN 名称
- 用户名
- 密码
- 鉴权类型（None / PAP / CHAP）
- IP 类型（IPv4 / IPv6 / IPv4v6）
- 保存按钮

**网络锁定面板**（含 5 个子区域）

1. **首选网络**：下拉选择（自动 / 仅5G SA / 仅NR NSA / 仅LTE / 仅WCDMA）
2. **频段锁定**：checkbox 网格列出支持频段（LTE: B1/B3/B5/B7/B8/B20/B34/B38/B39/B40/B41；NR: n1/n3/n5/n8/n28/n38/n40/n41/n77/n78/n79），多选，应用按钮
3. **小区锁定**：CellID 输入 + EARFCN/NR-ARFCN 频点输入，锁定/解锁按钮
4. **频点锁定**：EARFCN 或 NR-ARFCN 输入，锁定/解锁按钮
5. **运营商锁定**：MCC 输入 + MNC 输入，锁定/解锁按钮

**邻区信息面板**（两个 tab：LTE / NR）

- LTE 邻区表格：CellID、PCI、RSRP、RSRQ、EARFCN、频偏
- NR 邻区表格：CellID、PCI、RSRP、SINR、NR-ARFCN、频偏
- 刷新按钮

---

### 3. AT调试

- Terminal 输出区（高度自适应填满剩余空间）
- 快捷命令栏（AT / AT+CSQ / AT+CIMI / AT+CGDCONT? / AT+CEREG?）
- 命令输入框 + 发送按钮
- ↑↓ 键翻命令历史

---

### 4. 硬件信息

**信息面板**（2列 grid）
- 模组型号、生产厂家、固件版本、AP基线版本、CP基线版本、SOC温度、PA温度

**操作面板**

Toggle 开关区：
- 开启 ADB
- 开启 LAN AT
- 开启 UART AT
- 开启 AP Log

危险操作区（warning/danger 样式，二次确认）：
- 重启模组
- 恢复出厂设置

---

### 5. 工程模式（占位）

显示"功能开发中"提示，nav item 置灰不可交互。

---

## CSS 规范

沿用现有 CSS 变量体系（`--bg-primary`、`--accent` 等），新增：

```css
--danger: #dc2626;
--toggle-on: var(--accent);
--toggle-off: var(--border-color);
```

新增组件类：
- `.toggle-switch`：自定义 checkbox 样式的滑动开关
- `.tab-bar` + `.tab-panel`：邻区信息 tab 切换
- `.band-grid`：频段 checkbox 网格
- `.danger-zone`：危险操作区红色边框 panel
- `.status-dot`：侧边栏底部连接状态指示

---

## 数据（Mock）

所有数据均为前端 mock，不接后端。连接后填充，断开后清空。

---

## 约束

- 纯 HTML/CSS/JS，零依赖
- 单文件 `src/desktop/index.html`
- 保持深色/浅色主题切换功能
- 不修改后端 TypeScript 代码
