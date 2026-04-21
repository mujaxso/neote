import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ZaroxiTheme, ThemeData, ThemeSettings } from './types';

interface ThemeStore {
  themeMode: ZaroxiTheme;
  isDark: boolean;
  isSystem: boolean;
  isLoading: boolean;
  
  // Actions
  setThemeMode: (mode: ZaroxiTheme) => Promise<void>;
  loadThemeSettings: () => Promise<void>;
  applyTheme: (data: { mode: ZaroxiTheme; isDark: boolean }) => void;
}

export const useThemeStore = create<ThemeStore>()(
  persist(
    (set, get) => ({
      themeMode: 'system',
      isDark: true,
      isSystem: true,
      isLoading: false,
      
      setThemeMode: async (mode) => {
        set({ isLoading: true });
        try {
          await invoke('set_theme', { theme: mode });
          
          // Determine if dark based on mode
          const isSystem = mode === 'system';
          const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const isDark = mode === 'dark' || (isSystem && systemPrefersDark);
          
          set({ 
            themeMode: mode, 
            isDark,
            isSystem,
            isLoading: false 
          });
          
          // Update CSS variables
          updateCssVariables(isDark);
        } catch (error) {
          console.error('Failed to set theme:', error);
          set({ isLoading: false });
        }
      },
      
      loadThemeSettings: async () => {
        set({ isLoading: true });
        try {
          const settings: ThemeSettings = await invoke('load_theme_settings');
          const currentTheme: ZaroxiTheme = await invoke('get_current_theme');
          
          const isSystem = currentTheme === 'system';
          const systemPrefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
          const isDark = currentTheme === 'dark' || (isSystem && systemPrefersDark);
          
          set({ 
            themeMode: currentTheme,
            isDark,
            isSystem,
            isLoading: false 
          });
          
          updateCssVariables(isDark);
        } catch (error) {
          console.error('Failed to load theme settings:', error);
          set({ isLoading: false });
        }
      },
      
      applyTheme: (data) => {
        const isSystem = data.mode === 'system';
        set({
          themeMode: data.mode,
          isDark: data.isDark,
          isSystem,
        });
        updateCssVariables(data.isDark);
      },
    }),
    {
      name: 'zaroxi-theme-storage',
      partialize: (state) => ({
        themeMode: state.themeMode,
        isDark: state.isDark,
      }),
    }
  )
);

// Listen to theme changes from backend
export function setupThemeListener() {
  return listen<{ mode: ZaroxiTheme; isDark: boolean }>('theme:changed', (event) => {
    useThemeStore.getState().applyTheme(event.payload);
  });
}

// Update CSS custom properties based on theme
function updateCssVariables(isDark: boolean) {
  const root = document.documentElement;
  
  if (isDark) {
    root.classList.add('dark');
    root.classList.remove('light');
  } else {
    root.classList.add('light');
    root.classList.remove('dark');
  }
  
  // Set data attribute for CSS selectors
  root.setAttribute('data-theme', isDark ? 'dark' : 'light');
}

// Initialize theme on app start
export function initializeTheme() {
  const store = useThemeStore.getState();
  
  // Load saved theme
  store.loadThemeSettings();
  
  // Listen to system theme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const handleSystemThemeChange = (e: MediaQueryListEvent) => {
    const { themeMode } = useThemeStore.getState();
    if (themeMode === 'system') {
      updateCssVariables(e.matches);
      useThemeStore.getState().applyTheme({ 
        mode: 'system', 
        isDark: e.matches 
      });
    }
  };
  
  mediaQuery.addEventListener('change', handleSystemThemeChange);
  
  // Setup backend listener
  setupThemeListener();
  
  return () => {
    mediaQuery.removeEventListener('change', handleSystemThemeChange);
  };
}
