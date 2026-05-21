# 技术栈

## 1. 核心技术

### 1.1 桌面应用框架
- **Tauri v2.10.3**: Rust 编写的桌面应用框架
  - 特性: `custom-protocol`, `tray-icon`
  - 能力: 窗口管理、系统托盘、IPC 通信

### 1.2 前端技术
- **HTML5 + CSS3 + Vanilla JavaScript**: 单文件前端（`src/desktop/index.html`）
  - 无前端框架依赖
  - 通过 Tauri IPC 调用后端命令

### 1.3 后端技术
- **Rust 2021 Edition**
- **Tokio**: 异步运行时
- **serialport v4**: 串口通信
- **winreg**: Windows 注册表访问（端口友好名）
- **modem-hal**: 项目内共享 Rust HAL（厂商识别、传输抽象、解析能力）

## 2. 依赖关系

```
src/desktop/index.html
        |
     invoke()
        |
src-tauri/src/lib.rs
        |
  at_adapter.rs / at_parser.rs
        |
      modem-hal
        |
   串口/TCP 与 5G Modem
```

## 3. 构建环境

### 3.1 编译工具
- **Rust 1.80+**
- **cargo-tauri v2**（首次构建时自动安装）
- **MSVC Build Tools 2019/2022**（Windows，由 `build-win.bat` 自动定位）
- **Xcode CLI Tools**（macOS）

### 3.2 构建脚本

```bash
./build-mac.sh          # macOS：产出 .app + .dmg
build-win.bat           # Windows：产出 .exe + .msi
./bump-version.sh X.Y.Z # 升级版本号（同步所有文件）
```

详细说明见 [docs/BUILD.md](BUILD.md)。

## 4. 当前模式

- 项目当前仅支持 **Desktop 模式**。
- 原 CLI 模式已移除，不再作为构建或发布目标。
