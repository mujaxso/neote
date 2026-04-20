import { ReactNode, useEffect } from 'react';
import { ErrorBoundary } from './ErrorBoundary';
import { TauriProvider } from './TauriProvider';
import { KeyboardShortcutsProvider } from '@/lib/keyboard/KeyboardShortcutsProvider';
import { FontLoader } from './FontLoader';

interface AppProviderProps {
  children: ReactNode;
}

export function AppProvider({ children }: AppProviderProps) {
  useEffect(() => {
    // Add debug class to body for font checking
    document.body.classList.add('debug-font-check');
    
    // Check which font is actually being used
    const checkFont = () => {
      const testElement = document.createElement('div');
      testElement.style.fontFamily = 'JetBrainsMonoNL Nerd Font Mono';
      testElement.style.position = 'absolute';
      testElement.style.opacity = '0';
      testElement.style.pointerEvents = 'none';
      testElement.textContent = 'Test';
      document.body.appendChild(testElement);
      
      setTimeout(() => {
        const computed = window.getComputedStyle(testElement);
        console.log('Actual font being used:', computed.fontFamily);
        document.body.removeChild(testElement);
      }, 100);
    };
    
    checkFont();
  }, []);

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
