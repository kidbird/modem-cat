import { describe, it, expect, beforeAll } from 'vitest';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';

// Structural tests for src/desktop/styles.css. These guard against the two
// regressions we've already shipped:
//   1. `@import` in the middle of the file — browsers ignore it, Inter
//      never loads, Windows falls back to Segoe UI and looks "ugly".
//   2. Missing or mismatched CSS variables across theme blocks — would
//      cause some themes to render with :root defaults (e.g. midnight
//      blue) instead of their own palette.

const STYLES = readFileSync(
  resolve(import.meta.dirname, 'styles.css'),
  'utf8',
);

const REQUIRED_VARS = [
  '--bg-primary',
  '--bg-secondary',
  '--bg-tertiary',
  '--border-color',
  '--border-hover',
  '--text-primary',
  '--text-secondary',
  '--text-muted',
  '--accent',
  '--accent-hover',
  '--accent-glow',
  '--success',
  '--success-glow',
  '--warning',
  '--error',
  '--danger',
  '--danger-hover',
  '--cyan',
  '--tab-bar-bg',
  '--tab-active-bg',
];

// Each theme block defines these vars. Capture the body of each
// `[data-theme="X"] { ... }` block (or `:root { ... }`) by tracking
// brace depth.
function extractBlocks(css) {
  const blocks = [];
  const re = /(:root|\[data-theme="[a-z-]+"\])\s*\{/g;
  let m;
  while ((m = re.exec(css)) !== null) {
    const start = m.index;
    const selector = m[1];
    let depth = 1;
    let i = re.lastIndex;
    while (i < css.length && depth > 0) {
      const ch = css[i];
      if (ch === '{') depth++;
      else if (ch === '}') depth--;
      i++;
    }
    const body = css.slice(re.lastIndex, i - 1);
    blocks.push({ selector, body });
  }
  return blocks;
}

// Extract `--name: value;` declarations from a block body.
function extractVars(body) {
  const out = {};
  const re = /(--[a-z0-9-]+)\s*:\s*([^;]+);/g;
  let m;
  while ((m = re.exec(body)) !== null) {
    out[m[1].trim()] = m[2].trim();
  }
  return out;
}

// Strip the trailing `!important` (and surrounding whitespace) from a CSS
// value so we can compare colors / sizes across theme blocks without
// the noise.
function stripImportant(value) {
  return (value || '').replace(/\s*!important\s*$/, '').trim();
}

describe('styles.css structure', () => {
  let blocks;
  beforeAll(() => {
    blocks = extractBlocks(STYLES);
  });

  it('@import is the first rule in the file (CSS spec requirement)', () => {
    // Find the first non-blank, non-comment line that opens a block or is
    // an at-rule. It must be @import.
    const lines = STYLES.split('\n');
    for (let i = 0; i < lines.length; i++) {
      const trimmed = lines[i].trim();
      if (!trimmed) continue;
      if (trimmed.startsWith('/*') || trimmed.startsWith('*') || trimmed.startsWith('//')) continue;
      // First non-blank/non-comment line must be @import.
      expect(
        trimmed.startsWith('@import'),
        `First rule at line ${i + 1} is "${trimmed.slice(0, 60)}" but should be @import`,
      ).toBe(true);
      return;
    }
    throw new Error('No rules found in styles.css');
  });

  it('does not contain a duplicate @import later in the file', () => {
    const matches = STYLES.match(/@import\s+url/g) || [];
    expect(matches.length).toBe(1);
  });

  it('defines :root + 3 theme blocks (light, blue-light, dark)', () => {
    const selectors = blocks.map((b) => b.selector);
    expect(selectors).toContain(':root');
    expect(selectors).toContain('[data-theme="light"]');
    expect(selectors).toContain('[data-theme="blue-light"]');
    expect(selectors).toContain('[data-theme="dark"]');
  });

  it.each([':root', '[data-theme="light"]', '[data-theme="blue-light"]', '[data-theme="dark"]'])(
    '%s defines all 20 required CSS variables',
    (selector) => {
      const block = blocks.find((b) => b.selector === selector);
      expect(block, `block ${selector} not found`).toBeDefined();
      const vars = extractVars(block.body);
      for (const name of REQUIRED_VARS) {
        expect(vars[name], `${selector} missing var ${name}`).toBeDefined();
      }
      // Also: no extra/unexpected vars
      const declared = Object.keys(vars);
      expect(declared.length, `${selector} has ${declared.length} vars, expected 20`).toBe(20);
    },
  );

  it.each([':root', '[data-theme="light"]', '[data-theme="blue-light"]', '[data-theme="dark"]'])(
    '%s theme vars avoid !important so themes can cascade normally',
    (selector) => {
      const block = blocks.find((b) => b.selector === selector);
      const vars = extractVars(block.body);
      for (const [name, value] of Object.entries(vars)) {
        expect(
          value.endsWith('!important'),
          `${selector} var ${name} = "${value}" should not use !important`,
        ).toBe(false);
      }
    }
  );

  it('blue-light background (#f0f5fa) differs from :root default (#0a0e1a)', () => {
    // Regression guard: if someone reverts blue-light to use the same
    // colors as :root, the user would see no visual change when switching.
    const root = extractVars(extractBlocks(STYLES).find((b) => b.selector === ':root').body);
    const blue = extractVars(
      extractBlocks(STYLES).find((b) => b.selector === '[data-theme="blue-light"]').body,
    );
    expect(stripImportant(blue['--bg-primary'])).not.toBe(stripImportant(root['--bg-primary']));
    expect(stripImportant(blue['--bg-primary'])).toBe('#f0f5fa');
  });
});
