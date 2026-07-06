import fs from 'node:fs';
import path from 'node:path';
import assert from 'node:assert/strict';

const script = fs.readFileSync('build.ps1', 'utf8');
const buildHelperScript = fs.readFileSync('scripts/build-helper.ps1', 'utf8');
const setupWebviewScript = fs.readFileSync('scripts/setup-webview2.ps1', 'utf8');
const tauriConfig = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8'));
const webviewInstallMode = tauriConfig.bundle.windows.webviewInstallMode;
const bundleResources = tauriConfig.bundle.resources;

assert.equal(webviewInstallMode.type, 'fixedRuntime', 'desktop build should keep fixedRuntime enabled by default');
assert.ok(!webviewInstallMode.path.includes('..'), 'fixed runtime path must not escape above the executable directory');
assert.equal(
  path.normalize(path.join('dist', webviewInstallMode.path)),
  path.normalize(path.join('dist', 'webview2-runtime')),
  'portable full zip must place fixed runtime alongside modem-cat.exe'
);

assert.ok(!script.includes('license-gen.exe'), 'release script should not package license-gen');
assert.ok(script.includes('function Remove-StaleDistArtifacts'), 'release script should clean stale dist artifacts before publishing');
assert.ok(script.includes('modem-cat.zip'), 'release script should explicitly remove the legacy modem-cat.zip artifact');
assert.ok(script.includes('ModemCat_v${ver}_portable.zip'), 'release script should create the full portable zip');
assert.ok(script.includes('ModemCat_v${ver}_portable-lite.zip'), 'release script should create a lite portable zip');
assert.ok(script.includes('src-tauri\\webview2-runtime'), 'release script should stage the fixed runtime from src-tauri/webview2-runtime');
assert.ok(
  script.includes('Join-Path $distDir "webview2-runtime"'),
  'directly runnable dist\\modem-cat.exe should stage webview2-runtime alongside the exe'
);
assert.ok(
  script.includes('Copy-Item -LiteralPath $stagedWebview2 -Destination $distRuntimeDir -Recurse -Force'),
  'release script should copy the fixed runtime into dist for direct exe validation'
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
assert.ok(script.includes('Join-Path $pFull "webview2-runtime"'), 'full portable zip should include the runtime folder next to modem-cat.exe');
assert.ok(
  !setupWebviewScript.includes('linkid=2124701'),
  'setup-webview2 should not prepare app-local runtime from the Evergreen standalone installer'
);
assert.ok(
  setupWebviewScript.includes('FixedVersionRuntime'),
  'setup-webview2 should target the official Fixed Version package'
);
assert.ok(
  setupWebviewScript.includes('developer.microsoft.com/en-us/microsoft-edge/webview2'),
  'setup-webview2 should source fixed-version metadata from the official WebView2 download page'
);

console.log('release packaging assertions passed');
