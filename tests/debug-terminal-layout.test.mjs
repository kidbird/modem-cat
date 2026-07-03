import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const cssPath = path.join(__dirname, '..', 'src', 'desktop', 'styles.css');
const css = fs.readFileSync(cssPath, 'utf8');

assert.match(
  css,
  /#page-adbdebug,\s*#page-sshdebug\s*\{[^}]*flex:\s*1;[^}]*min-height:\s*0;/s,
  'ADB / SSH pages should stretch to fill the main content area.',
);

assert.match(
  css,
  /#page-adbdebug\s+\.panel,\s*#page-sshdebug\s+\.panel\s*\{[^}]*flex:\s*1;[^}]*display:\s*flex;[^}]*flex-direction:\s*column;[^}]*min-height:\s*0;/s,
  'ADB / SSH panels should stretch vertically with the page.',
);

assert.match(
  css,
  /\.terminal-wrap\s*\{[^}]*flex:\s*1;[^}]*min-height:\s*0;/s,
  'The debug terminal wrapper should consume the remaining panel height.',
);

assert.match(
  css,
  /\.debug-terminal-stream\s*\{[^}]*flex:\s*1;[^}]*min-height:\s*320px;/s,
  'The debug terminal stream should flex with the layout instead of using a fixed height.',
);

assert.doesNotMatch(
  css,
  /\.debug-terminal-stream\s*\{[^}]*height:\s*clamp\(/s,
  'The debug terminal stream should not use a fixed clamp height anymore.',
);

console.log('debug terminal layout assertions passed');
