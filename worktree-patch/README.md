# Modem Cat · Claude Code 风皮肤补丁

这个文件夹镜像了项目目录结构，只有一个改动：

```
src/desktop/index.html   ← 替换这一个文件
```

## 改了什么

1. **在 `</head>` 之前追加了一段 `<style id="claude-skin">`** — 重写所有 CSS 变量（颜色 tokens）+ 关键组件的圆角、阴影、字体；保留所有原 class，所以 JS / HTML 一行没动。
2. **小小的 JS 补丁**：原 `setTheme` 初始化只识别 `localStorage.theme === 'light'`，现在也识别 `'dark'`。这样默认从浅色切回深色后刷新不会被吃掉。

其余字节与原始文件 byte-for-byte 相同。

## 怎么用

假设你的 worktree 在 `modem-cat/.claude/worktrees/intelligent-swartz-9e7753/`，把这个 `src/desktop/index.html` 覆盖过去：

```powershell
# Windows PowerShell
$src = "<下载解压后的路径>\worktree-patch\src\desktop\index.html"
$dst = "<modem-cat 路径>\.claude\worktrees\intelligent-swartz-9e7753\src\desktop\index.html"
Copy-Item $src $dst -Force

# 然后切到 worktree 编译
cd <modem-cat 路径>\.claude\worktrees\intelligent-swartz-9e7753\src-tauri
cargo build --release
```

或者直接拉个新 worktree：

```powershell
cd <modem-cat 路径>
git worktree add .claude/worktrees/claude-skin -b claude-skin-ui
# 然后覆盖 src/desktop/index.html
```

## 验证清单

- [ ] 浅色模式：暖米白背景 `#F7F3EC`，焦糖橙 `#EA580C`，面板白底圆角 12px
- [ ] 深色模式：暖深色 `#1A1714`（不是赛博蓝），accent 转 `#F97316`
- [ ] info-grid 4 列卡片：圆角 10px，hover 出橙色 3px 光圈而不是发光抬起
- [ ] AT 终端：JetBrains Mono 等宽字体，`›` 提示符为橙色
- [ ] 频段 chip：圆角 8px，等宽，选中态为橙色软底
- [ ] 设置页 toggle-group：分段选择器风格，选中带白底投影
- [ ] 主题切换按钮（设置页）刷新后状态保留

## 回退

```powershell
git checkout src/desktop/index.html
```

或者直接删掉 `<style id="claude-skin">` 那段也行 — 改动是纯追加的，删了立刻回到原样。
