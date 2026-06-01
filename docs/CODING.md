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
- 用户未传敏感字段时返回错误，不得 fallback 到 `"12345678"`、`"admin"`、空密码等默认值。
- 所有进入 AT 日志的命令必须经过 `redact_at_command`；新增敏感 AT 指令时同步补 redaction 测试。

## 3. 前端入口

- Tauri 当前加载 `src/desktop/index.html`。
- 修改前端行为时，先确认 `index.html` 是否引用外部 `app.js` / `styles.css`。如果没有引用，实际运行代码在 `index.html` 的内联 `<script>` / `<style>` 中。
- 若保留拆分文件作为同步副本，改同名函数时必须同步修改，避免文档和检查脚本只验证未加载文件。

## 4. 文档与检查

- 结构性改动后同步更新 `docs/ARCHITECTURE.md`、`docs/CODE_MAP.md`、`docs/CALL_FLOW.md`。
- 安全/稳定性问题状态同步更新 `docs/REVIEW.md`。
- 提交前至少运行：

```bash
cargo test --workspace
bash scripts/verify-docs.sh
```
