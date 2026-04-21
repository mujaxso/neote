import { useThemeStore } from '@/lib/theme/theme-store';
import { ZaroxiTheme } from '@/lib/theme/types';

export function ThemeSwitcher() {
  const { themeMode, setThemeMode, isLoading } = useThemeStore();
  
  const themes: { value: ZaroxiTheme; label: string }[] = [
    { value: 'system', label: 'System' },
    { value: 'light', label: 'Light' },
    { value: 'dark', label: 'Dark' },
  ];
  
  return (
    <div className="flex items-center gap-2 p-2 bg-panel rounded-lg border">
      <span className="text-sm text-muted">Theme:</span>
      <div className="flex gap-1">
        {themes.map((theme) => (
          <button
            key={theme.value}
            onClick={() => setThemeMode(theme.value)}
            disabled={isLoading}
            className={`px-3 py-1 text-sm rounded-md transition-colors ${
              themeMode === theme.value
                ? 'bg-accent text-on-accent'
                : 'bg-hover hover:bg-active text-secondary'
            }`}
          >
            {theme.label}
          </button>
        ))}
      </div>
    </div>
  );
}
