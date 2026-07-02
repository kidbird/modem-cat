# 最终用户版菜单收敛与 ADB/SSH 调试设计

**日期**：2026-07-02
**状态**：待评审

## 目标

把当前偏工程/工厂混用的桌面工具收敛成面向最终用户的版本，同时补齐两条独立调试能力：

1. 删除最终用户不应看到的工厂模式与激活相关入口
2. 固件下载改为默认可见，不再依赖 License 可见性控制
3. 新增 `ADB 调试` 菜单，提供内嵌 `adb shell` 终端
4. 新增 `SSH 调试` 菜单，提供内嵌 SSH shell 终端
5. 保持现有 live modem AT 主路径不变，不新增第二条 AT 队列

## 业务范围

### 包含

- 左侧导航结构调整
- 帮助/菜单中的激活、License 最终用户入口清理
- 固件下载默认显示
- ADB 调试页
- SSH 调试页
- Windows 下 ADB 随包分发
- SSH 用户名保存、密码不保存
- 相关 owner 文档与 `scripts/verify-docs.sh` 同步更新

### 不包含

- 保留工厂模式的任何最终用户可见入口
- 设备激活、写 SN、产品配置、生产记录等工厂能力复用
- macOS ADB 可用性保证
- 基于系统外部终端窗口的调试流程
- 复用 `AppState.transport` 承载 ADB/SSH 交互

## 当前问题

- 左侧导航仍保留 `工厂模式`，并受 License 控制显示
- `固件下载` 仍受 License 控制显示，不符合最终用户版目标
- 当前只有 `AT调试` 终端，没有 ADB/SSH 的独立调试入口
- 已有网卡枚举能力仅用于连接页，没有形成 SSH 调试主路径
- 帮助/菜单仍存在 License 加载与状态入口，容易把最终用户产品重新带回激活/工厂心智

## 设计总览

最终方案分成三条互不混淆的主路径：

1. **live modem AT 主路径**
   前端现有页面 -> Tauri IPC -> `AppState.transport` -> `AtTransport::send_at`
2. **ADB 调试主路径**
   `ADB 调试` 页 -> debug terminal IPC -> Windows 随包 `adb.exe` -> `adb shell`
3. **SSH 调试主路径**
   `SSH 调试` 页 -> debug terminal IPC -> Rust 原生 SSH shell channel

ADB/SSH 都属于宿主机上的独立调试会话，不进入 `AppState.transport`，不复用 `send_raw_at`，不写入 AT 日志环。

## 前端设计

### 1. 左侧导航调整

最终用户版左侧导航调整为：

- 模组状态
- 蜂窝网络
- IP 信息
- AT调试
- ADB 调试
- SSH 调试
- 系统信息
- 情景模式
- 在线监控
- AT手册
- 系统设置
- 固件下载

删除：

- 工厂模式

保留但语义调整：

- `固件下载` 默认显示，不再依赖 License 状态

### 2. 最终用户入口清理

最终用户主路径中移除以下激活/工厂相关入口：

- 左侧 `工厂模式`
- 工厂页整块 DOM 与对应前端逻辑
- 帮助或菜单中的激活说明入口
- 托盘/菜单中的 `加载 License...`
- 托盘/菜单中的 `License 状态`
- 前端用于控制工厂/固件导航可见性的 License 监听主逻辑

保留边界：

- 若后端 License 模块仍被其他内部功能依赖，可暂时保留模块代码，但不再暴露最终用户入口

### 3. ADB 调试页

页面元素：

- 状态提示区
- 说明文案区
- 连接按钮
- 断开按钮
- 终端输出区
- 命令输入框

交互规则：

1. 进入页面时先读取当前模组功能开关状态
2. 若 `adb=false`，页面显示明确提示：
   `ADB 未开启，请先到“系统信息”页开启 ADB，并重启设备后重新连接。`
3. 此页**不提供** ADB 开关按钮
4. 用户点击连接后，前端发起 `start_adb_session`
5. 连接成功后进入交互式终端
6. 用户点击断开或页面销毁时关闭会话

平台规则：

- Windows：显示并可用
- 非 Windows：隐藏 `ADB 调试` 菜单，不暴露半可用入口

### 4. SSH 调试页

页面元素：

- 有线网卡下拉框
- 设备 IP 输入框
- 用户名输入框
- 密码输入框
- 连接按钮
- 断开按钮
- 终端输出区
- 命令输入框

交互规则：

1. 进入页面时自动调用网卡枚举
2. 下拉框只显示有线网卡，不显示 Wi-Fi、蓝牙、VPN、虚拟网卡
3. 用户选择网卡后，设备 IP 默认填入该网卡的网关地址
4. 用户可手动覆盖 IP
5. 用户名可保存并自动带出
6. 密码每次必须重新输入，不落盘、不补默认值
7. 点击连接后创建 SSH shell 会话，成功后进入交互式终端
8. 断开后显式关闭会话并清空密码输入框

### 5. 终端 UI 复用策略

`ADB 调试` 与 `SSH 调试` 共用一套终端 UI 结构和大部分样式，沿用现有 `AT调试` 页的交互习惯：

- 输出区滚动
- 输入后回车发送
- 连接/断开状态提示
- 错误行与普通输出分色

但三者数据主路径必须分开：

- `AT调试` -> `send_raw_at`
- `ADB 调试` -> debug terminal session
- `SSH 调试` -> debug terminal session

禁止把 ADB/SSH 终端拼接进 AT 命令日志。

## 后端设计

### 1. 新增 debug terminal 模块

新增独立模块，例如：

- `src-tauri/src/debug_terminal.rs`

职责：

- 维护 ADB/SSH 调试会话
- 启动/关闭会话
- 接收前端输入并写入会话 stdin
- 持续收集 stdout/stderr 并提供给前端
- 管理会话状态与平台判定

该模块不参与：

- `AppState.transport`
- `ModemVendor`
- `AtTransport::send_at`
- MQTT / monitor / factory / dloader

### 2. 状态 owner

新增单一 owner，例如 `DebugTerminalState`，由 Tauri `manage()` 挂入应用状态。

它至少包含：

- 当前活动会话单槽位
- 会话类型（adb / ssh）
- 会话 stdin writer
- 输出缓冲或事件转发器
- 保存的 SSH 偏好（用户名、上次网卡、上次 IP）

约束：

- ADB 与 SSH 不共享同一个 OS 进程
- 全应用同一时刻只允许一条活跃调试会话
- 会话异常退出时必须及时通知前端

### 3. ADB 会话路径

Windows 下使用随包 `adb.exe` 启动子进程：

```text
start_adb_session
  -> resolve bundled adb path
  -> spawn `adb shell`
  -> pipe stdin/stdout/stderr
  -> push output to frontend terminal
```

设计约束：

- 优先使用应用随包的 adb，不依赖系统 PATH
- 若 adb 缺失，明确报错，不做 fallback 猜测
- ADB 会话不写入 AT 环形日志
- 断开时显式结束子进程

### 4. SSH 会话路径

SSH 使用 Rust 原生 SSH 客户端库建立连接并打开 shell channel：

```text
start_ssh_session
  -> validate ip/username/password
  -> connect tcp 22
  -> authenticate with username + password
  -> open shell channel
  -> bridge stdin/stdout/stderr to frontend terminal
```

设计约束：

- 不调用系统 `ssh.exe`
- 用户名/密码必须来自显式输入
- 不保存密码
- 不提供公开默认用户名或密码
- 连接失败直接报错，不做第二条认证 fallback

### 5. IPC 设计

新增一组独立 IPC，例如：

- `get_debug_terminal_capabilities`
- `list_debug_network_adapters`
- `get_debug_terminal_prefs`
- `save_debug_terminal_prefs`
- `start_adb_session`
- `start_ssh_session`
- `write_debug_terminal_input`
- `close_debug_terminal_session`

并统一通过事件推送输出：

- `debug-terminal-output`

要求：

- ADB/SSH 共用一套会话生命周期接口
- 连接、收发、关闭必须只有这一条主路径
- 旧工厂页和 License 最终用户入口删掉后，不能残留第二条前端调用路径

## 配置与保存策略

### 保存项

允许保存到本机配置：

- SSH 用户名
- 上次选择的网卡名称
- 上次手动确认的设备 IP

### 禁止保存项

- SSH 密码
- ADB shell 历史中的敏感凭据
- 任何公开默认 token / password

### 读取规则

- 若保存的网卡名称仍存在，则恢复该选中项
- 若保存的网卡已不存在，则只恢复用户名和上次 IP，不自动猜新网卡
- 密码框每次启动或断开后都为空

## 打包与平台策略

### Windows

- 把 `adb.exe` 及其运行依赖作为随包二进制一并分发
- 分发产物仍统一放到 `dist/` 根目录
- 运行时从应用资源或外部二进制目录解析 adb 路径

### macOS / 非 Windows

- 本轮不保证 ADB 可用
- 文档、UI 和实现保持一致：隐藏 `ADB 调试` 菜单，不暴露未支持入口

### SSH

- 平台无强绑定，优先按跨平台 Rust 实现设计
- 但本轮验证重点先保证 Windows 桌面可用

## 删改范围

### 前端

- `src/desktop/index.html`
- `src/desktop/app.js`
- `src/desktop/styles.css`
- `src/desktop/js/i18n.js`
- `src/desktop/js/debug-terminal.js`

### 后端

- `src-tauri/src/lib.rs`
- `src-tauri/src/connection.rs`
- 新增 `src-tauri/src/debug_terminal.rs`
- 新增配置持久化辅助模块

### 文档

- `docs/ARCHITECTURE.md`
- `docs/CODE_MAP.md`
- `docs/CALL_FLOW.md`
- `docs/TECH_STACK.md`
- `docs/REVIEW.md`（若实现过程中顺手收敛相关偏差）
- `scripts/verify-docs.sh`

## 测试与验证

### 行为测试

1. 固件下载导航在无 License 条件下默认可见
2. 工厂模式导航和页面入口完全消失
3. ADB 页在 `adb=false` 时提示用户去系统信息页开启
4. ADB 页在 Windows 下能创建和关闭 shell 会话
5. SSH 页能正确列出有线网卡
6. SSH 页切换网卡后自动带出该网关地址
7. SSH 用户名能保存并恢复
8. SSH 密码不会保存，重启后为空
9. SSH 连接失败时返回明确错误
10. ADB/SSH 断开后会话资源被回收

### 回归验证

- `cargo test --workspace`
- `cargo build -p modem-hal`
- `bash scripts/verify-docs.sh`

## 风险与注意事项

- **Windows ADB 打包路径**：必须保证随包 adb 的解析路径在开发和发布态都一致，否则会出现“开发可用、安装包不可用”
- **交互式终端兼容性**：ADB 子进程和 SSH shell channel 的输出节奏不同，前端终端组件要按“流式输出”设计，不能假定按行齐整到达
- **会话状态清理**：窗口切页、重复点击连接、异常退出时都要防止残留孤儿会话
- **唯一业务主路径**：一旦 debug terminal 模块建成，前端只能通过这一条路径收发 ADB/SSH 数据，禁止再长出外部窗口 fallback
- **文档同步**：最终用户版的导航、菜单和调试主路径变化必须同步到 owner 文档，否则本次改动不算完成

## 实施建议

推荐按下面顺序实现：

1. 先删除工厂模式和 License 最终用户入口，打开固件下载默认显示
2. 再抽出共用终端 UI 容器
3. 先落 ADB Windows 会话路径
4. 再落 SSH 原生会话路径与偏好保存
5. 最后补文档与验证脚本
