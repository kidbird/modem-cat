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
check "ARCHITECTURE 记录 USB VID/PID 先判型" "grep -q 'VID/PID' docs/ARCHITECTURE.md && grep -q 'AT+CGMM' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 connected_usb_ids 真相源" "grep -q 'connected_usb_ids' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 AT 队列 10ms 最小间隔" "grep -q 'LoggingTransport::send_at' docs/ARCHITECTURE.md && grep -q '10ms' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 port-changed 结构化条目" "grep -q 'port-changed' docs/ARCHITECTURE.md && grep -q 'timestamp' docs/ARCHITECTURE.md && grep -q 'detectedChipset' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 startup_diagnostics.rs" "grep -q 'startup_diagnostics.rs' docs/ARCHITECTURE.md && grep -q 'startup.log' docs/ARCHITECTURE.md && grep -q 'msedgewebview2.exe' docs/ARCHITECTURE.md"
check "ARCHITECTURE 记录 r26 runtime PATH 注入" "grep -q 'r26-runtime' docs/ARCHITECTURE.md && grep -q 'PATH' docs/ARCHITECTURE.md"
check "CODE_MAP 说明执行面分散在模块文件中" "grep -q 'handlers.rs' docs/CODE_MAP.md && grep -q 'connection.rs' docs/CODE_MAP.md"
check "CODE_MAP 记录 list_ports 的 USB 识别字段" "grep -q 'usbVid' docs/CODE_MAP.md && grep -q 'detectedModel' docs/CODE_MAP.md && grep -q 'detectedChipset' docs/CODE_MAP.md"
check "CODE_MAP 记录硬件信息里的 USB VID/PID 展示" "grep -q 'USB VID-PID' docs/CODE_MAP.md && grep -q 'get_hardware_info' docs/CODE_MAP.md"
check "CODE_MAP 列出 AT 终端导出 IPC export_at_log" "grep -q 'export_at_log' docs/CODE_MAP.md"
check "CODE_MAP 记录 IMS / CFUN / LAN 专用 IPC" "grep -q 'get_ims_enabled' docs/CODE_MAP.md && grep -q 'get_cfun_mode' docs/CODE_MAP.md && grep -q 'get_lan_config' docs/CODE_MAP.md"
check "CODE_MAP 记录 MTU / DMZ / LAN 写入专用 IPC" "grep -q 'set_mtu' docs/CODE_MAP.md && grep -q 'set_dmz' docs/CODE_MAP.md && grep -q 'set_lan_config' docs/CODE_MAP.md"
check "CALL_FLOW 指向 connection.rs::auto_connect_at" "grep -q 'connection.rs::auto_connect_at' docs/CALL_FLOW.md"
check "CALL_FLOW 记录 USB VID/PID -> AT 分支 -> AT+CGMM fallback" "grep -q 'create_with_usb_ids' docs/CALL_FLOW.md && grep -q '0x2C7C:0x0900' docs/CALL_FLOW.md && grep -q '0x2C7C:0x0800/0x0801' docs/CALL_FLOW.md && grep -q '0x2C7C:0x0600' docs/CALL_FLOW.md && grep -q 'AT+CGMM' docs/CALL_FLOW.md"
check "CALL_FLOW 记录 connected_usb_ids 回填硬件信息" "grep -q 'connected_usb_ids' docs/CALL_FLOW.md && grep -q 'get_hardware_info' docs/CALL_FLOW.md"
check "CALL_FLOW 记录 port-changed 新 payload" "grep -q 'timestamp' docs/CALL_FLOW.md && grep -q 'usbVid' docs/CALL_FLOW.md && grep -q 'detectedChipset' docs/CALL_FLOW.md"
check "CALL_FLOW 说明 AT 终端导出来源为前端 DOM" "grep -q 'export_at_log' docs/CALL_FLOW.md"
check "CALL_FLOW 记录 live AT 发送前 10ms 间隔" "grep -q '不足 10ms' docs/CALL_FLOW.md"
check "CALL_FLOW 记录 startup diagnostics 启动链路" "grep -q 'install_startup_diagnostics' docs/CALL_FLOW.md && grep -q 'startup.log' docs/CALL_FLOW.md && grep -q 'append_runtime_layout_snapshot' docs/CALL_FLOW.md"
check "CALL_FLOW 明确业务配置走专用 IPC 而非 raw AT" "grep -q '业务配置专用流程' docs/CALL_FLOW.md && grep -q 'send_raw_at' docs/CALL_FLOW.md && grep -q '只保留给 AT 调试页' docs/CALL_FLOW.md"
check "UniSoc/ASR 端口影响开关成功后清空 live transport" "grep -q 'feature_toggle_drops_live_transport' src-tauri/src/handlers.rs && grep -q 'tguard.take' src-tauri/src/handlers.rs && grep -q '清空当前 live transport/vendor' docs/ARCHITECTURE.md && grep -q 'uartAt' docs/CALL_FLOW.md"
check "owner 文档不再把 usbcfg 当成 USB ID 通用恢复路径" "! grep -nE '恢复.*usbcfg|补查 .*usbcfg|recover USB VID/PID via AT\\+QCFG=\"usbcfg\"' docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md 2>/dev/null"
check "owner 文档不再把业务配置描述成前端快捷 AT" "! grep -n '前端快捷 AT' docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md 2>/dev/null"

echo "[2] 死路径与过时引用"
check "owner 文档不再引用已删除的 commands.rs" "! grep -n 'commands.rs' AGENTS.md CLAUDE.md docs/CONTEXT_PACK.md docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/REVIEW.md docs/BUILD.md docs/TECH_STACK.md docs/CODING.md docs/README.md 2>/dev/null"
check "owner 文档不再写死旧行数 1142/1556/6479" "! grep -nE '1142|1556|6479' AGENTS.md CLAUDE.md docs/CONTEXT_PACK.md docs/ARCHITECTURE.md docs/AT_COMMANDS.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/REVIEW.md docs/BUILD.md docs/TECH_STACK.md docs/CODING.md docs/README.md 2>/dev/null"
check "owner 文档不再描述最终用户 License / Factory 菜单入口" "! grep -nE 'show-load-license|show-license-status|#page-factory|factory_write_sn_to_device|get_license_status|load_license_file' docs/ARCHITECTURE.md docs/CODE_MAP.md docs/CALL_FLOW.md docs/BUILD.md docs/TECH_STACK.md 2>/dev/null"
check "本地 agent/tool 目录不再被 Git 跟踪" "[ -z \"$(git ls-files .agents .codegraph .gitnexus .specify .understand-anything .workbuddy opencode.json 2>/dev/null)\" ]"
check ".gitignore 继续忽略本地 agent/tool 目录" "grep -q '^.agents/$' .gitignore && grep -q 'opencode.json' .gitignore && grep -q '\\.codegraph/' .gitignore && grep -q '\\.gitnexus/' .gitignore && grep -q '\\.specify/' .gitignore && grep -q '\\.understand-anything/' .gitignore && grep -q '\\.workbuddy/' .gitignore"
check ".gitignore 继续忽略 fixed WebView2 runtime（含 app-local 目录）" "grep -q '^webview2-runtime/$' .gitignore && grep -q '^src-tauri/webview2-runtime/$' .gitignore"
check ".gitignore 继续忽略 r26 sidecar 运行库 DLL" "grep -q '^src-tauri/resources/r26-runtime/vcruntime140.dll$' .gitignore"
check "fixed WebView2 runtime 不进入 Git" "[ -z \"$(git ls-files webview2-runtime src-tauri/webview2-runtime 2>/dev/null)\" ]"
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
check "前端业务流程不再复用 send_raw_at" "[ \"$(grep -c 'send_raw_at' src/desktop/app.js)\" -eq 1 ]"

echo "[5] 文档验证入口"
check "BUILD 文档包含 verify-docs" "grep -q 'verify-docs.sh' docs/BUILD.md"
check "CODING 文档包含 verify-docs" "grep -q 'verify-docs.sh' docs/CODING.md"
check "AGENTS 验证基线包含 cargo test / cargo build / verify-docs" "grep -q 'cargo test --workspace' AGENTS.md && grep -q 'cargo build -p modem-hal' AGENTS.md && grep -q 'verify-docs.sh' AGENTS.md"
check "AT_COMMANDS 说明 USB VID/PID 优先、AT+CGMM 兜底" "grep -q 'VID/PID' docs/AT_COMMANDS.md && grep -q 'AT+CGMM' docs/AT_COMMANDS.md"
check "AT_COMMANDS 记录 0900/0800/0801/0600 USB 平台映射" "grep -q '0x2C7C:0x0900' docs/AT_COMMANDS.md && grep -q '0x2C7C:0x0800' docs/AT_COMMANDS.md && grep -q '0x2C7C:0x0801' docs/AT_COMMANDS.md && grep -q '0x2C7C:0x0600' docs/AT_COMMANDS.md"
check "AT_COMMANDS 记录 IMS / LAN / DMZ / MTU 专用合同" "grep -q 'get_ims_enabled' docs/AT_COMMANDS.md && grep -q 'AT+QMAP=\"DMZ\",1,4' docs/AT_COMMANDS.md && grep -q 'AT+QCFG=\"mtu\",<value>' docs/AT_COMMANDS.md"
check "CODE_MAP 记录 ADB / SSH 调试 IPC" "grep -q 'start_adb_session' docs/CODE_MAP.md && grep -q 'start_ssh_session' docs/CODE_MAP.md"
check "ADB 调试页不读取 modem AT 状态" "grep -q '不读取任何 modem AT 状态' docs/CALL_FLOW.md && grep -q '不读取任何 modem AT 状态' docs/AT_COMMANDS.md && ! grep -nE \"get_feature_toggles|get_adb_enabled|QCFG\" src/desktop/js/debug-terminal.js"
check "BUILD 记录 ADB 资源目录" "grep -q 'src-tauri/resources/adb/' docs/BUILD.md && grep -q 'Sdk/' docs/BUILD.md"
check "BUILD 记录 r26 runtime 资源目录与 dist DLL" "grep -q 'src-tauri/resources/r26-runtime/' docs/BUILD.md && grep -q 'dist/vcruntime140.dll' docs/BUILD.md"
check "BUILD / TECH_STACK 记录 downloadBootstrapper 与系统 WebView2 约束" "grep -q 'downloadBootstrapper' docs/BUILD.md && grep -q 'downloadBootstrapper' docs/TECH_STACK.md && grep -q '系统 WebView2' docs/BUILD.md"
check "BUILD 记录双安装包变体 webview/nowebview" "grep -q 'webview' docs/BUILD.md && grep -q 'nowebview' docs/BUILD.md"
check "BUILD 记录 portable ZIP 当前都依赖系统 WebView2" "grep -q 'portable-lite.zip' docs/BUILD.md && grep -q '同内容' docs/BUILD.md && grep -q '依赖系统 WebView2' docs/BUILD.md"
check "BUILD 记录 portable ZIP 不包含 license / 激活工具" "grep -q '任何 license / 设备激活工具都不属于最终用户桌面交付物' docs/BUILD.md && ! grep -q 'license-gen' docs/BUILD.md"
check "BUILD 记录启动失败日志路径" "grep -q 'startup.log' docs/BUILD.md && grep -q 'LOCALAPPDATA' docs/BUILD.md && grep -q 'msedgewebview2.exe' docs/BUILD.md"
check "TECH_STACK 记录 Windows 打包资产边界" "grep -q '.cargo/config.toml' docs/TECH_STACK.md && grep -q 'src-tauri/resources/adb/' docs/TECH_STACK.md"
check "TECH_STACK 记录 r26 x86 runtime 约束" "grep -q 'r26-runtime' docs/TECH_STACK.md && grep -q 'x86 sidecar' docs/TECH_STACK.md"
check "根 README 记录架构 / 环境 / 构建 / 发版内容" "[ -e README.md ] && grep -q '## 架构概览' README.md && grep -q '## 开发环境' README.md && grep -q '## 构建与验证' README.md && grep -q '## 发版建议' README.md"
check "README 记录启动失败日志路径" "grep -q 'startup.log' README.md && grep -q '双击后' README.md && grep -q 'msedgewebview2.exe' README.md"
check "README 记录 r26 runtime 打包说明" "grep -q 'vcruntime140.dll' README.md && grep -q 'r26-cli' README.md"
check "README 记录 docs/specs 需求文档位置" "grep -q 'docs/specs/' docs/README.md || grep -q 'specs/001-modem-debug-tool' docs/README.md"
check "README 记录构建产物与适用场景" "grep -q 'portable-lite.zip' docs/README.md && grep -q '便携包' docs/README.md && grep -q 'NSIS 安装包' docs/README.md"
check "BUILD / README 指向当前 build.ps1 入口" "grep -q 'build.ps1' docs/BUILD.md && grep -q 'build.ps1' README.md && grep -q 'build.ps1' AGENTS.md"
check "build-win.bat 只作为 build.ps1 包装器" "grep -q 'build.ps1' build-win.bat && ! grep -q 'src-tauri\\\\webview2-runtime' build-win.bat && ! grep -q 'fixed WebView2 runtime' build-win.bat"
check "tauri.conf / build.rs / setup-webview2 已切到 downloadBootstrapper 流程" "grep -q 'downloadBootstrapper' src-tauri/tauri.conf.json && ! grep -q 'ensure_fixed_runtime' src-tauri/build.rs && grep -q 'downloadBootstrapper' scripts/setup-webview2.ps1 && ! grep -q 'fixedRuntime' scripts/setup-webview2.ps1"
check "main.rs 不再做 WebView2 预检/安装（交给 Tauri + 系统 WebView2）" "! grep -q 'ensure_webview2' src-tauri/src/main.rs && ! grep -q 'webview2_installed' src-tauri/src/main.rs"
check "不再维护重复的 WEBVIEW2_BUILD 文档" "[ ! -e docs/WEBVIEW2_BUILD.md ]"
check "主工程不再引用 modem-license" "! grep -q 'modem-license' src-tauri/Cargo.toml && ! grep -q 'modem_license' src-tauri/src/lib.rs && [ ! -e src-tauri/src/license.rs ]"
check "仓库不再保留 license 工具目录" "[ ! -e modem-license ] && [ ! -e tools/license-gen ]"

echo
echo "=== 结果: $pass 通过 / $fail 失败 ==="
if [ "$fail" -gt 0 ]; then
  exit 1
fi
