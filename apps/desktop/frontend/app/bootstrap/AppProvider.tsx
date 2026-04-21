import { ReactNode, useEffect } from 'react';
import { ErrorBoundary } from './ErrorBoundary';
import { TauriProvider } from './TauriProvider';
import { KeyboardShortcutsProvider } from '@/lib/keyboard/KeyboardShortcutsProvider';
import { FontLoader } from './FontLoader';
import { ThemeProvider } from '@/lib/theme/ThemeProvider';
import '@/styles/tokens.css';

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  return (
    <ErrorBoundary>
      <TauriProvider>
        <KeyboardShortcutsProvider>
          <ThemeProvider>
            <FontLoader />
            <div className="font-sans antialiased bg-app text-primary">
              {children}
            </div>
          </ThemeProvider>
        </KeyboardShortcutsProvider>
      </TauriProvider>
    </ErrorBoundary>
  );
}
