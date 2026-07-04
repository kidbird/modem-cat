import fs from 'node:fs';
import assert from 'node:assert/strict';

const script = fs.readFileSync('scripts/build-release.ps1', 'utf8');

assert.ok(script.includes('$portablePayloadFiles = @('), 'portable payload list should be explicit');
assert.ok(!script.includes('license-gen.exe'), 'release script should not package license-gen');
assert.ok(script.includes('ModemCat_v${ver}_portable-lite.zip'), 'release script should create a lite portable zip');
assert.ok(script.includes('ModemCat_v${ver}_portable.zip'), 'release script should create the full portable zip');
assert.ok(script.includes('Add-DirectoryToZip -Archive $fullArchive -SourceDir $webview2 -EntryPrefix "webview2-runtime"'), 'full portable zip should still include webview2 runtime');
assert.ok(script.includes('Add-FilesToZip -Archive $liteArchive -BaseDir $dist -Files $portablePayloadFiles'), 'lite portable zip should only include the curated payload list');
assert.ok(script.includes('Add-FilesToZip -Archive $fullArchive -BaseDir $dist -Files $portablePayloadFiles'), 'full portable zip should include the curated payload list before webview2');

console.log('release packaging assertions passed');
