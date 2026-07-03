import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const htmlPath = path.join(__dirname, '..', 'src', 'desktop', 'index.html');
const html = fs.readFileSync(htmlPath, 'utf8');

const navMatches = [...html.matchAll(/<div class="nav-item(?: active)?"[^>]*data-page="([^"]+)"/g)];
const navPages = navMatches.map((match) => match[1]);

const anchorPages = ['status', 'cellular', 'ip', 'at', 'adbdebug', 'sshdebug'];
const expectedTail = ['monitor', 'scene', 'firmware', 'hardware', 'atmanual', 'settings'];

const firstTailIndex = navPages.findIndex((page) => page === expectedTail[0]);
assert.notEqual(firstTailIndex, -1, 'The reordered tail section should exist in the nav.');

assert.deepEqual(
  navPages.slice(0, anchorPages.length),
  anchorPages,
  'The top section of the nav should remain unchanged.',
);

assert.deepEqual(
  navPages.slice(firstTailIndex, firstTailIndex + expectedTail.length),
  expectedTail,
  'The lower section of the nav should follow the requested order.',
);

console.log('nav order assertions passed');
