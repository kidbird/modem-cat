import fs from 'node:fs';
import assert from 'node:assert/strict';

const script = fs.readFileSync('scripts/build-release.ps1', 'utf8');

assert.ok(script.includes('$portablePayloadFiles = @('), 'portable payload list should be explicit');
assert.ok(!script.includes('license-gen.exe'), 'release script should not package license-gen');
assert.ok(script.includes('Add-DirectoryToZip -Archive $archive -SourceDir $webview2 -EntryPrefix "webview2-runtime"'), 'portable zip should still include webview2 runtime');
assert.ok(script.includes('foreach ($file in $portablePayloadFiles)'), 'portable zip should only include the curated payload list');

console.log('release packaging assertions passed');
