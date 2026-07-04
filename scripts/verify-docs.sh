#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

fail=0
pass=0

check() {
  local desc="$1"
  local cmd="$2"
  if eval "$cmd" >/dev/null 2>&1; then
    echo "  ✓ $desc"
    pass=$((pass + 1))
  else
    echo "  ✗ $desc"
    echo "    → $cmd"
    fail=$((fail + 1))
  fi
}

echo "=== verify-docs.sh ==="
echo

echo "[1] 入口与执行面"
check "AGENTS 入口指向 CONTEXT_PACK" "grep -q 'docs/CONTEXT_PACK.md' AGENTS.md"
check "AGENTS 路由包含 handlers/connection/monitor" "grep -q 'handlers,connection,monitor,mqtt' AGENTS.md"
check "ARCHITECTURE 记录 handlers.rs" "grep -q 'handlers.rs' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 connection.rs" "grep -q 'connection.rs' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 monitor.rs" "grep -q 'monitor.rs' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 debug_terminal.rs" "grep -q 'debug_terminal.rs' docs/ARCHITECTURE.md"
check "CODE_MAP 说明执行面分散在模块文件中" "grep -q 'handlers.rs' docs/CODE_MAP.md && grep -q 'connection.rs' docs/CODE_MAP.md"
check "CALL_FLOW 指向 connection.rs::auto_connect_at" "grep -q 'connection.rs::auto_connect_at' docs/CALL_FLOW.md"

echo "[2] 死路径与过时引用"
check "owner 文档不再引用已删除的 commands.rs" "! grep -n 'commands.rs' AGENTS.md CLAUDE.md docs/CONTEXT_PACK.md docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/REVIEW.md docs/BUILD.md docs/TECH_STACK.md docs/CODING.md docs/README.md 2>/dev/null"
check "owner 文档不再写死旧行数 1142/1556/6479" "! grep -nE '1142|1556|6479' AGENTS.md CLAUDE.md docs/CONTEXT_PACK.md docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/REVIEW.md docs/BUILD.md docs/TECH_STACK.md docs/CODING.md docs/README.md 2>/dev/null"
check "owner 文档不再描述最终用户 License / Factory 菜单入口" "! grep -nE 'show-load-license|show-license-status|#page-factory|factory_write_sn_to_device|get_license_status|load_license_file' docs/ARCHITECTURE.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/BUILD.md docs/TECH_STACK.md 2>/dev/null"
check "本地 agent/tool 目录不再被 Git 跟踪" "[ -z \"$(git ls-files .agents .codegraph .gitnexus .specify .understand-anything .workbuddy opencode.json 2>/dev/null)\" ]"
check ".gitignore 继续忽略本地 agent/tool 目录" "grep -q '^.agents/$' .gitignore && grep -q 'opencode.json' .gitignore && grep -q '\\.codegraph/' .gitignore && grep -q '\\.gitnexus/' .gitignore && grep -q '\\.specify/' .gitignore && grep -q '\\.understand-anything/' .gitignore && grep -q '\\.workbuddy/' .gitignore"
check ".gitignore 继续忽略 fixed WebView2 runtime" "grep -q '^webview2-runtime/$' .gitignore"
check "fixed WebView2 runtime 不进入 Git" "[ -z \"$(git ls-files webview2-runtime 2>/dev/null)\" ]"
check "仓库根 specs/ 不再作为正式需求目录" "grep -q '^/specs/$' .gitignore && [ ! -e specs ]"

echo "[3] 凭据与真相源"
check "WebSocket 不再补 admin 默认凭据" "! grep -R -n 'unwrap_or(\"admin\")' src-tauri/src modem-hal/src 2>/dev/null"
check "MQTT 不再硬编码旧 broker/凭据" "! grep -R -n '82\\.157\\.177\\.161|iot_client|6yvqYJ6Y9dAa9p' src-tauri/src 2>/dev/null"
check "前端不再把 mqtt_enabled 写进 localStorage" "! grep -nE 'localStorage.*mqtt_enabled|mqtt_enabled.*localStorage' src/desktop/app.js"
check "TECH_STACK 记录 MQTT 显式配置约束" "grep -q 'broker / port / 认证信息必须显式提供' docs/TECH_STACK.md"

echo "[4] 严格失败语义"
check "feature toggle 查询不再 Err(_) => false" "! grep -n 'Err(_) => false' modem-hal/src/vendors/quectel/mod.rs"
check "NAT/live QCFG 查询不再用 parse_qcfg_int(...).unwrap_or(0) 伪默认" "! grep -nE 'parse_qcfg_int\\(&.*\\)\\.unwrap_or\\(0\\)' modem-hal/src/vendors/quectel/mod.rs"
check "运行时锁路径不再使用 lock().unwrap()" "! grep -R -n 'lock().unwrap()' src-tauri/src modem-hal/src 2>/dev/null"

echo "[5] 文档验证入口"
check "BUILD 文档包含 verify-docs" "grep -q 'verify-docs.sh' docs/BUILD.md"
check "CODING 文档包含 verify-docs" "grep -q 'verify-docs.sh' docs/CODING.md"
check "AGENTS 验证基线包含 cargo test / cargo build / verify-docs" "grep -q 'cargo test --workspace' AGENTS.md && grep -q 'cargo build -p modem-hal' AGENTS.md && grep -q 'verify-docs.sh' AGENTS.md"
check "CODE_MAP 记录 ADB / SSH 调试 IPC" "grep -q 'start_adb_session' docs/CODE_MAP.md && grep -q 'start_ssh_session' docs/CODE_MAP.md"
check "BUILD 记录 ADB 资源目录" "grep -q 'src-tauri/resources/adb/' docs/BUILD.md && grep -q 'Sdk/' docs/BUILD.md"
check "BUILD 记录 fixed WebView2 非跟仓约束" "grep -q 'webview2-runtime/' docs/BUILD.md && grep -q '不进入 Git' docs/BUILD.md"
check "BUILD 记录双 portable ZIP 与适用场景" "grep -q 'portable-lite.zip' docs/BUILD.md && grep -q '目标机器已安装系统 WebView2' docs/BUILD.md && grep -q '目标机器可能没有系统 WebView2' docs/BUILD.md"
check "BUILD 记录 portable ZIP 不包含 license / 激活工具" "grep -q '任何 license / 设备激活工具都不属于最终用户桌面交付物' docs/BUILD.md && ! grep -q 'license-gen' docs/BUILD.md"
check "TECH_STACK 记录 Windows 打包资产边界" "grep -q '.cargo/config.toml' docs/TECH_STACK.md && grep -q 'src-tauri/resources/adb/' docs/TECH_STACK.md"
check "根 README 记录架构 / 环境 / 构建 / 发版内容" "[ -e README.md ] && grep -q '## 架构概览' README.md && grep -q '## 开发环境' README.md && grep -q '## 构建与验证' README.md && grep -q '## 发版建议' README.md"
check "README 记录 docs/specs 需求文档位置" "grep -q 'docs/specs/' docs/README.md || grep -q 'specs/001-modem-debug-tool' docs/README.md"
check "README 记录构建产物与适用场景" "grep -q 'portable-lite.zip' docs/README.md && grep -q '完整免安装包' docs/README.md && grep -q 'NSIS 安装包' docs/README.md"
check "不再维护重复的 WEBVIEW2_BUILD 文档" "[ ! -e docs/WEBVIEW2_BUILD.md ]"
check "主工程不再引用 modem-license" "! grep -q 'modem-license' src-tauri/Cargo.toml && ! grep -q 'modem_license' src-tauri/src/lib.rs && [ ! -e src-tauri/src/license.rs ]"
check "仓库不再保留 license 工具目录" "[ ! -e modem-license ] && [ ! -e tools/license-gen ]"

echo
echo "=== 结果: $pass 通过 / $fail 失败 ==="
if [ "$fail" -gt 0 ]; then
  exit 1
fi
