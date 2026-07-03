import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appJsPath = path.join(__dirname, '..', 'src', 'desktop', 'app.js');
const appJs = fs.readFileSync(appJsPath, 'utf8');

assert.match(
  appJs,
  /for \(let c = 1; c <= 8; c\+\+\)/,
  'APN CID selector should only offer CID 1-8.',
);

assert.match(
  appJs,
  /apnData = \(list \|\| \[\]\)\s*\.filter\(\s*a => \(a\.cid \|\| 0\) >= 1 && \(a\.cid \|\| 0\) <= 8\s*\)/s,
  'APN list rendering should defensively filter entries to CID 1-8.',
);

console.log('apn cid range assertions passed');
