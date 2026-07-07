# 构建指南

## 概述

Modem Cat 使用 Tauri 2.x 构建桌面应用，**一次构建同时产出两种产物**：

| 产物 | macOS | Windows |
|---|---|---|
| **Portable**（免安装） | `Modem Cat.app` | `modem-cat.exe` |
| **安装包** | `Modem Cat_x.y.z_aarch64.dmg` | `Modem Cat_x.y.z_x64-setup.exe` / `.msi` |

---

## 一、前置依赖

### macOS

| 依赖 | 安装方式 |
|---|---|
| Rust + Cargo | `curl https://sh.rustup.rs -sSf \| sh` |
| Xcode CLI Tools | `xcode-select --install` |
| cargo-tauri | 首次运行构建脚本时自动安装，或手动 `cargo install tauri-cli --version '^2' --locked` |

### Windows

| 依赖 | 安装方式 |
|---|---|
| Rust + Cargo | https://rustup.rs （选 MSVC toolchain） |
| Visual Studio 2019/2022 Build Tools | https://visualstudio.microsoft.com/visual-cpp-build-tools/ （勾选「使用 C++ 的桌面开发」） |
| WebView2 Runtime | Windows 10 1803+ / Windows 11 已内置，无需额外安装 |
| cargo-tauri | 首次运行构建脚本时自动安装 |

> **MSVC 版本无需手动指定**：构建脚本通过 `vswhere.exe` 自动定位已安装的 VS 工具链，
> 换机或升级 VS 后无需修改脚本。

---

## 二、版本管理

版本号以 `src-tauri/tauri.conf.json` 的 `version` 字段为**唯一真相源**。
UI 内的版本号在运行时通过 `window.__TAURI__.app.getVersion()` 自动读取，无需手动维护。

### 升级版本号

```bash
# macOS / Linux
./bump-version.sh 0.2.0

# Windows
bump-version.bat 0.2.0
```

脚本会同步以下文件：

| 文件 | 字段 |
|---|---|
| `src-tauri/tauri.conf.json` | `"version"` |
| `src-tauri/Cargo.toml` | `version =` |
| `package.json` | `"version"` |

升级后提交：

```bash
git add -A && git commit -m "chore: bump version to v0.2.0"
```

---

## 三、构建

### macOS

```bash
./build-mac.sh
```

| 选项 | 说明 |
|---|---|
| `./build-mac.sh` | 构建当前架构（aarch64 或 x86_64） |
| `./build-mac.sh --universal` | 交叉编译两架构并用 `lipo` 合并为 Universal Binary |

构建完成后脚本打印产物路径，例如：

```
[OK]    Portable  →  src-tauri/target/release/bundle/macos/Modem Cat.app  (28M)
[OK]    安装包    →  src-tauri/target/release/bundle/dmg/Modem Cat_0.2.0_aarch64.dmg  (12M)
```

### Windows

```bat
build-win.bat
```

`build-win.bat` 现在只是 `build.ps1` 的薄包装；实际构建入口和产物规则都以 `build.ps1` 为准，不再单独维护 fixed WebView2 runtime 逻辑。

若只想快速验证 Windows 正式构建，不生成便携 ZIP，可直接执行：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\build.ps1 -Quick
```

若要把 ADB 调试随 Windows 包交付，先把以下文件放到项目根目录的 `Sdk/`（或 `sdk/`）：

- `adb.exe`
- `AdbWinApi.dll`
- `AdbWinUsbApi.dll`

`build.ps1` 会在构建开始前把 ADB 组件复制到 `dist/` 根目录，方便便携版直接运行。`r26-cli` 固件 sidecar 额外依赖 **x86** `vcruntime140.dll`，构建脚本会从 Windows 构建机同步到 `src-tauri/resources/r26-runtime/`（供安装包使用）以及 `dist/vcruntime140.dll`（供便携版 / 直接双击验证使用）。WebView2 方面，当前 `tauri.conf.json` 使用 `embedBootstrapper`：安装包优先复用系统 WebView2，缺失时再自动拉起 bootstrapper；便携版则直接依赖目标机器已有的系统 WebView2。

Windows 安装包固定产出两个变体：

- `webview`：沿用 `embedBootstrapper`，适合常规交付
- `nowebview`：临时覆写为 `skip`，适合目标环境已统一预装/管控 WebView2 的场景

Windows 正式发版时，`dist/` 根目录只保留最终交付文件，不再创建 `dist/installer/`、`dist/portable/` 这类分类目录。

其中两个 portable ZIP 的根层公共内容都是：

- `modem-cat.exe`
- `adb.exe`
- `AdbWinApi.dll`
- `AdbWinUsbApi.dll`
- `r26-cli-x86_64-pc-windows-msvc.exe`
- `r26-cli.version.txt`
- `vcruntime140.dll`（x86，供 `r26-cli` 使用）

当前两个 portable ZIP 的差异如下：

| 包名 | WebView2 要求 | 适用场景 |
|---|---|---|
| `ModemCat_vX.Y.Z_portable-lite.zip` | 依赖系统 WebView2 | 保留给既有分发链路；当前与 `portable.zip` 同内容 |
| `ModemCat_vX.Y.Z_portable.zip` | 依赖系统 WebView2 | 默认便携交付物，适用于目标机器已具备系统 WebView2 |

额外说明：

- `dist/modem-cat.exe` 不再依赖同层 `webview2-runtime/`；它和两个 portable ZIP 一样，直接要求目标机器已有系统 WebView2
- 若外机提示缺少 WebView2，优先改发 `webview` 安装包，或先在目标机器安装系统 WebView2 Runtime
- `dist/r26-cli-x86_64-pc-windows-msvc.exe` 现在默认依赖同层的 `dist/vcruntime140.dll`；若漏掉它，主程序仍可启动，但固件下载会在启动 sidecar 时失败

任何 license / 设备激活工具都不属于最终用户桌面交付物，也不进入 `dist/` 或 portable ZIP。

`build.ps1` 本身不再为 app-local fixed runtime 预处理目录；安装包默认直接走 `embedBootstrapper`，便携版默认依赖系统 WebView2。`scripts/build-helper.ps1` 只会尽量复用 `%LOCALAPPDATA%\tauri` 中已缓存的 `MicrosoftEdgeWebview2Setup.exe`，减少重复下载。`scripts/setup-webview2.ps1` 也已经改成模式校验/清理脚本，不再把配置改回 `fixedRuntime`。WebView2 约束统一维护在本文件，不再单独维护重复的 `WEBVIEW2_BUILD.md`。

为保证远端 CI/CD 也能完整执行 Windows 打包，以下构建输入必须保留在 Git：

- `.cargo/config.toml`（Windows static CRT 配置）
- `src-tauri/binaries/r26-cli*`
- `src-tauri/resources/adb/`
- `src-tauri/resources/r26-runtime/README.md`

旧的 `webview2-runtime/` / `src-tauri/webview2-runtime/` 目录继续保持不进 Git，只作为历史缓存清理目标，不再是当前打包输入。`src-tauri/resources/r26-runtime/vcruntime140.dll` 也不进入 Git；它由 `build.ps1` / `scripts/build-helper.ps1` 从 Windows 构建机本地的 x86 VC 运行库目录临时同步，安装包运行 `r26-cli` 时再通过 `PATH` 注入给 sidecar。

如果 CI 使用 Ubuntu runner，则不要指望它替代 Windows 机器去产出完整 Windows 安装包。当前工程的 MSI / 常规 Tauri Windows 打包应放在 Windows runner 上执行；Ubuntu runner 更适合跑测试、文档校验和非安装包任务。

以下目录属于机器本地代理 / 索引 / 记忆缓存，必须继续留在 `.gitignore`，不要提交到远端：

- `.codegraph/`
- `.gitnexus/`
- `.specify/`
- `.understand-anything/`
- `.workbuddy/`

构建完成后脚本打印产物路径，例如：

```
[OK]   Portable  >  src-tauri\target\release\modem-cat.exe
[OK]   安装包    >  src-tauri\target\release\bundle\msi\Modem Cat_0.2.0_x64_en-US.msi
[OK]   安装包    >  src-tauri\target\release\bundle\nsis\Modem Cat_0.2.0_x64-setup.exe
```

---

## 四、发版完整流程

```bash
# 1. 升级版本
./bump-version.sh 0.2.0

# 2. 提交版本变更
git add -A && git commit -m "chore: bump version to v0.2.0"

# 3. 构建
./build-mac.sh            # 在 macOS 机器上执行
build-win.bat             # 在 Windows 机器上执行
```

---

## 五、开发构建（仅验证编译）

不产出安装包，仅验证代码能编译通过：

```bash
cargo check --workspace          # 最快，仅类型检查，不链接
cargo build --workspace          # Debug 构建
cargo test --workspace           # 运行所有 Rust 测试
bash scripts/verify-docs.sh      # 文档 / 约束 / 护栏一致性检查
```

---

## 六、常见问题

### MQTT 开关启用后立即报配置错误

这是预期行为。当前代码禁止内置公开默认 broker / 凭据；若要启用 MQTT，需先显式设置：

- `MODEM_CAT_MQTT_HOST`
- `MODEM_CAT_MQTT_PORT`
- `MODEM_CAT_MQTT_USERNAME`（可选）
- `MODEM_CAT_MQTT_PASSWORD`（可选，但若设置用户名则必须同时设置）

### macOS: `cargo tauri build` 提示找不到 icon

检查 `src-tauri/icons/` 目录是否包含所有 `tauri.conf.json` 里声明的图标文件。

### Windows: 找不到 MSVC 工具链

确认 Visual Studio 安装时勾选了「**使用 C++ 的桌面开发**」工作负载。
安装完成后重新运行 `build-win.bat`，脚本会通过 `vswhere.exe` 自动定位。

### Windows: `.msi` 安装后提示"Windows 已保护你的电脑"

应用尚未进行代码签名。点击「更多信息」→「仍要运行」即可。
正式发布前可在 `tauri.conf.json` 的 `bundle.windows` 节配置签名证书。

### Portable `.exe` 报错找不到 WebView2

当前便携版直接依赖系统 WebView2。Windows 10 1803 以下版本通常需要手动安装 WebView2 Runtime：
https://developer.microsoft.com/microsoft-edge/webview2/

若是双击 `modem-cat.exe` 后完全没有界面、也没有弹窗，先区分两类情况：

- 使用 `portable.zip` / `portable-lite.zip`：确认目标机器已经安装系统 WebView2 Runtime
- 使用 `webview` 安装包：确认安装时没有被安全策略拦截 bootstrapper；如仍失败，再看启动日志

所有启动期错误、panic 和 Tauri runtime 初始化失败，都会追加到：

```text
%LOCALAPPDATA%\Modem Cat\logs\startup.log
```

若目标机器没有 `LOCALAPPDATA`，运行时会依次回退到 `%TEMP%\Modem Cat\logs\startup.log`，再回退到当前工作目录下的 `Modem Cat\logs\startup.log`。

日志开头还会主动记录：

- `modem-cat.exe` 的实际路径
- 当前工作目录
- 同层 `webview2-runtime/` 目录是否存在（仅用于识别旧 fixed runtime 残留）
- `webview2-runtime/msedgewebview2.exe` 是否存在（仅用于识别旧 fixed runtime 残留）

所以外机若继续报 WebView2 或“点了没反应”，优先把这个日志带回来，就能先判断是打包布局、安装目录还是别的启动期错误。

### ADB 调试页提示未找到 ADB 组件

检查项目根目录的 `Sdk/`（或 `sdk/`）是否已放入 `adb.exe`、`AdbWinApi.dll`、`AdbWinUsbApi.dll`，并重新执行 Windows 构建。
