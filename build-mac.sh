#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# build-mac.sh — Modem Cat macOS 构建脚本
#
# 产出物（构建完成后统一展示路径）:
#   Portable : src-tauri/target/release/bundle/macos/Modem Cat.app
#   安装包   : src-tauri/target/release/bundle/dmg/Modem Cat_*.dmg
#
# 用法:
#   ./build-mac.sh              # 构建当前架构（aarch64 或 x86_64）
#   ./build-mac.sh --universal  # 交叉编译并合并为 Universal Binary
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

info()  { echo -e "${CYAN}[INFO]${RESET}  $*"; }
ok()    { echo -e "${GREEN}[OK]${RESET}    $*"; }
warn()  { echo -e "${YELLOW}[WARN]${RESET}  $*"; }
error() { echo -e "${RED}[ERR]${RESET}   $*" >&2; exit 1; }
sep()   { echo -e "${BOLD}───────────────────────────────────────────${RESET}"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

UNIVERSAL=false
for arg in "$@"; do
  case "$arg" in
    --universal) UNIVERSAL=true ;;
    --help|-h)
      echo "用法: $0 [--universal]"
      echo "  (无参数)     构建当前架构"
      echo "  --universal  交叉编译 aarch64 + x86_64，合并为 Universal Binary"
      exit 0 ;;
    *) error "未知参数: $arg" ;;
  esac
done

# ── 前置检查 ──────────────────────────────────────────────────────────────────
sep
info "检查前置依赖..."

command -v cargo  >/dev/null 2>&1 || error "未找到 cargo，请安装 Rust: https://rustup.rs"
command -v rustup >/dev/null 2>&1 || error "未找到 rustup"

ok "Rust: $(rustc --version)"

if ! cargo tauri --version >/dev/null 2>&1; then
  warn "未安装 cargo-tauri，正在安装..."
  cargo install tauri-cli --version '^2' --locked
fi
ok "tauri-cli: $(cargo tauri --version)"

if $UNIVERSAL; then
  for t in aarch64-apple-darwin x86_64-apple-darwin; do
    rustup target list --installed | grep -q "$t" || {
      info "添加 Rust target: $t"
      rustup target add "$t"
    }
  done
fi

# ── 构建 ──────────────────────────────────────────────────────────────────────
sep
echo ""
echo -e "${BOLD}  Modem Cat macOS Build  —  $(date '+%Y-%m-%d %H:%M:%S')${RESET}"
echo ""

START=$(date +%s)

if $UNIVERSAL; then
  # 分别构建两架构，再用 lipo 合并 .app 内的可执行文件
  info "构建 aarch64-apple-darwin..."
  (cd src-tauri && cargo tauri build --target aarch64-apple-darwin)

  info "构建 x86_64-apple-darwin..."
  (cd src-tauri && cargo tauri build --target x86_64-apple-darwin)

  AARCH_BIN="src-tauri/target/aarch64-apple-darwin/release/modem-cat"
  X86_BIN="src-tauri/target/x86_64-apple-darwin/release/modem-cat"

  if [[ -f "$AARCH_BIN" && -f "$X86_BIN" ]]; then
    info "合并 Universal Binary..."
    lipo -create -output "src-tauri/target/release/modem-cat" "$AARCH_BIN" "$X86_BIN"
  fi
else
  info "构建当前架构..."
  (cd src-tauri && cargo tauri build)
fi

END=$(date +%s)

# ── 展示产出物 ────────────────────────────────────────────────────────────────
sep
echo ""
echo -e "${BOLD}  产出物${RESET}"
echo ""

BUNDLE_DIR="src-tauri/target/release/bundle"

# Portable — .app bundle
APP=$(find "$BUNDLE_DIR/macos" -maxdepth 1 -name "*.app" 2>/dev/null | head -1)
if [[ -n "$APP" ]]; then
  SIZE=$(du -sh "$APP" | cut -f1)
  ok "Portable  →  $APP  ($SIZE)"
else
  warn "未找到 .app（路径: $BUNDLE_DIR/macos）"
fi

# 安装包 — .dmg
DMG=$(find "$BUNDLE_DIR/dmg" -maxdepth 1 -name "*.dmg" 2>/dev/null | head -1)
if [[ -n "$DMG" ]]; then
  SIZE=$(du -sh "$DMG" | cut -f1)
  ok "安装包   →  $DMG  ($SIZE)"
else
  warn "未找到 .dmg（路径: $BUNDLE_DIR/dmg）"
fi

echo ""
echo -e "${GREEN}${BOLD}✓ 构建成功，耗时 $((END - START))s${RESET}"
sep
