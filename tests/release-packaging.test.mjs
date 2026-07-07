import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';

const script = fs.readFileSync('build.ps1', 'utf8');
const buildWinScript = fs.readFileSync('build-win.bat', 'utf8');
const buildHelperScript = fs.readFileSync('scripts/build-helper.ps1', 'utf8');
const setupWebviewScript = fs.readFileSync('scripts/setup-webview2.ps1', 'utf8');
const tauriConfig = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
const webviewInstallMode = tauriConfig.bundle.windows.webviewInstallMode;
const bundleResources = tauriConfig.bundle.resources;

assert.equal(
  webviewInstallMode.type,
  'embedBootstrapper',
  'desktop build should use embedBootstrapper by default'
);
assert.ok(
  webviewInstallMode.path == null,
  'embedBootstrapper config should not pin a fixed WebView2 runtime path'
);

assert.ok(!script.includes('license-gen.exe'), 'release script should not package license-gen');
assert.ok(script.includes('function Remove-StaleDistArtifacts'), 'release script should clean stale dist artifacts before publishing');
assert.ok(script.includes('modem-cat.zip'), 'release script should explicitly remove the legacy modem-cat.zip artifact');
assert.ok(script.includes('ModemCat_v${ver}_portable.zip'), 'release script should create the full portable zip');
assert.ok(script.includes('ModemCat_v${ver}_portable-lite.zip'), 'release script should create a lite portable zip');
assert.ok(
  !script.includes('src-tauri\\webview2-runtime'),
  'release script should not stage a fixed WebView2 runtime from src-tauri/webview2-runtime'
);
assert.ok(
  script.includes('Join-Path $DistDir "webview2-runtime"'),
  'release script may clean stale dist\\webview2-runtime leftovers from older builds'
);
assert.ok(
  script.includes('Join-Path $distDir "vcruntime140.dll"'),
  'release script should stage the x86 r26 runtime DLL alongside the sidecar'
);
assert.ok(
  script.includes('src-tauri\\resources\\r26-runtime'),
  'release script should stage the r26 runtime resource directory before packaging'
);
assert.ok(
  buildHelperScript.includes('src-tauri\\resources\\r26-runtime'),
  'installer helper should stage the r26 runtime resource directory before tauri build'
);
assert.ok(
  bundleResources.includes('resources/r26-runtime/'),
  'tauri bundle resources should include the r26 runtime directory for installer builds'
);
assert.ok(
  !script.includes('Join-Path $pFull "webview2-runtime"') &&
    !script.includes('Copy-Item -LiteralPath $stagedWebview2') &&
    !script.includes('src-tauri\\webview2-runtime'),
  'portable artifacts should not stage a fixed WebView2 runtime directory as a build input'
);
assert.ok(
  buildWinScript.includes('build.ps1'),
  'build-win.bat should delegate to build.ps1'
);
assert.ok(
  !buildWinScript.includes('src-tauri\\webview2-runtime') &&
    !buildWinScript.includes('未找到 fixed WebView2 runtime'),
  'build-win.bat should not require a fixed WebView2 runtime path'
);
assert.ok(
  !setupWebviewScript.includes('linkid=2124701'),
  'setup-webview2 should not prepare app-local runtime from the Evergreen standalone installer'
);
assert.ok(
  !setupWebviewScript.includes('FixedVersionRuntime'),
  'setup-webview2 should no longer target a Fixed Version runtime package'
);
assert.ok(
  setupWebviewScript.includes('embedBootstrapper') &&
    setupWebviewScript.includes('webview2-runtime'),
  'setup-webview2 should document embedBootstrapper mode and optional legacy cleanup'
);

console.log('release packaging assertions passed');
