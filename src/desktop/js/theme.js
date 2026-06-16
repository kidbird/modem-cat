// theme module — extracted from app.js
    // ── Theme ──
    (function () {
      const saved = localStorage.getItem('theme');
      if (saved === 'light') setTheme('light');
      else if (saved === 'blue-light') setTheme('blue-light');
      else setTheme('dark');
    })();

    function toggleTheme() {
      const current = localStorage.getItem('theme') || 'dark';
      if (current === 'dark') setTheme('light');
      else if (current === 'light') setTheme('blue-light');
      else setTheme('dark');
    }

    function setTheme(theme) {
      // state.isDark was a 2-state flag that no longer reflects the 3 themes
      // (dark/light/blue-light) — toggleTheme + updateThemeToggle both read
      // localStorage directly. Drop the dead-write; the `isDark: true` field
      // in the state literal above is harmless initial seed.
      document.documentElement.setAttribute('data-theme', theme);
      localStorage.setItem('theme', theme);
      updateThemeToggle(theme);
    }

    // Pass theme as a parameter (single source of truth = the argument to setTheme).
    // Previously this re-read localStorage, which created 3 sources of truth
    // (setTheme arg / data-theme attribute / localStorage) and could desync.
    function updateThemeToggle(theme) {
      const darkBtn = document.getElementById('themeDark');
      const lightBtn = document.getElementById('themeLight');
      const blueLightBtn = document.getElementById('themeBlueLight');
      if (darkBtn) darkBtn.classList.toggle('active', theme === 'dark');
      if (lightBtn) lightBtn.classList.toggle('active', theme === 'light');
      if (blueLightBtn) blueLightBtn.classList.toggle('active', theme === 'blue-light');
    }
