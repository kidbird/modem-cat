# 技术栈

> 最近更新：2026-06-16

## 1. 核心技术

### 1.1 桌面应用框架
- **Tauri 2.10.3**（Rust 编写的桌面应用框架）
  - features: `custom-protocol`, `tray-icon`
  - 能力: 窗口管理、系统托盘、IPC 通信
  - `withGlobalTauri = true`：前端直接用 `window.__TAURI__.core.invoke`

### 1.2 前端技术
- **HTML5 + CSS3 + Vanilla JavaScript**（无框架）
- **已拆分为 3 个文件**：
  - `src/desktop/index.html`（含 10 个 page 容器）
  - `src/desktop/app.js`（交互逻辑）
  - `src/desktop/styles.css`（主题样式）
- 当前 Tauri 实际入口是 `index.html`。
- 状态管理：→ `AGENTS.md §2`
- DOM 缓存：`$.dom` 一次预查常用 ID
- 通过 Tauri IPC 调用后端命令

### 1.3 后端技术
- **Rust 2021 Edition**
- **Tokio**：异步运行时（所有 IPC 走 `tokio::task::spawn_blocking`）
- **serialport v4**：串口通信
- **reqwest 0.12**：HTTP 客户端（工厂模式设备通信）
- **chrono 0.4**：时间处理（SN 生成、CSV 记录）
- **winreg 0.56**：Windows 注册表访问（端口友好名）
- **modem-hal**：项目内共享 Rust HAL（厂商识别、传输抽象、解析能力）
- **modem-license**：项目内 License 验签 crate（Ed25519）
- **错误处理**：全程 `Result<T, String>`（**未引入 thiserror**，见 [REVIEW.md#17]）

### 1.4 HAL 拆分
- `modem-hal/src/transport/` — AtTransport trait + Serial/TCP 实现
- `modem-hal/src/modem_vendor.rs` — ModemVendor trait
- `modem-hal/src/modem_factory.rs` — `AT+CGMM` → 厂商识别
- `modem-hal/src/vendors/quectel/` — Quectel 全家（Qualcomm / UniSoc 二态）

### 1.5 固件下载 Sidecar
- **r26-cli**（32-bit Windows 可执行文件）
  - 位置：`src-tauri/binaries/r26-cli-x86_64-pc-windows-msvc.exe`
  - 功能：解析 PAC 文件、驱动 DLFrame.dll 进行 Unisoc 模组刷写
  - 通信方式：stdout JSON 事件流 → Tauri 后端转发为 `firmware-event` 事件

## 2. 依赖关系

```
src/desktop/{index.html, app.js, styles.css}
        |
     invoke() / listen()
        |
src-tauri/src/{lib.rs, ports.rs, main.rs, license.rs, factory.rs, dloader.rs}
        |
   Box<dyn ModemVendor>  (lib.rs:AppState.vendor)
        |
   modem-hal/src/
        |
   Box<dyn AtTransport>  (lib.rs:AppState.transport, 含 LoggingTransport 装饰)
        |
   serialport / TcpStream / reqwest (HTTP)
        |
   串口 / TCP / HTTP → 5G Modem

   r26-cli sidecar (32-bit) → DLFrame.dll → Unisoc 模组
```

## 3. 构建环境

### 3.1 编译工具
- **Rust 1.80+**（用 edition 2021）
- **cargo-tauri v2**（首次构建时自动安装）
- **MSVC Build Tools 2019/2022**（Windows，由 `build-win.bat` 自动定位）
- **Xcode CLI Tools**（macOS）

### 3.2 构建脚本

```bash
./build-mac.sh          # macOS：产出 .app + .dmg
build-win.bat           # Windows：产出 .exe + .msi
./bump-version.sh X.Y.Z # 升级版本号（同步所有文件）
```

详细说明见 [BUILD.md](BUILD.md)。

## 4. 运行模式

→ `ARCHITECTURE.md §8`

## 5. 关键 Cargo 依赖（节选）

| Crate | 版本 | 用途 |
|---|---|---|
| `tauri` | 2 | 桌面框架 |
| `tauri-plugin-shell` | 2 | shell 插件（sidecar 管理） |
| `tauri-plugin-dialog` | 2 | 文件选择对话框 |
| `tauri-plugin-single-instance` | 2 | 单实例 |
| `tokio` | 1 (rt, rt-multi-thread) | 异步运行时 |
| `serialport` | 4 | 串口 |
| `reqwest` | 0.12 (json, rustls-tls) | HTTP 客户端（工厂模式） |
| `chrono` | 0.4 | 时间处理 |
| `serde` / `serde_json` | 1 | 序列化 |
| `winreg` | 0.56 | Windows 注册表（仅 windows target） |
| `modem-hal` | path = "../modem-hal" | 项目内 HAL |
| `modem-license` | path = "../modem-license" | License 验签（Ed25519） |

`modem-hal` 内部：`serialport` 4（optional, default feature）、`napi` 2 + `napi-derive` 2（optional, napi-feature）。
