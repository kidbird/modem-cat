// theme module — extracted from app.js
    // ── Theme ──
    (function () {
      const saved = localStorage.getItem('theme');
      setTheme(saved === 'light' ? 'light' : 'dark');
    })();

    function setTheme(theme) {
      document.documentElement.setAttribute('data-theme', theme);
      localStorage.setItem('theme', theme);
      updateThemeToggle(theme);
      if (typeof redrawCharts === 'function') {
        try { redrawCharts(); } catch(_) {}
      }
    }

    // Pass theme as a parameter (single source of truth = the argument to setTheme).
    // Previously this re-read localStorage, which created multiple sources of truth
    // (setTheme arg / data-theme attribute / localStorage) and could desync.
    function updateThemeToggle(theme) {
      const darkBtn = document.getElementById('themeDark');
      const lightBtn = document.getElementById('themeLight');
      if (darkBtn) darkBtn.classList.toggle('active', theme === 'dark');
      if (lightBtn) lightBtn.classList.toggle('active', theme === 'light');
    }
