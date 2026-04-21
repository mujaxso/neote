import { ReactNode } from 'react';
import { ErrorBoundary } from './ErrorBoundary';
import { TauriProvider } from './TauriProvider';
import { KeyboardShortcutsProvider } from '@/lib/keyboard/KeyboardShortcutsProvider';
import { FontLoader } from './FontLoader';

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  return (
    <ErrorBoundary>
      <TauriProvider>
        <KeyboardShortcutsProvider>
          <FontLoader />
          <div className="font-sans antialiased">
            {children}
          </div>
        </KeyboardShortcutsProvider>
      </TauriProvider>
    </ErrorBoundary>
  );
}
