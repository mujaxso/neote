import { useState } from 'react';
import { Menu, Settings, Sun, Moon, Monitor } from 'lucide-react';
import { useThemeStore } from '@/lib/theme/theme-store';
import { ZaroxiTheme } from '@/lib/theme/types';

export function Toolbar() {
  const { themeMode, setThemeMode } = useThemeStore();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  
  const handleThemeChange = (theme: ZaroxiTheme) => {
    setThemeMode(theme);
    setIsMenuOpen(false);
  };
  
  const themeIcon = {
    system: <Monitor className="w-4 h-4" />,
    light: <Sun className="w-4 h-4" />,
    dark: <Moon className="w-4 h-4" />,
  }[themeMode];
  
  return (
    <div className="h-12 bg-title-bar border-b border-border flex items-center justify-between px-4">
      {/* Left side */}
      <div className="flex items-center gap-4">
        <button 
          className="p-2 rounded hover:bg-hover-bg text-primary"
          onClick={() => setIsMenuOpen(!isMenuOpen)}
        >
          <Menu className="w-5 h-5" />
        </button>
        
        <div className="text-sm font-medium text-primary">
          Zaroxi IDE
        </div>
      </div>
      
      {/* Right side */}
      <div className="flex items-center gap-2">
        {/* Theme switcher dropdown */}
        <div className="relative">
          <button 
            className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-border hover:bg-hover-bg text-primary"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            {themeIcon}
            <span className="text-sm capitalize">{themeMode}</span>
          </button>
          
          {isMenuOpen && (
            <div className="absolute right-0 top-full mt-1 w-48 bg-panel border border-border rounded-lg shadow-lg z-50">
              <div className="py-1">
                <button
                  onClick={() => handleThemeChange('system')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'system' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <Monitor className="w-4 h-4" />
                  <span>System</span>
                </button>
                <button
                  onClick={() => handleThemeChange('light')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'light' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <Sun className="w-4 h-4" />
                  <span>Light</span>
                </button>
                <button
                  onClick={() => handleThemeChange('dark')}
                  className={`w-full text-left px-4 py-2 flex items-center gap-3 hover:bg-hover-bg ${
                    themeMode === 'dark' ? 'bg-selected-bg text-accent' : 'text-primary'
                  }`}
                >
                  <Moon className="w-4 h-4" />
                  <span>Dark</span>
                </button>
              </div>
            </div>
          )}
        </div>
        
        <button 
          className="p-2 rounded hover:bg-hover-bg text-primary"
          onClick={() => {
            // Emit event to open settings
            window.dispatchEvent(new CustomEvent('open-settings'));
          }}
        >
          <Settings className="w-5 h-5" />
        </button>
      </div>
    </div>
  );
}
