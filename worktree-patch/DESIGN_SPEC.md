# Modem Cat · Claude Code 风 UI 设计规范

> 这份文档是给 AI 助手（Claude Code 等）读的设计契约。任何 UI 新增 / 改动都必须遵循这里的 tokens 与组件规则。
> 完整 CSS 见 `src/desktop/index.html` 里的 `<style id="claude-skin">` 块。

---

## 1. 设计哲学

把工具从「深蓝赛博 + 霓虹橙」翻译成「暖纸面 + 焦糖橙 + 暖深色」的 Claude Code 桌面调性：

- **克制**：除非状态需要语义色（success/warn/err/accent），其余一律 neutral
- **纸面感**：浅色背景是暖米白 `#F7F3EC`，不是纯白；深色是暖深 `#1A1714`，不是赛博蓝
- **数据 mono**：所有数字、十六进制、IP、IMEI、AT 命令、频段名一律 JetBrains Mono + tabular-nums
- **不发光**：不要 box-shadow glow、不要 filter: drop-shadow、不要 transform: translateY hover 抬起。Hover 用 3px 软光圈 `box-shadow: 0 0 0 3px var(--accent-glow)` 就够
- **圆角中等**：按钮 10px，面板 12px，小卡片/输入框/info-item 10px，segmented 控件 6-8px

---

## 2. 颜色 Tokens（必须用变量，禁止硬编码颜色）

### 浅色（默认）
```css
--bg-primary:    #F7F3EC;   /* 全局背景，暖米白 */
--bg-secondary:  #FFFFFF;   /* 面板 / 卡片 */
--bg-tertiary:   #FAF6EE;   /* info-item / input 底 */
--border-color:  #E5DDCB;   /* 1px hairline */
--text-primary:  #1E1B16;   /* 主文字 */
--text-secondary:#5F574A;
--text-muted:    #8C8474;   /* label / hint */
--accent:        #EA580C;   /* 焦糖橙 */
--accent-hover:  #C2410C;
--accent-glow:   rgba(234, 88, 12, 0.18);
--success:       #2F8A53;
--warning:       #B5870F;
--error:         #B5483A;
--danger:        #B5483A;
--tab-bar-bg:    #EFEAE0;
--tab-active-bg: #FFFFFF;
```

### 深色
```css
--bg-primary:    #1A1714;   /* 不要用 #000 或赛博蓝 */
--bg-secondary:  #25211C;
--bg-tertiary:   #2C2823;
--border-color:  #36312A;
--text-primary:  #F2ECDC;   /* 暖米黄文字 */
--text-secondary:#B5AB97;
--text-muted:    #847B6B;
--accent:        #F97316;   /* 深色用更亮一档 */
--success:       #6FBE85;
--warning:       #E0B548;
--error:         #E07A6B;
```

---

## 3. 字体

```css
body { font-family: 'Inter', -apple-system, 'PingFang SC', system-ui, sans-serif; }
.mono, .terminal, .info-value, .data-table td, .band-chip, .terminal-input input {
  font-family: 'JetBrains Mono', 'SF Mono', Consolas, ui-monospace, monospace;
  font-feature-settings: 'tnum';   /* 表格数字 */
}
```

- 全文 body 13.5px
- panel-title 11px uppercase letter-spacing 0.08em
- info-label 10.5px uppercase
- info-value 14.5px mono
- 主标题 / hero 数字才上 18px+

---

## 4. 组件契约

### 4.1 Panel（面板）
```html
<div class="panel">
  <div class="panel-header">
    <div class="panel-title">面板标题</div>
    <button class="btn btn-secondary btn-sm">操作</button>
  </div>
  <!-- content -->
</div>
```
- `border-radius: 12px`，1px hairline 描边，浅阴影
- `panel-title`：11px / 600 / 大写 / letter-spacing 0.08em / `--text-muted`
- panel hover **不要**变色

### 4.2 Button
```html
<button class="btn btn-primary">主按钮</button>      <!-- 实心橙 -->
<button class="btn btn-secondary">次按钮</button>    <!-- bg-tertiary -->
<button class="btn btn-danger">危险</button>         <!-- 边框 only -->
<button class="btn btn-sm">小按钮</button>           <!-- 8px radius -->
```
- 圆角 10px（btn-sm 8px）
- 不要 box-shadow / glow

### 4.3 info-grid（4 列数据小卡）
```html
<div class="info-grid">                <!-- 4 列 -->
<div class="info-grid-2">              <!-- 2 列 -->
  <div class="info-item">
    <div class="info-header">
      <svg class="info-icon">...</svg> <!-- 13×13 橙色 -->
      <div class="info-label">RSRP</div>
    </div>
    <div class="info-value good">−85 dBm</div>
    <!-- value class: good / warn / data / muted -->
  </div>
</div>
```
- info-item：圆角 10px，min-height 64px，hover 出 3px 橙色软光圈
- info-label：10.5px / 600 / 大写
- info-value：mono / 14.5px / 左对齐 / tabular-nums

### 4.4 tab-bar（页内 tab）
```html
<div class="tab-bar">
  <button class="tab-btn active">网络状态</button>
  <button class="tab-btn">IP 信息</button>
</div>
```
- 容器圆角 10px，tab 圆角 7px
- active：白底浅阴影 + 主色文字

### 4.5 toggle-group（分段选择）
```html
<div class="toggle-group">
  <button class="toggle-btn active">开启</button>
  <button class="toggle-btn">关闭</button>
</div>
```
- 用于「开/关」「中文/EN」「LTE/NR」这类对立选择
- active：白底 + accent 文字 + 浅阴影

### 4.6 form-group
```html
<div class="form-row">
  <div class="form-group">
    <label>端口</label>
    <input type="text">
    <!-- 或 -->
    <select class="conn-select">...</select>
  </div>
</div>
```
- input / select：高 34px / 圆角 10px / focus 出 3px accent 光圈
- label：11px / 600 / 大写 / letter-spacing 0.06em

### 4.7 Terminal（AT 调试）
```html
<div class="terminal-wrap">
  <div class="terminal">
    <div class="terminal-line tx">→ AT+CGSN</div>
    <div class="terminal-line rx">  861234567890456</div>
    <div class="terminal-line ok">  OK</div>
    <div class="terminal-line err">  ERROR</div>
    <div class="terminal-line info">  ※ 提示</div>
  </div>
  <div class="shortcut-bar"><!-- 等宽快捷命令 chip --></div>
  <div class="terminal-input">
    <span class="terminal-prompt">›</span>
    <input type="text">
    <button class="btn btn-primary btn-sm">发送</button>
  </div>
</div>
```
- 整个 terminal 用 mono 字体
- `›` 提示符 14px / accent 色
- 快捷命令 chip 用 mono 11px

### 4.8 band-chip（频段选择）
```html
<div class="band-chip">n78</div>
<div class="band-chip checked">n78</div>      <!-- 选中 -->
<div class="band-chip invalid">n3</div>        <!-- 不支持 -->
```
- 圆角 8px，mono 字体，选中态为 accent 软底 + accent 文字

### 4.9 settings-row（设置项）
```html
<div class="settings-list">
  <div class="settings-row">
    <div class="settings-label">
      <svg class="settings-icon">...</svg>
      <span>设置名</span>
    </div>
    <div class="toggle-group">...</div>
    <!-- 或 .settings-select / .btn / 文本 -->
  </div>
</div>
```

### 4.10 状态 chip / pill
```html
<span class="chip ok"><span class="chip-dot"></span>已连接</span>
<span class="chip warn">弱信号</span>
<span class="chip err">断开</span>
<span class="chip accent">NR5G-SA</span>
<span class="chip info">n78</span>
```

---

## 5. 数据状态语义

- **正常 / 已连接**：`good` / `chip ok` → `var(--success)`
- **警告 / 弱信号**：`warn` / `chip warn` → `var(--warning)`
- **错误 / 断开**：`chip err` → `var(--error)`
- **数据高亮 / 当前驻留**：`data` / `chip accent` → `var(--accent)`
- **未知 / 待机**：`muted` → `var(--text-muted)`

---

## 6. 加新页面的标准模板

```html
<div class="page" id="page-XXX">
  <!-- 顶部如果需要全局操作（连接、刷新），放一个 panel -->
  <div class="panel">
    <div class="panel-title">配置区</div>
    <div class="form-row">...</div>
  </div>

  <!-- 多 tab 时 -->
  <div class="tab-bar">
    <button class="tab-btn active">Tab 1</button>
    <button class="tab-btn">Tab 2</button>
  </div>

  <!-- 主内容 panel -->
  <div class="panel">
    <div class="panel-header">
      <div class="panel-title">标题</div>
      <button class="btn btn-secondary btn-sm">刷新</button>
    </div>
    <div class="info-grid">
      <!-- info-item × N -->
    </div>
  </div>
</div>
```

---

## 7. 禁止事项

1. ❌ 不要用 emoji 作图标。所有图标用 SVG（`stroke-width: 2`，`stroke-linecap: round`，`fill: none`）
2. ❌ 不要硬编码颜色十六进制 — 一律用 `var(--xxx)`
3. ❌ 不要发光（`filter: drop-shadow`、`box-shadow` 带模糊半径 > 12px）
4. ❌ 不要 hover translateY 抬起
5. ❌ 不要在浅色模式用纯黑文字 / 深色模式用纯白文字 — 用 token
6. ❌ 不要用 Inter 写数字 — 用 JetBrains Mono
7. ❌ 不要在 panel 上再嵌 panel 当容器 — 用 info-item / info-row / 子区块
8. ❌ 不要超过 2 种圆角值在同一组件内

---

## 8. 加新组件的判断流程

写新 UI 前先问：
1. 这个东西在现有 4.1–4.10 里有没有对应组件？有 → 直接用
2. 是不是一个「数据展示卡片」？→ 用 info-item，配 info-icon + info-label + info-value
3. 是不是一个「设置项」？→ 用 settings-row
4. 是不是「开/关」或「二选一」？→ 用 toggle-group
5. 是不是「多选 chip」（频段、能力）？→ 用 band-chip 样式
6. 都不是 → 在 `<style id="claude-skin">` 里加新规则，命名遵循已有风格，**所有色用 token**

---

## 9. 主题适配 checklist

新组件改完后，setTheme('dark') 切换一次，检查：

- [ ] 背景不是纯黑
- [ ] 文字不是纯白
- [ ] accent 自动转 `#F97316`
- [ ] 描边在深色下仍清晰（不要用 `rgba(0,0,0,...)` 写边）
- [ ] hover 光圈在深色下不刺眼

完。
