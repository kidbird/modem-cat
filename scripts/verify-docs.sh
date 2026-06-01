#!/usr/bin/env bash
# verify-docs.sh — 防止 doc-code drift
#
# 跑在 commit 改文档前：
#   bash scripts/verify-docs.sh
#
# 退出码：0 = 全部通过；1 = 至少一条漂移。
#
# 起源：2026-06-01 review 一次性发现 20+ 处 doc 与代码不一致，参见 docs/REVIEW.md。
# 每条 check 都对应一条已确认的 bug，加新 check 请写在末尾并附 commit 引用。

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

fail=0
pass=0

# ── Helpers ─────────────────────────────────────────────────────────────
# 期望: check() 失败打印 message 但不退出，收集 fail/pass 计数
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

# count_eq <desc> <file> <expected> <grep-pattern>
count_eq() {
    local desc="$1" file="$2" expected="$3" pattern="$4"
    local actual
    actual=$(grep -cE "$pattern" "$file" 2>/dev/null || echo 0)
    if [ "$actual" = "$expected" ]; then
        echo "  ✓ $desc ($actual)"
        pass=$((pass + 1))
    else
        echo "  ✗ $desc: 期望 $expected, 实际 $actual"
        fail=$((fail + 1))
    fi
}

echo "=== verify-docs.sh ==="
echo

# ── 1. 文件 / 行数 断言（防止 doc 写错 wc -l）─────────────────────────
echo "[1] 文件行数"
count_eq "lib.rs 行数 1139 (含 F2/F4 安全修复)" "src-tauri/src/lib.rs" 1139 "^"
count_eq "commands.rs 行数 504" "src-tauri/src/commands.rs"  504 "^"
count_eq "app.js 行数 1551"     "src/desktop/app.js"        1551 "^"
count_eq "types.rs 行数 252"    "modem-hal/src/types.rs"     252 "^"
count_eq "main.rs 行数 7"       "src-tauri/src/main.rs"        7 "^"

# ── 2. IPC 命令数（活 52 / 死 30 / 前端 30）────────────────────────────
echo "[2] IPC 命令数"
# 活数: lib.rs 中 #[tauri::command] 注解总数
count_eq "lib.rs #[tauri::command] 注解数 = 52"     "src-tauri/src/lib.rs"      52 "tauri::command"
count_eq "commands.rs 死代码 30 个 #[tauri::command]" "src-tauri/src/commands.rs" 30 "tauri::command"
# 前端唯一 invoke 名字数（直接 eval 绕过 bash 转义陷阱）
fe_count=$(grep -oE "invoke\('[a-z_]+'" src/desktop/app.js | sort -u | wc -l | tr -d ' ')
if [ "$fe_count" = "30" ]; then
    echo "  ✓ 前端 app.js 唯一 invoke 数 = 30 ($fe_count)"
    pass=$((pass + 1))
else
    echo "  ✗ 前端 app.js 唯一 invoke 数: 期望 30, 实际 $fe_count"
    fail=$((fail + 1))
fi

# ── 3. 死代码 / 安全断言 ─────────────────────────────────────────────
echo "[3] commands.rs 死代码"
count_eq "commands.rs .unwrap() 数 = 64" "src-tauri/src/commands.rs" 64 "\\.unwrap\\(\\)"
check    "commands.rs 0 caller (无 'mod commands')" \
         "! grep -rn 'mod commands' src-tauri/src/ src/desktop/"

# ── 4. 厂商检测关键字 (modem_factory.rs 是 source of truth) ────────────
echo "[4] 厂商检测关键字（顺序敏感）"
check "MT5700 优先于 RG500Q" "grep -B1 -A2 '\"MT5700\"' modem-hal/src/modem_factory.rs | grep -A2 'tdtech' | head -1 | grep -q 'MT5700'"
check "Qualcomm 关键字在 modem_factory.rs" "grep -q '\"RG500Q\"' modem-hal/src/modem_factory.rs"
check "UniSoc 关键字在 modem_factory.rs"   "grep -q '\"RG200U\"' modem-hal/src/modem_factory.rs"

# ── 5. 行号 / N 计数（防止 ARCHITECTURE.md 又写 28+）──────────────────
echo "[5] 关键数字"
# trait 内 fn 数 (区间内, 排除 default impl 块的嵌套)
trait_methods=$(sed -n '/^pub trait ModemVendor/,/^}/p' modem-hal/src/modem_vendor.rs | grep -cE "^    fn ")
if [ "$trait_methods" = "62" ]; then
    echo "  ✓ ModemVendor trait 区间内 fn 数 = 62 ($trait_methods)"
    pass=$((pass + 1))
else
    echo "  ✗ ModemVendor trait 方法数: 期望 62, 实际 $trait_methods"
    fail=$((fail + 1))
fi

# ── 6. AT 命令格式（防止 QMAP=MP 复活）────────────────────────────────
echo "[6] AT 命令格式"
check "AT+QMAP 必须用 'connect' 子命令（不是 'MP'）" \
      "! grep -rn 'QMAP=\"MP\"' modem-hal/src/ src-tauri/src/ src/desktop/ docs/ 2>/dev/null"
check "AT+QNETDEVCTL 用 cid 在首位" \
      "grep -q 'QNETDEVCTL=.*1,1' modem-hal/src/vendors/quectel/unisoc.rs"

# ── 7. 页面 / 容器数 ─────────────────────────────────────────────────
echo "[7] 页面 / 容器"
count_eq "index.html page 容器数 = 8" "src/desktop/index.html" 8 'id="page-'
check    "doc 不再写 '9 个 page'" \
         "! grep -rn '9 个 page\\|9 page' docs/ CLAUDE.md AGENTS.md"

# ── 8. spec_bands 函数不存在（防止 ARCHITECTURE 又写）────────────────
echo "[8] spec_bands 状态"
check "spec_bands_for_model 已删除（不应在代码中）" \
      "! grep -rn 'spec_bands' modem-hal/src/ src-tauri/src/ src/desktop/"

# ── 9. Stale app.js 行号（最大问题——所有 :NNNN 必须 < 1551 或语义化）──
echo "[9] Stale 行号"
# 任意形如 app.js:NNNN 且 NNNN>=1552 的引用即视作漂移（app.js 现 1551 行）
stale=$(grep -rnE "app\.js:[0-9]{4,}" docs/ CLAUDE.md AGENTS.md 2>/dev/null | awk -F: '{n=$NF+0; if(n>=1552) print}' || true)
if [ -z "$stale" ]; then
    echo "  ✓ 无 app.js 行号 ≥ 1552 的引用（漂移已清）"
    pass=$((pass + 1))
else
    echo "  ✗ 发现漂移的 app.js 行号:"
    echo "$stale" | sed 's/^/    /'
    fail=$((fail + 1))
fi

# ── 10. User-defined checks ──────────────────────────────────────────
# TODO: 加一条你自己最关心的检查。
# 比如：
#   - "frontend 0 个残留的 `AT+CFUN=1,1` 重启命令"（实际只有 reboot 路径用）
#   - "vendor 关键字表 (ARCHITECTURE.md) 与 modem_factory.rs 字符级一致"
#   - "result of pop_at_commands 不会无限增长"（绕 LoggingTransport Vec<Vec>）
# 写好后在下面加一行 check "..." "..." 即可。
# 例子（取消注释试试）:
# check "DEBUG 测试" "test -f scripts/verify-docs.sh"

echo
echo "=== 结果: $pass 通过 / $fail 失败 ==="
if [ "$fail" -gt 0 ]; then
    echo
    echo "⚠ 建议: 修完后再跑一次本脚本，确认 0 失败再 commit。"
    exit 1
fi
echo "✓ 全部通过，可以 commit。"
