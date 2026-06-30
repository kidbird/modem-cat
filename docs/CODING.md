# 编码规范

> 最近更新：2026-06-01

## 1. AT 输入校验

AT 校验分两类，不能混用：

| 场景 | 使用函数 | 规则 |
|---|---|---|
| 拼进带引号 AT 参数的用户输入 | `validate_at_string` | 禁止 `"`、`\r`、`\n` 和控制字符，防止逃逸字符串或注入新命令 |
| `send_raw_at` 发送的完整 AT 命令 | `validate_raw_at_command` | 允许合法双引号语法；禁止 `\r`、`\n`、控制字符和 `;` 命令串联；必须以 `AT` 开头 |
| PDP CID | `validate_cid` | 仅允许 1-16 |

示例：

```rust
// 参数：会被 format!(r#"AT+QSIMLOCK="PN","{}""#, password) 包进引号
validate_at_string(&password)?;

// 完整命令：AT+QCFG="ims" 里的引号是合法 AT 语法
validate_raw_at_command(&command)?;
```

## 2. 敏感信息

- 不允许给 PLMN lock、APN、HTTP auth、token 等敏感字段设置公开默认值。
- 用户未传敏感字段时返回错误或保持未配置，不得 fallback 到 `"12345678"`、`"admin"`、空密码、固定 MQTT broker 凭据等默认值。
- 所有进入 AT 日志的命令必须经过 `redact_at_command`；新增敏感 AT 指令时同步补 redaction 测试。

## 3. 前端入口

- Tauri 加载 `src/desktop/index.html`，前端逻辑拆分到 `app.js` + `styles.css`（通过 `<link>` / `<script src>` 引用）。
- 修改前端行为时，根据改动类型编辑对应文件；纯样式改 `styles.css`，逻辑改 `app.js`。

## 4. 文档与检查

- 结构性改动后同步更新 `docs/ARCHITECTURE.md`、`docs/CODE_MAP.md`、`docs/CALL_FLOW.md`。
- 安全/稳定性问题状态同步更新 `docs/REVIEW.md`。
- 修改认证、MQTT、WebSocket 连接行为时同步更新 `docs/TECH_STACK.md`、`docs/ARCHITECTURE.md`、`scripts/verify-docs.sh`。
- 提交前至少运行：

```bash
cargo test --workspace
bash scripts/verify-docs.sh
```
