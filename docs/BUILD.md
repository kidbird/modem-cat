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
```

---

## 六、常见问题

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
