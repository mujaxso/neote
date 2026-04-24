import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export interface Tab {
  id: string;
  title: string;
  isDirty: boolean;
}

interface TabsState {
  tabs: Tab[];
  activeTabId: string | null;
  openFile: (id: string, title: string) => void;
  closeTab: (id: string) => void;
  setActiveTab: (id: string) => void;
  markDirty: (id: string) => void;
  markClean: (id: string) => void;
}

export const useTabsStore = create<TabsState>()(
  devtools(
    (set, get) => ({
      tabs: [],
      activeTabId: null,

      openFile: (id, title) => {
        const { tabs } = get();
        const existing = tabs.find((t) => t.id === id);
        if (existing) {
          set({ activeTabId: id });
          return;
        }
        const newTab: Tab = { id, title, isDirty: false };
        set({
          tabs: [...tabs, newTab],
          activeTabId: id,
        });
      },

      closeTab: (id) => {
        const { tabs, activeTabId } = get();
        const tab = tabs.find((t) => t.id === id);
        if (!tab) return;
        // prevent closing dirty tab without confirmation
        if (tab.isDirty) {
          const confirmed = window.confirm(
            `${tab.title} has unsaved changes. Close anyway?`
          );
          if (!confirmed) {
            return; // user cancelled closing
          }
        }
        const idx = tabs.findIndex((t) => t.id === id);
        if (idx === -1) return;

        const newTabs = tabs.filter((t) => t.id !== id);
        let newActive = activeTabId;
        if (activeTabId === id) {
          if (idx < newTabs.length) {
            newActive = newTabs[idx].id;
          } else if (newTabs.length > 0) {
            newActive = newTabs[newTabs.length - 1].id;
          } else {
            newActive = null;
          }
        }
        set({ tabs: newTabs, activeTabId: newActive });
      },

      setActiveTab: (id) => {
        set({ activeTabId: id });
      },

      markDirty: (id) => {
        set((state) => ({
          tabs: state.tabs.map((t) =>
            t.id === id ? { ...t, isDirty: true } : t
          ),
        }));
      },

      markClean: (id) => {
        set((state) => ({
          tabs: state.tabs.map((t) =>
            t.id === id ? { ...t, isDirty: false } : t
          ),
        }));
      },
    }),
    { name: 'tabs-store' }
  )
);
