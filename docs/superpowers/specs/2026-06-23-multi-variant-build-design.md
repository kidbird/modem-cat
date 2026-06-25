# 多变体构建方案设计

**日期**：2026-06-23
**状态**：待评审

## 目标

让 `build.bat` 一次产出三个分发变体，覆盖 WebView2 装/不装、便携包/安装包的所有使用场景：

1. **带 WebView 安装包** (`*_webview_*`) — 离线自带 WebView2 Runtime，体积大，离线/老机器可用
2. **不带 WebView 安装包** (`*_nowebview_*`) — 安装时按需从微软服务器下载 WebView2，包小
3. **便携包** (`portable/`) — 单文件夹、需系统已装 WebView2

三个变体都**必须**包含固件下载组件 `r26-cli.exe`（sidecar），目前只有安装包包含，便携包缺失。

## 当前状况

- `dist/installer/Modem Cat_0.2.6_x64_zh-CN.msi` (210 MB) — 含 WebView2 离线包，含 r26-cli
- `dist/installer/Modem Cat_0.2.6_x64-setup.exe` (205 MB) — NSIS 版，同样含 WebView2
- `dist/portable/modem-cat.exe` (6.7 MB) — **缺少** r26-cli.exe，无法做固件下载
- 固件下载通过 `app.shell().sidecar("r26-cli")` 调用，要求 sidecar 二进制在主 exe 同级目录

## 设计

### 1. 配置文件分离

新建 `src-tauri/tauri.nowebview.conf.json`：

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "extends": "./tauri.conf.json",
  "bundle": {
    "windows": {
      "webviewInstallMode": {
        "type": "onlineInstaller"
      }
    }
  }
}
```

`tauri.conf.json` 保持现状（`offlineInstaller`）作为带 webview 的默认配置。

### 2. 产物目录结构

```
dist/
├── installer/
│   ├── Modem Cat_0.2.6_webview_x64_zh-CN.msi
│   ├── Modem Cat_0.2.6_webview_x64-setup.exe
│   ├── Modem Cat_0.2.6_nowebview_x64_zh-CN.msi
│   └── Modem Cat_0.2.6_nowebview_x64-setup.exe
└── portable/
    └── Modem Cat_0.2.6_portable/
        ├── modem-cat.exe
        └── r26-cli-x86_64-pc-windows-msvc.exe
```

### 3. `build.bat` 流程重写

```
[1/6] 环境检查 (VS / tauri-cli)
[2/6] webview 安装包构建 (默认配置) → 改名带 _webview_ 后缀
[3/6] nowebview 安装包构建 (--config 覆盖配置) → 改名带 _nowebview_ 后缀
[4/6] license-gen 构建
[5/6] portable 构建 (cargo build --release) + 复制 r26-cli sidecar
[6/6] 汇总 + 打印产物路径
```

### 4. Sidecar 复制细节

Tauri 2 的 `sidecar("r26-cli")` 会按目标三元组解析文件名：
- 解析后文件名 = `r26-cli-x86_64-pc-windows-msvc.exe`（不动后缀）

来源路径（release 构建产物）：
- `src-tauri/target/release/r26-cli-x86_64-pc-windows-msvc.exe`

目标路径：
- `dist/portable/Modem Cat_0.2.6_portable/r26-cli-x86_64-pc-windows-msvc.exe`

### 5. 文件名后缀实现

Tauri 2 的 bundle 文件名格式：`{productName}_{version}_{arch}_{lang}.{ext}`
- 默认产出：`Modem Cat_0.2.6_x64_zh-CN.msi`

后缀添加方式：构建完成后用 `ren`/`Copy-Item` 改名，例如：
```
ren "Modem Cat_0.2.6_x64_zh-CN.msi" "Modem Cat_0.2.6_webview_x64_zh-CN.msi"
```

## 改造任务

1. **新增** `src-tauri/tauri.nowebview.conf.json`
2. **重写** `build.bat`（按 [1/6]-[6/6] 步骤串行）
3. **执行** build.bat 验证三个变体
4. **检查** dist 目录结构是否符合预期

## 风险与注意

- **构建时间**：从一次 tauri build 增加到两次，预计 2m × 2 + 1m17s license-gen + 1m portable ≈ 7 分钟
- **target 目录共享**：webview 和 nowebview 共用 `src-tauri/target/`，第一次构建后产物已就位，第二次会因 `cargo tauri build` 内部判断不重新编译大部分 crate
- **portable 命名空间**：Tauri 的便携版 `mainBinaryName` 默认就是 `modem-cat.exe`，无需额外配置
- **r26-cli.exe 后缀必须保留**：Tauri 解析 sidecar 路径时强制追加目标三元组后缀
