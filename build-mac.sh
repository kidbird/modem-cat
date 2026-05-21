#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# build-mac.sh — Modem Cat macOS 构建脚本
#
# 用法:
#   ./build-mac.sh           # 构建当前架构 (aarch64 / x86_64)
#   ./build-mac.sh --bundle  # 构建 + 打包 .dmg 安装包
#   ./build-mac.sh --both    # 同时构建 aarch64 + x86_64
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

# ── 颜色输出 ──────────────────────────────────────────────────────────────────
RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

info()  { echo -e "${CYAN}[INFO]${RESET} $*"; }
ok()    { echo -e "${GREEN}[OK]${RESET}   $*"; }
warn()  { echo -e "${YELLOW}[WARN]${RESET} $*"; }
error() { echo -e "${RED}[ERR]${RESET}  $*" >&2; exit 1; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# ── 参数解析 ──────────────────────────────────────────────────────────────────
MODE="native"      # native | bundle | both
for arg in "$@"; do
  case "$arg" in
    --bundle) MODE="bundle" ;;
    --both)   MODE="both"   ;;
    --help|-h)
      echo "用法: $0 [--bundle] [--both]"
      echo "  (无参数)   仅构建当前架构的二进制"
      echo "  --bundle   构建 + 打包 .dmg 安装包"
      echo "  --both     交叉编译 aarch64 + x86_64 两个架构"
      exit 0
      ;;
    *) error "未知参数: $arg  (使用 --help 查看用法)" ;;
  esac
done

# ── 前置检查 ──────────────────────────────────────────────────────────────────
info "检查前置依赖..."

command -v cargo >/dev/null 2>&1 || error "未找到 cargo，请安装 Rust: https://rustup.rs"
command -v rustup >/dev/null 2>&1 || error "未找到 rustup"

RUST_VER=$(rustc --version)
ok "Rust: $RUST_VER"

# 检查 tauri-cli（仅 --bundle 模式需要）
if [[ "$MODE" == "bundle" ]]; then
  if ! cargo tauri --version >/dev/null 2>&1; then
    warn "未安装 cargo-tauri，正在安装..."
    cargo install tauri-cli --version '^2' --locked
  fi
  ok "tauri-cli: $(cargo tauri --version)"
fi

# 交叉编译目标检查
if [[ "$MODE" == "both" ]]; then
  for target in aarch64-apple-darwin x86_64-apple-darwin; do
    if ! rustup target list --installed | grep -q "$target"; then
      info "添加 Rust target: $target"
      rustup target add "$target"
    fi
  done
  ok "交叉编译 targets 就绪"
fi

# ── 开始构建 ──────────────────────────────────────────────────────────────────
START=$(date +%s)
echo ""
echo -e "${BOLD}═══════════════════════════════════════════${RESET}"
echo -e "${BOLD}  Modem Cat — macOS 构建 [$(date '+%H:%M:%S')]${RESET}"
echo -e "${BOLD}═══════════════════════════════════════════${RESET}"
echo ""

build_target() {
  local target="$1"
  local label="${target:-当前架构}"
  info "构建: $label"
  if [[ "$MODE" == "bundle" ]]; then
    (cd src-tauri && cargo tauri build ${target:+--target "$target"})
  else
    if [[ -n "${target:-}" ]]; then
      (cd src-tauri && cargo build --release --target "$target")
    else
      (cd src-tauri && cargo build --release)
    fi
  fi
  ok "构建完成: $label"
}

case "$MODE" in
  native)
    build_target ""
    BINARY="target/release/modem-cat"
    if [[ -f "$BINARY" ]]; then
      SIZE=$(du -sh "$BINARY" | cut -f1)
      ok "二进制: $BINARY ($SIZE)"
    fi
    ;;
  bundle)
    build_target ""
    # 找到产出的 .dmg
    DMG=$(find src-tauri/target/release/bundle/dmg -name "*.dmg" 2>/dev/null | head -1)
    if [[ -n "$DMG" ]]; then
      SIZE=$(du -sh "$DMG" | cut -f1)
      ok "安装包: $DMG ($SIZE)"
    fi
    ;;
  both)
    build_target "aarch64-apple-darwin"
    build_target "x86_64-apple-darwin"
    # 合并为 Universal Binary（可选）
    AARCH="src-tauri/target/aarch64-apple-darwin/release/modem-cat"
    X86="src-tauri/target/x86_64-apple-darwin/release/modem-cat"
    if [[ -f "$AARCH" && -f "$X86" ]]; then
      info "合并为 Universal Binary..."
      lipo -create -output "target/modem-cat-universal" "$AARCH" "$X86"
      SIZE=$(du -sh "target/modem-cat-universal" | cut -f1)
      ok "Universal Binary: target/modem-cat-universal ($SIZE)"
    fi
    ;;
esac

# ── 完成 ──────────────────────────────────────────────────────────────────────
END=$(date +%s)
ELAPSED=$((END - START))
echo ""
echo -e "${GREEN}${BOLD}✓ 构建成功！耗时 ${ELAPSED}s${RESET}"
