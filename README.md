# Modem Cat

Modem Cat 是一个面向 5G 模组调试、配置和交付的 Tauri 桌面工具，当前主线聚焦 Windows 桌面使用场景。

它把常见的模组工作流收敛到一个界面里：

- 模组连接与状态查询
- APN / 频段 / 网络模式配置
- ADB / SSH 调试终端
- 固件下载
- AT 调试与常用查询

## 适合谁

- 现场测试人员：需要快速连接模组、查看状态、执行基础调试
- 研发与联调人员：需要 AT、ADB、SSH、网络配置、固件下载放在同一工具里
- 发布与交付人员：需要明确区分轻量免安装包、完整免安装包和安装包

## 如何使用

### 1. 先选对交付物

Windows 发版产物统一输出到 `dist/` 根目录，常见选择如下：

| 产物 | 文件名模式 | 适用场景 |
|---|---|---|
| 轻量免安装包 | `ModemCat_vX.Y.Z_portable-lite.zip` | 目标机器已经安装系统 WebView2，想要更小体积、更快分发 |
| 完整免安装包 | `ModemCat_vX.Y.Z_portable.zip` | 目标机器可能没有 WebView2，需要离线开箱即用 |
| NSIS 安装包 | `Modem Cat_X.Y.Z_x64-setup.exe` | 常规终端用户安装，适合桌面快捷方式和开始菜单交付 |
| MSI 安装包 | `Modem Cat_X.Y.Z_x64_zh-CN.msi` | 企业分发、批量部署、标准 Windows 安装介质 |

补充说明：

- `portable-lite.zip` 与 `portable.zip` 都包含 `modem-cat.exe`、ADB 组件、`r26-cli` 固件 sidecar。
- 两个 portable ZIP 都额外包含 `vcruntime140.dll`（x86），供 `r26-cli` 固件 sidecar 直接运行。
- 两者唯一差异是 `portable.zip` 额外内置 `webview2-runtime/`。
- 如果轻量免安装包在目标机器上提示缺少 WebView2，请改用完整免安装包，或先安装系统 WebView2 Runtime。

### 2. 启动后做什么

1. 接入模组并启动程序。
2. 在状态页选择连接方式：
   - 串口直连
   - TCP
   - WebSocket 网关
3. 连接成功后，按需要进入对应页面：
   - `模组状态`：看 SIM、注册、IMEI、ICCID、流量、连接状态
   - `蜂窝网络`：改 APN、网络模式、频段、小区锁
   - `AT 调试`：手工发送 AT
   - `ADB 调试` / `SSH 调试`：进入 shell
   - `固件下载`：选择 PAC，执行刷机

### 3. 使用时要知道的几个点

- ADB 调试依赖 `adb.exe`、`AdbWinApi.dll`、`AdbWinUsbApi.dll`，仓库和发布包都已包含。
- 固件下载依赖 `r26-cli` sidecar，仓库和发布包都已包含。
- `r26-cli` 额外需要 x86 `vcruntime140.dll`；发布脚本会一起带上，缺它只会影响固件下载，不影响主程序启动。
- MQTT 没有公开默认 broker / 凭据；若要启用，必须显式设置环境变量。
- 轻量免安装包不自带 WebView2 运行时，这是它体积更小的原因。
- `dist/modem-cat.exe` 现在不是“裸 exe”示例，而是和同层 `dist/webview2-runtime/` 一起构成可直接双击验证的完整离线产物；少了这个目录就会提示缺少 WebView2。
- 如果目标机器双击后“完全没反应”，先看 `%LOCALAPPDATA%\\Modem Cat\\logs\\startup.log`；日志会直接写出 `exe` 路径、工作目录以及 `webview2-runtime/msedgewebview2.exe` 是否存在。完整 `portable.zip` 会把 app-local `webview2-runtime/` 一起带上。

## 架构概览

当前主线是一个 Tauri 2 桌面应用，结构分三层：

```text
src/desktop/
  纯 HTML / CSS / plain JavaScript 前端
        │ invoke / listen
        ▼
src-tauri/src/
  Tauri 装配、连接、状态查询、ADB/SSH、固件下载
        │
        ▼
modem-hal/src/
  transport / vendor / parser / AT 适配层
```

几个关键设计约束：

- live modem I/O 只有一条 AT 主路径，最终统一到 `AtTransport::send_at`
- 前端 live 状态以 `state` 为准
- 后端 live 状态以 `AppState` 为准
- ADB / SSH / 固件下载属于辅助路径，不能旁路污染 live AT 队列

如果你要改代码，建议先读：

- [AGENTS.md](AGENTS.md)
- [docs/CONTEXT_PACK.md](docs/CONTEXT_PACK.md)
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)
- [docs/BUILD.md](docs/BUILD.md)

## 开发环境

### Windows

当前建议直接按 Windows 环境准备，正式 Windows 发版也应在 Windows 机器上执行。

支持范围：

- 开发与打包建议使用 Windows 10 1803 及以上，或 Windows 11
- 默认面向 64 位 Windows 环境

必需环境：

- Rust + Cargo
  推荐通过 `rustup` 安装，并选择 `stable-x86_64-pc-windows-msvc`
- Visual Studio 2019/2022 Build Tools
  安装时必须勾选“使用 C++ 的桌面开发”
- `cargo-tauri`
  首次可由脚本自动安装，也可以手动执行：

```powershell
cargo install tauri-cli --version "^2" --locked
```

运行与打包相关的必需/可选组件：

| 组件 | 是否必需 | 用途 |
|---|---|---|
| 系统 WebView2 Runtime | 运行 `portable-lite.zip` 时通常必需 | 目标机器如果没有系统 WebView2，请改用完整 `portable.zip` 或先安装系统 WebView2 |
| 本地 `src-tauri/webview2-runtime/` 目录 | 仅构建完整离线包时必需 | 用于把 fixed WebView2 一起打进完整 portable 和安装包，并在运行时与 `modem-cat.exe` 同层可达 |
| PowerShell 5.1+ | 建议具备 | 执行 `build.ps1` |
| Git | 建议具备 | 同步仓库、切分支、提交和发布 |

建议按下面顺序准备环境：

1. 安装 Rust
   推荐使用 `rustup`，安装完成后确认目标链路是 `stable-x86_64-pc-windows-msvc`。
2. 安装 Visual Studio Build Tools
   必须勾选“使用 C++ 的桌面开发”，否则 Tauri/Wry 在 Windows 下无法完整链接。
3. 安装或确认 `cargo-tauri`
   可手动安装，也可以在第一次执行发布脚本时让脚本自动补装。
4. 确认 PowerShell 可用
   发布脚本当前以 PowerShell 为入口，建议直接在 Windows PowerShell 或 PowerShell 7 中执行。
5. 按需要准备 WebView2
   - 只开发、测试或发布 `portable-lite.zip`：目标机器有系统 WebView2 即可
   - 要发布完整离线包：构建机本地必须有 `src-tauri/webview2-runtime/`
6. 按需要准备 ADB 来源目录
   如果你要替换随仓的 ADB 二进制，可以在仓库根放 `Sdk/` 或 `sdk/`，发布脚本会优先从这里同步

仓库内已跟踪的发布资产：

- `src-tauri/resources/adb/`
- `src-tauri/resources/r26-runtime/README.md`
- `src-tauri/binaries/r26-cli*`

本地额外准备：

- 若要构建完整离线包，需要在 `src-tauri/webview2-runtime/` 准备 fixed runtime；`scripts/setup-webview2.ps1` 会从官方 WebView2 下载页解析 Fixed Version CAB 并解包到这里
- 若你要替换 ADB 二进制来源，可在 `Sdk/` 或 `sdk/` 中放置 ADB 文件，构建脚本会同步到 `src-tauri/resources/adb/`
- 若你要完整打包固件下载能力，Windows 构建机还需要本地可用的 x86 `vcruntime140.dll`；`build.ps1` / `scripts/build-helper.ps1` 会自动同步到 `src-tauri/resources/r26-runtime/` 与 `dist/vcruntime140.dll`

建议先用下面两条命令确认环境：

```powershell
cargo --version
cargo tauri --version
```

如果你想一次性确认 Windows 构建环境是否齐全，建议再补跑：

```powershell
rustup show
where.exe cargo
where.exe cargo-tauri
```

一个最小可用的 Windows 开发/发布环境，可以理解为：

- Windows 10 1803+ / Windows 11
- Rust MSVC toolchain
- VS Build Tools + “使用 C++ 的桌面开发”
- PowerShell
- 仓库内自带的 `src-tauri/resources/adb/`、`src-tauri/resources/r26-runtime/README.md` 与 `src-tauri/binaries/r26-cli*`

而“完整离线发版环境”则是在上面基础上，再额外准备：

- `src-tauri/webview2-runtime/`
- 构建机本地可用的 x86 `vcruntime140.dll`（脚本会自动同步，不要求跟仓）

### macOS / Linux

- 可以用于阅读代码、跑部分校验、做非 Windows 侧开发
- 完整 Windows 安装包和最终发版仍应在 Windows 机器上执行

## 构建与验证

### 日常开发验证

```bash
cargo test --workspace
cargo build -p modem-hal
bash scripts/verify-docs.sh
```

### Windows 正式打包

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\build.ps1
```

如果只想快速验证编译和安装包，不生成 portable ZIP：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\build.ps1 -Quick
```

## 发版建议

推荐同时交付两种 portable：

- `portable-lite.zip`：给大多数已有系统 WebView2 的内部同事或客户
- `portable.zip`：给环境不确定、需要离线兜底的现场或外部机器

安装包建议：

- 普通人工安装优先 `setup.exe`
- 企业分发、统一部署优先 `.msi`

## 仓库结构

```text
src/desktop/                前端页面与脚本
src-tauri/src/              Tauri 后端
src-tauri/resources/adb/    ADB 发布资源
src-tauri/resources/r26-runtime/ r26 sidecar 运行库占位目录
src-tauri/binaries/         固件 sidecar
modem-hal/src/              transport / vendor / parser
scripts/                    构建、验证、辅助脚本
docs/                       架构、构建、调用链、AT 合同
dist/                       构建产物输出目录
```

## 文档入口

- [docs/BUILD.md](docs/BUILD.md)：构建、发版、产物规则
- [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)：架构边界、状态 owner、后台模块
- [docs/CODE_MAP.md](docs/CODE_MAP.md)：前端页面与 IPC 映射
- [docs/CALL_FLOW.md](docs/CALL_FLOW.md)：关键调用链
- [docs/TECH_STACK.md](docs/TECH_STACK.md)：技术栈与运行约束
- [docs/README.md](docs/README.md)：文档总索引
