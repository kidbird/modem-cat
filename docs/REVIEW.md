# REVIEW.md

> 最近更新：2026-06-18
> 只记录当前主线仍然存在的 live 技术债与文档/代码偏差；历史已修问题不再在本文件重复维护。

## High Priority

### 1. transport 超时后可能把截断响应当成功

- `serial.rs` / `tcp.rs` / `websocket.rs` 现在都可能在“收到部分字节后超时”时返回 `Ok`
- 风险：上层 parser 会把链路抖动误判成真实状态
- 处理原则：实时状态读取应把“不完整响应”视为错误，而不是伪成功

## Medium Priority

### 2. 自动连接缺少 in-flight 保护

- 热插拔自动重连与人工连接触发之间缺少统一“连接中”状态
- 风险：旧断开收尾打掉新连接，或重复发起并发连接任务

## Cleanup Rules

- 触碰 live AT 队列、实时状态读取、vendor 检测、连接路径时，必须同步清理 fallback 和死代码
- 不要把技术债写成正式合同；合同进 `ARCHITECTURE.md` / `AT_COMMANDS.md`，现状偏差留在本文件
- 清理完的债务应直接从本文件移除，不保留“已修历史年表”
