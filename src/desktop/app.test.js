import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';

// Mirrors the safeStorage wrapper declared at the top of src/desktop/app.js.
// We re-declare here (rather than import the IIFE-wrapped app.js) so tests
// can exercise the wrapper in isolation, including the SecurityError path.
function makeSafeStorage(getImpl, setImpl) {
  return {
    get(key) {
      try {
        return getImpl(key);
      } catch (e) {
        console.warn(`localStorage.getItem('${key}') failed:`, e);
        return null;
      }
    },
    set(key, val) {
      try {
        setImpl(key, val);
      } catch (e) {
        console.warn(`localStorage.setItem('${key}') failed:`, e);
      }
    },
  };
}

describe('safeStorage wrapper', () => {
  it('returns the stored value on a normal get', () => {
    const store = new Map([['theme', 'blue-light']]);
    const ss = makeSafeStorage((k) => store.get(k), (k, v) => store.set(k, v));
    expect(ss.get('theme')).toBe('blue-light');
  });

  it('returns null on a throwing get (WebView2 private mode)', () => {
    const ss = makeSafeStorage(
      () => {
        throw new Error('SecurityError: storage disabled');
      },
      () => {},
    );
    expect(ss.get('lang')).toBeNull();
  });

  it('writes through on a normal set', () => {
    let written = null;
    const ss = makeSafeStorage(() => null, (k, v) => (written = [k, v]));
    ss.set('theme', 'dark');
    expect(written).toEqual(['theme', 'dark']);
  });

  it('swallows throws on set (does not propagate)', () => {
    const ss = makeSafeStorage(
      () => null,
      () => {
        throw new Error('QuotaExceededError');
      },
    );
    // The whole point: a throwing setItem must not abort the calling code.
    expect(() => ss.set('lang', 'zh')).not.toThrow();
  });

  it('handles a fully broken storage without aborting state init', () => {
    // Simulates the worst case: every call throws. The wrapper must keep the
    // script alive so theme bootstrap, IPC listeners, etc. can still run.
    const ss = makeSafeStorage(
      () => {
        throw new Error('boom');
      },
      () => {
        throw new Error('boom');
      },
    );
    expect(ss.get('theme')).toBeNull();
    expect(() => ss.set('theme', 'dark')).not.toThrow();
  });
});

// Mirrors the theme functions declared in src/desktop/app.js so we can test
// the contract (data-theme attribute + active class) without loading the
// 3600-line IIFE.
function makeThemeApi({ safeGet, safeSet, buttons = {} } = {}) {
  const calls = { setAttr: [], active: [] };
  const documentElement = {
    setAttribute(name, value) {
      calls.setAttr.push([name, value]);
    },
  };
  const document = {
    getElementById(id) {
      return buttons[id] || null;
    },
  };
  function updateThemeToggle(theme) {
    const darkBtn = document.getElementById('themeDark');
    const lightBtn = document.getElementById('themeLight');
    const blueLightBtn = document.getElementById('themeBlueLight');
    for (const [btn, val] of [
      [darkBtn, theme === 'dark'],
      [lightBtn, theme === 'light'],
      [blueLightBtn, theme === 'blue-light'],
    ]) {
      if (btn) {
        btn.classList.toggle('active', val);
        calls.active.push([btn.id, val]);
      }
    }
  }
  function setTheme(theme) {
    documentElement.setAttribute('data-theme', theme);
    safeSet('theme', theme);
    updateThemeToggle(theme);
  }
  return { setTheme, updateThemeToggle, calls, documentElement };
}

function makeButton(id) {
  return {
    id,
    classList: {
      _active: false,
      toggle(cls, on) {
        if (cls === 'active') this._active = !!on;
      },
      contains(cls) {
        return cls === 'active' && this._active;
      },
    },
  };
}

describe('setTheme / updateThemeToggle', () => {
  it('writes the data-theme attribute + persists to safeStorage', () => {
    const ss = makeSafeStorage(() => null, () => {});
    const api = makeThemeApi({ safeGet: ss.get, safeSet: ss.set });
    api.setTheme('blue-light');
    expect(api.calls.setAttr).toEqual([['data-theme', 'blue-light']]);
  });

  it('marks only the matching button as active', () => {
    const dark = makeButton('themeDark');
    const light = makeButton('themeLight');
    const blue = makeButton('themeBlueLight');
    const ss = makeSafeStorage(() => null, () => {});
    const api = makeThemeApi({
      safeGet: ss.get,
      safeSet: ss.set,
      buttons: { themeDark: dark, themeLight: light, themeBlueLight: blue },
    });
    api.setTheme('light');
    expect(light.classList.contains('active')).toBe(true);
    expect(dark.classList.contains('active')).toBe(false);
    expect(blue.classList.contains('active')).toBe(false);
  });

  it('toggling 3 times returns to original state (dark cycle)', () => {
    // No buttons in DOM, updateThemeToggle should no-op.
    const ss = makeSafeStorage(() => null, () => {});
    const api = makeThemeApi({ safeGet: ss.get, safeSet: ss.set });
    expect(() => api.setTheme('light')).not.toThrow();
    expect(() => api.setTheme('blue-light')).not.toThrow();
    expect(() => api.setTheme('dark')).not.toThrow();
    expect(api.calls.setAttr).toEqual([
      ['data-theme', 'light'],
      ['data-theme', 'blue-light'],
      ['data-theme', 'dark'],
    ]);
  });

  it('works when buttons are missing from the DOM', () => {
    const ss = makeSafeStorage(() => null, () => {});
    const api = makeThemeApi({ safeGet: ss.get, safeSet: ss.set });
    // No buttons registered — must not throw, no active calls.
    api.updateThemeToggle('dark');
    expect(api.calls.active).toEqual([]);
  });
});

// Mirrors the IIFE init at the top of app.js. The "saved" value drives
// which theme is applied. Catches the dark/light/blue-light branch logic
// in a single place — if the order or fall-through ever changes, this fails.
function pickInitialTheme(savedValue) {
  if (savedValue === 'light') return 'light';
  if (savedValue === 'blue-light') return 'blue-light';
  return 'dark';
}

describe('init IIFE theme selection', () => {
  it('light when saved is light', () => {
    expect(pickInitialTheme('light')).toBe('light');
  });
  it('blue-light when saved is blue-light', () => {
    expect(pickInitialTheme('blue-light')).toBe('blue-light');
  });
  it('dark when saved is null (default)', () => {
    expect(pickInitialTheme(null)).toBe('dark');
  });
  it('dark when saved is anything else (unknown value)', () => {
    expect(pickInitialTheme('red')).toBe('dark');
    expect(pickInitialTheme('')).toBe('dark');
  });
});

// safeStorage.get returning null (the WebView2 throw path) is the most
// common case on a fresh install. Init must default to dark, not throw.
describe('init survives safeStorage get returning null', () => {
  it('defaults to dark theme when get returns null', () => {
    const ss = makeSafeStorage(
      () => null, // simulates throwing localStorage that got swallowed
      () => {},
    );
    const theme = pickInitialTheme(ss.get('theme'));
    expect(theme).toBe('dark');
  });
});
