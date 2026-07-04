import fs from 'node:fs';
import assert from 'node:assert/strict';

const workspaceToml = fs.readFileSync('Cargo.toml', 'utf8');
const tauriToml = fs.readFileSync('src-tauri/Cargo.toml', 'utf8');
const tauriLib = fs.readFileSync('src-tauri/src/lib.rs', 'utf8');

assert.ok(!workspaceToml.includes('"modem-license"'), 'workspace should not build modem-license');
assert.ok(!tauriToml.includes('modem-license'), 'main desktop app should not depend on modem-license');
assert.ok(!tauriLib.includes('modem_license::'), 'main desktop app should not retain modem-license state');
assert.ok(!fs.existsSync('src-tauri/src/license.rs'), 'desktop app should not keep a license IPC module');
assert.ok(!fs.existsSync('modem-license'), 'repository should not keep the modem-license crate');
assert.ok(!fs.existsSync('tools/license-gen'), 'repository should not keep the license generator tool');

console.log('license removal assertions passed');
