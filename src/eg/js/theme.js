/* ============================================================
   SCHOOL ERP - THEME ENGINE
   Handles dark/light toggle + accent color switching
   ============================================================ */

const ThemeEngine = (() => {
  // Predefined accent colors
  const ACCENT_COLORS = {
    indigo:  { h: 245, s: 75, l: 55 },
    blue:    { h: 221, s: 83, l: 53 },
    teal:    { h: 173, s: 58, l: 39 },
    emerald: { h: 142, s: 71, l: 45 },
    violet:  { h: 262, s: 83, l: 58 },
    rose:    { h: 346, s: 84, l: 50 },
    orange:  { h: 24,  s: 95, l: 53 },
    amber:   { h: 38,  s: 92, l: 50 },
    cyan:    { h: 199, s: 89, l: 48 },
    pink:    { h: 322, s: 81, l: 55 },
    red:     { h: 0,   s: 72, l: 51 },
    slate:   { h: 215, s: 25, l: 27 },
  };

  function hslToRgb(h, s, l) {
    s /= 100; l /= 100;
    const a = s * Math.min(l, 1 - l);
    const f = (n) => {
      const k = (n + h / 30) % 12;
      return l - a * Math.max(Math.min(k - 3, 9 - k, 1), -1);
    };
    return [Math.round(f(0) * 255), Math.round(f(8) * 255), Math.round(f(4) * 255)];
  }

  function getStoredTheme() {
    return localStorage.getItem('erp-theme') || 'light';
  }

  function getStoredAccent() {
    return localStorage.getItem('erp-accent') || 'indigo';
  }

  function applyTheme(mode) {
    document.documentElement.setAttribute('data-bs-theme', mode);
    localStorage.setItem('erp-theme', mode);

    // Update toggle button icons
    const sunIcon = document.getElementById('theme-icon-sun');
    const moonIcon = document.getElementById('theme-icon-moon');
    if (sunIcon && moonIcon) {
      if (mode === 'dark') {
        sunIcon.style.display = 'none';
        moonIcon.style.display = 'inline';
      } else {
        sunIcon.style.display = 'inline';
        moonIcon.style.display = 'none';
      }
    }
  }

  function applyAccent(colorName) {
    const color = ACCENT_COLORS[colorName];
    if (!color) return;

    const root = document.documentElement;
    root.style.setProperty('--accent-h', color.h);
    root.style.setProperty('--accent-s', color.s + '%');
    root.style.setProperty('--accent-l', color.l + '%');

    const rgb = hslToRgb(color.h, color.s, color.l);
    root.style.setProperty('--accent-rgb', rgb.join(', '));

    localStorage.setItem('erp-accent', colorName);

    // Update active swatch
    document.querySelectorAll('.accent-swatch').forEach(el => {
      el.classList.toggle('active', el.dataset.color === colorName);
    });
  }

  function toggleTheme() {
    const current = document.documentElement.getAttribute('data-bs-theme');
    applyTheme(current === 'dark' ? 'light' : 'dark');
  }

  function init() {
    // Apply stored preferences immediately
    applyTheme(getStoredTheme());
    applyAccent(getStoredAccent());

    // Theme toggle button
    document.addEventListener('click', (e) => {
      if (e.target.closest('#theme-toggle')) {
        toggleTheme();
      }
    });

    // Accent swatches
    document.addEventListener('click', (e) => {
      const swatch = e.target.closest('.accent-swatch');
      if (swatch && swatch.dataset.color) {
        applyAccent(swatch.dataset.color);
      }
    });

    // Sidebar toggle (mobile)
    document.addEventListener('click', (e) => {
      if (e.target.closest('.sidebar-toggler')) {
        document.querySelector('.sidebar')?.classList.toggle('show');
        document.querySelector('.sidebar-overlay')?.classList.toggle('show');
      }
      if (e.target.closest('.sidebar-overlay')) {
        document.querySelector('.sidebar')?.classList.remove('show');
        document.querySelector('.sidebar-overlay')?.classList.remove('show');
      }
    });
  }

  // Initialize when DOM is ready
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }

  return { applyTheme, applyAccent, toggleTheme, ACCENT_COLORS };
})();
