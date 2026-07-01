# WebView2 构建说明

> 最近更新：2026-06-30
> 当前模式：**fixedRuntime**（`tauri.conf.json` 已配置，构建完全离线）

## 当前配置

`src-tauri/tauri.conf.json`：

```json
"webviewInstallMode": {
  "type": "fixedRuntime",
  "path": "../webview2-runtime"
}
```

- ✅ **构建完全离线**——不下载任何 WebView2 组件
- ✅ **安装包内嵌 WebView2**——目标机器无需联网安装
- ✅ **版本固定**——`webview2-runtime/` 中的版本（当前 150.0.4078.28）由项目控制

## 一次性设置（仅首次）

`webview2-runtime/` 目录不在 git 中（785 文件，~400MB）。首次环境准备：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\setup-webview2.ps1
```

脚本会：
1. 下载 WebView2 Evergreen Standalone Installer（约 200MB，只需一次）
2. 解压到 `webview2-runtime/`
3. 确认 `tauri.conf.json` 已是 fixedRuntime 模式

后续所有构建复用此目录，**永不重新下载**。

## 完整构建

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1
```

产物全部输出到 `dist/` 根目录（详见 `.agents/skills/release-package/SKILL.md`）。

## 模式对比

| 模式 | 构建需联网 | 安装包含 WebView2 | 版本可控 | 适用场景 |
|---|---|---|---|---|
| **fixedRuntime**（当前） | ❌ | ✅ 完整内嵌 | ✅ | 生产发行、离线环境 |
| skip | ❌ | ❌ | — | 开发调试 |
| embedBootstrapper | 首次 | 引导程序（~2MB，安装时在线下载） | ❌ | 有网环境 |
| offlineInstaller | 首次 | 完整安装包（~200MB） | ❌ | 有网环境首次 |

## 相关文件

| 文件 | 作用 |
|---|---|
| `scripts/setup-webview2.ps1` | 一次性下载 + 解压 WebView2 |
| `scripts/build-release.ps1` | 完整发行构建（含 WebView2 打包） |
| `webview2-runtime/` | fixedRuntime 目录（gitignored，本地准备） |
| `src-tauri/tauri.conf.json` | `bundle.windows.webviewInstallMode` 配置 |
