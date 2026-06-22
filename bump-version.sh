#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# bump-version.sh — 统一更新 Modem Cat 版本号
#
# 用法:
#   ./bump-version.sh 0.2.0
#
# 会同步更新以下文件：
#   src-tauri/tauri.conf.json   → "version"
#   src-tauri/Cargo.toml        → version
#   package.json                → "version"
#   modem-hal/Cargo.toml        → version
#   modem-license/Cargo.toml    → version
#   Cargo.lock 通过 cargo 自动更新
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'
ok()    { echo -e "${GREEN}[OK]${RESET}    $*"; }
info()  { echo -e "${CYAN}[INFO]${RESET}  $*"; }
error() { echo -e "${RED}[ERR]${RESET}   $*" >&2; exit 1; }

# ── 参数检查 ──────────────────────────────────────────────────────────────────
[[ $# -eq 1 ]] || { echo "用法: $0 <版本号>  (例: $0 0.2.0)"; exit 1; }
NEW="$1"
# 简单格式校验：X.Y.Z
[[ "$NEW" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || error "版本号格式无效: '$NEW'，应为 X.Y.Z"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# 读取当前版本（以 tauri.conf.json 为准）
OLD=$(python3 -c "import json,sys; print(json.load(open('src-tauri/tauri.conf.json'))['version'])")
[[ "$OLD" == "$NEW" ]] && { echo "已是 v$NEW，无需更新。"; exit 0; }

echo ""
echo -e "${BOLD}  版本升级: v${OLD} → v${NEW}${RESET}"
echo ""

# ── 更新各文件 ────────────────────────────────────────────────────────────────

# 1. src-tauri/tauri.conf.json
python3 - <<PYEOF
import json, pathlib
p = pathlib.Path('src-tauri/tauri.conf.json')
d = json.loads(p.read_text())
d['version'] = '$NEW'
p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + '\n')
PYEOF
ok "src-tauri/tauri.conf.json  →  \"version\": \"$NEW\""

# 2. src-tauri/Cargo.toml  (第一个 version = "..." 行，即 [package] 的)
sed -i.bak "0,/^version = \"[^\"]*\"/{s/^version = \"[^\"]*\"/version = \"$NEW\"/}" src-tauri/Cargo.toml
rm -f src-tauri/Cargo.toml.bak
ok "src-tauri/Cargo.toml        →  version = \"$NEW\""

# 3. package.json
python3 - <<PYEOF
import json, pathlib
p = pathlib.Path('package.json')
d = json.loads(p.read_text())
d['version'] = '$NEW'
p.write_text(json.dumps(d, indent=2, ensure_ascii=False) + '\n')
PYEOF
ok "package.json                →  \"version\": \"$NEW\""

# 4. modem-hal/Cargo.toml
sed -i.bak "0,/^version = \"[^\"]*\"/{s/^version = \"[^\"]*\"/version = \"$NEW\"/}" modem-hal/Cargo.toml
rm -f modem-hal/Cargo.toml.bak
ok "modem-hal/Cargo.toml        →  version = \"$NEW\""

# 5. modem-license/Cargo.toml
sed -i.bak "0,/^version = \"[^\"]*\"/{s/^version = \"[^\"]*\"/version = \"$NEW\"/}" modem-license/Cargo.toml
rm -f modem-license/Cargo.toml.bak
ok "modem-license/Cargo.toml    →  version = \"$NEW\""

# 6. 更新 Cargo.lock（让 lockfile 与 Cargo.toml 保持一致）
info "更新 Cargo.lock..."
cargo update --workspace --precise "$NEW" -p modem-cat 2>/dev/null || \
  cargo generate-lockfile 2>/dev/null || true
ok "Cargo.lock 已同步"

echo ""
echo -e "${GREEN}${BOLD}✓ 版本已更新为 v${NEW}${RESET}"
echo ""
echo "  下一步："
echo "    git add -A && git commit -m \"chore: bump version to v${NEW}\""
echo "    ./build-mac.sh   或   build-win.bat"
echo ""
