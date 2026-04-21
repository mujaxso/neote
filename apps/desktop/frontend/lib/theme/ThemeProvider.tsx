import { ReactNode, useEffect } from 'react';
import { useThemeStore, initializeTheme } from './theme-store';

interface ThemeProviderProps {
  children: ReactNode;
}

export function ThemeProvider({ children }: ThemeProviderProps) {
  const { isLoading } = useThemeStore();
  
  useEffect(() => {
    const cleanup = initializeTheme();
    return cleanup;
  }, []);
  
  // Prevent flash of unstyled content
  if (isLoading) {
    return (
      <div className="fixed inset-0 bg-app flex items-center justify-center">
        <div className="text-muted">Loading theme...</div>
      </div>
    );
  }
  
  return <>{children}</>;
}
