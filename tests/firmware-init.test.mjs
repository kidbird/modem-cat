import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appJsPath = path.join(__dirname, '..', 'src', 'desktop', 'app.js');
const appJs = fs.readFileSync(appJsPath, 'utf8');

assert.match(
  appJs,
  /if \(item\.dataset\.page === 'firmware'\)\s*\{\s*initFirmwarePage\(\);\s*\}/s,
  'Firmware nav clicks should go through a dedicated lazy init entrypoint.',
);

assert.match(
  appJs,
  /async function initFirmwarePage\(\)\s*\{/,
  'Firmware page should expose a lazy init function.',
);

assert.match(
  appJs,
  /if \(fw\.initialized\) return;/,
  'Firmware page init should be guarded so listeners only bind once.',
);

assert.doesNotMatch(
  appJs,
  /document\.getElementById\('fwSelectPacBtn'\)\.addEventListener\('click'/,
  'Firmware button listeners should not bind eagerly at script load time.',
);

console.log('firmware lazy init assertions passed');
