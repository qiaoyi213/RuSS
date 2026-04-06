import { ref, watch } from 'vue';

const THEME_STORAGE_KEY = 'russ-theme';
const isDarkMode = ref(false);
let initialized = false;

function getInitialTheme(): boolean {
  const stored = window.localStorage.getItem(THEME_STORAGE_KEY);
  if (stored === 'dark') {
    return true;
  }
  if (stored === 'light') {
    return false;
  }

  return window.matchMedia('(prefers-color-scheme: dark)').matches;
}

function applyTheme(isDark: boolean): void {
  document.documentElement.dataset.theme = isDark ? 'dark' : 'light';
  document.documentElement.style.colorScheme = isDark ? 'dark' : 'light';
}

function initTheme(): void {
  if (initialized) {
    return;
  }

  isDarkMode.value = getInitialTheme();
  applyTheme(isDarkMode.value);

  watch(isDarkMode, (value) => {
    window.localStorage.setItem(THEME_STORAGE_KEY, value ? 'dark' : 'light');
    applyTheme(value);
  });

  initialized = true;
}

export function useTheme() {
  initTheme();

  function setDarkMode(value: boolean): void {
    isDarkMode.value = value;
  }

  function toggleDarkMode(): void {
    isDarkMode.value = !isDarkMode.value;
  }

  return {
    isDarkMode,
    setDarkMode,
    toggleDarkMode,
  };
}
