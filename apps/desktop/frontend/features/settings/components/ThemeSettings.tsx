import { useThemeStore } from '@/lib/theme/theme-store';
import { ZaroxiTheme } from '@/lib/theme/types';

export function ThemeSettings() {
  const { themeMode, setThemeMode, isLoading } = useThemeStore();
  
  const themes: { value: ZaroxiTheme; label: string; icon: string }[] = [
    { 
      value: 'system', 
      label: 'System', 
      icon: '🖥️'
    },
    { 
      value: 'light', 
      label: 'Light', 
      icon: '☀️'
    },
    { 
      value: 'dark', 
      label: 'Dark', 
      icon: '🌙'
    },
  ];
  
  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-lg font-semibold text-primary">Theme</h3>
          <p className="text-sm text-secondary mt-1">
            Choose how Zaroxi looks. System will follow your OS preference.
          </p>
        </div>
      </div>
      
      <div className="grid grid-cols-3 gap-4">
        {themes.map((theme) => (
          <button
            key={theme.value}
            onClick={() => setThemeMode(theme.value)}
            disabled={isLoading}
            className={`flex flex-col items-center justify-center p-5 rounded-lg border-2 transition-all ${
              themeMode === theme.value
                ? 'border-accent bg-accent-soft-background shadow-sm'
                : 'border-border hover:border-accent/40 hover:bg-hover-bg'
            }`}
          >
            <div className={`p-4 rounded-full mb-4 text-3xl ${
              themeMode === theme.value
                ? 'bg-accent/15 text-accent'
                : 'bg-panel text-secondary'
            }`}>
              {theme.icon}
            </div>
            <span className={`text-sm font-semibold ${
              themeMode === theme.value ? 'text-accent' : 'text-primary'
            }`}>
              {theme.label}
            </span>
          </button>
        ))}
      </div>
      
      <div className="pt-5 border-t border-divider">
        <div className="flex items-center justify-between">
          <div>
            <h4 className="text-sm font-semibold text-primary">Current Theme</h4>
            <p className="text-sm text-secondary mt-0.5">
              {themeMode === 'system' && 'Following system theme'}
              {themeMode === 'light' && 'Light theme active'}
              {themeMode === 'dark' && 'Dark theme active'}
            </p>
          </div>
          <div className="px-4 py-1.5 rounded-full bg-panel border border-border text-sm font-medium text-primary">
            {themeMode.charAt(0).toUpperCase() + themeMode.slice(1)}
          </div>
        </div>
      </div>
    </div>
  );
}
