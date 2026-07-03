---
name: release-package
description: "Complete Windows release build: compiles portable exe + MSI/NSIS installers, bundles r26-cli sidecar, license-gen, and WebView2 runtime into dist/. Outputs everything flat in dist/ (no subdirectories). Use when user asks to build release, create installer, package dist, or prepare for distribution. Trigger: /release or /build-release."
trigger: /release
---

# /release-package

Complete Windows release build pipeline. Produces portable executable, installers (MSI + NSIS), firmware-download sidecar, license-gen tool, and WebView2 runtime — all in `dist/` root (no subdirectories, per AGENTS.md).

## Usage

```
/release              # full release build
/release --verify     # build + run verify-docs.sh + cargo test
/release --quick      # skip portable ZIP stage (faster, for iteration)
```

## What It Does

1. **Cleans** `dist/` (removes all old artifacts)
2. **Verifies toolchain** (Rust, cargo-tauri, MSVC via vswhere, webview2-runtime exists)
3. **Builds Tauri** — `cargo tauri build` → portable exe + MSI + NSIS
4. **Copies r26-cli sidecar** — firmware download component
5. **Builds license-gen** — License generation tool
6. **Stages WebView2 runtime for ZIP payload** — bundled fixedRuntime (no download needed)
7. **Creates portable ZIP** — `ModemCat_vX.Y.Z_portable.zip` with all components, unless `--quick` skips this stage
8. **Lists final artifacts** with file sizes

## Output Layout (dist/ root — flat, no subdirectories)

```
dist/
├── modem-cat.exe                              # Portable executable
├── Modem Cat_X.Y.Z_x64_zh-CN.msi             # MSI installer (has WebView2)
├── Modem Cat_X.Y.Z_x64-setup.exe             # NSIS installer (has WebView2)
├── r26-cli-x86_64-pc-windows-msvc.exe         # Firmware download sidecar
├── r26-cli.version.txt                        # Sidecar version
├── license-gen.exe                            # License generation tool
└── ModemCat_vX.Y.Z_portable.zip               # All-in-one portable package
```

AGENTS.md constraint: **构建产物统一输出到 dist/ 根目录下，禁止创建子目录**.

## Prerequisites

| Dependency | How to install |
|---|---|
| Rust + MSVC | https://rustup.rs + Visual Studio C++ workload |
| cargo-tauri 2.x | Auto-installed on first run |
| WebView2 Runtime | Run `scripts/setup-webview2.ps1` once (downloads ~200MB, then offline) |

## Script

The implementation lives at: `scripts/build-release.ps1`

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\build-release.ps1 -Quick
```

## Key Design Decisions

1. **No WebView2 download** — uses fixedRuntime bundled at build time. The MSI/NSIS installers include it, and the portable ZIP embeds it for offline use.

2. **Flat dist/ structure** — AGENTS.md mandates "构建产物统一输出到 dist/ 根目录下，禁止创建子目录". The flat layout makes download links predictable and avoids nested-path issues.

3. **Portable ZIP includes everything** — r26-cli, webview2-runtime (for systems without it), license-gen. User can unzip and run `modem-cat.exe` immediately. ZIP generation now writes entries directly into the archive instead of copying the whole payload to a temp folder first, which reduces release time and disk churn.

4. **MSVC auto-detection** — uses `vswhere.exe` to find any VS 2019/2022 installation. No hardcoded paths.

5. **Version from tauri.conf.json** — single source of truth (`$.version`). All artifact names derived from it.

## Troubleshooting

| Problem | Solution |
|---|---|
| "未找到 cargo" | Install Rust from https://rustup.rs |
| "未找到 MSVC" | Install VS Build Tools with C++ workload |
| "未找到 webview2-runtime" | Run `scripts/setup-webview2.ps1` once |
| Build OOM | Close other apps; release build needs ~4GB RAM |
| MSI not generated | Check `tauri.conf.json > bundle.active = true` |
