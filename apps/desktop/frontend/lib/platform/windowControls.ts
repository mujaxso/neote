import { getCurrent } from '@tauri-apps/api/window';

export async function isTauri(): Promise<boolean> {
  return typeof window !== 'undefined' && '__TAURI__' in window;
}

export async function setupWindowControls() {
  if (await isTauri()) {
    try {
      // Enable custom window controls
      const appWindow = getCurrent();
      
      // Make the top bar draggable
      const dragRegions = document.querySelectorAll('[data-tauri-drag-region]');
      dragRegions.forEach(region => {
        region.addEventListener('mousedown', (e) => {
          // Check if the click is on a button or element with data-no-drag attribute
          const target = e.target as HTMLElement;
          if (target.closest('[data-no-drag="true"]')) {
            return;
          }
          // Start window dragging
          appWindow.startDragging();
        });
      });
    } catch (error) {
      console.error('Error setting up window controls:', error);
    }
  }
}

export const windowControlActions = {
  minimize: async () => {
    if (await isTauri()) {
      try {
        const window = getCurrent();
        await window.minimize();
      } catch (error) {
        console.error('Error minimizing window:', error);
      }
    }
  },
  maximize: async () => {
    if (await isTauri()) {
      try {
        const window = getCurrent();
        const isMaximized = await window.isMaximized();
        if (isMaximized) {
          await window.unmaximize();
        } else {
          await window.maximize();
        }
      } catch (error) {
        console.error('Error toggling maximize:', error);
      }
    }
  },
  close: async () => {
    if (await isTauri()) {
      try {
        const window = getCurrent();
        await window.close();
      } catch (error) {
        console.error('Error closing window:', error);
      }
    }
  },
};
