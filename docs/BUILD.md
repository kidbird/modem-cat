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

若只想快速验证 Windows 正式构建，不生成便携 ZIP，可直接执行：

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1 -Quick
```

若要把 ADB 调试随 Windows 包交付，先把以下文件放到项目根目录的 `Sdk/`（或 `sdk/`）：

- `adb.exe`
- `AdbWinApi.dll`
- `AdbWinUsbApi.dll`

`scripts/build-release.ps1` 会在构建开始前自动把它们同步到 `src-tauri/resources/adb/`，再由 `tauri.conf.json > bundle.resources` 打进安装包；同时也会把这 3 个文件复制到 `dist/` 根目录，方便便携版直接运行。

Windows 正式发版时，`dist/` 根目录只保留最终交付文件，不创建子目录。WebView2 fixed runtime 不会展开到 `dist/webview2-runtime/`，而是只打进：

- MSI / NSIS 安装包
- `ModemCat_vX.Y.Z_portable-lite.zip`
- `ModemCat_vX.Y.Z_portable.zip`

其中两个 portable ZIP 的根层公共内容都是：

- `modem-cat.exe`
- `adb.exe`
- `AdbWinApi.dll`
- `AdbWinUsbApi.dll`
- `r26-cli-x86_64-pc-windows-msvc.exe`
- `r26-cli.version.txt`

差异如下：

| 包名 | 是否包含 `webview2-runtime/` | 适用场景 |
|---|---|---|
| `ModemCat_vX.Y.Z_portable-lite.zip` | 否 | 目标机器已安装系统 WebView2 Runtime，希望下载体积更小、分发更快 |
| `ModemCat_vX.Y.Z_portable.zip` | 是 | 目标机器可能没有系统 WebView2，需要离线即开即用 |

任何 license / 设备激活工具都不属于最终用户桌面交付物，也不进入 `dist/` 或 portable ZIP。

`build-release.ps1` 本身不会联网下载 WebView2；它只检查本地 `webview2-runtime/` 是否已准备好。首次环境准备若缺少该目录，再单独执行 `scripts/setup-webview2.ps1`。WebView2 约束统一维护在本文件，不再单独维护重复的 `WEBVIEW2_BUILD.md`。

为保证远端 CI/CD 也能完整执行 Windows 打包，以下构建输入必须保留在 Git：

- `.cargo/config.toml`（Windows static CRT 配置）
- `src-tauri/binaries/r26-cli*`
- `src-tauri/resources/adb/`

`webview2-runtime/` 不进入 Git。若要做 fixed WebView2 离线打包，必须在执行构建的机器上预先准备该目录；它属于应用私有运行时，不覆盖目标机器系统 WebView2。

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

Windows 10 1803 以下版本需手动安装 WebView2 Runtime：
https://developer.microsoft.com/microsoft-edge/webview2/

### ADB 调试页提示未找到 ADB 组件

检查项目根目录的 `Sdk/`（或 `sdk/`）是否已放入 `adb.exe`、`AdbWinApi.dll`、`AdbWinUsbApi.dll`，并重新执行 Windows 构建。
