import { useState } from 'react';
import { cn } from '@/lib/utils';
import { useWorkbenchStore } from '../store/workbenchStore';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { invoke } from '@tauri-apps/api/core';

interface MenuItem {
  label: string;
  action: () => void;
}

export function MenuBar() {
  const [openMenu, setOpenMenu] = useState<string | null>(null);

  const handleOpenWorkspace = async () => {
    console.log('handleOpenWorkspace triggered');
    try {
      const result: { selected_path: string | null } = await invoke('open_file_dialog');
      console.log('Dialog result:', result);
      if (result.selected_path) {
        const selectedPath = result.selected_path;
        // 1. Open workspace via backend (emits workspace:opened event)
        await invoke('open_workspace', { path: selectedPath });

        // 2. Manually fetch the first‑level directory listing
        const entries: {
          name: string;
          path: string;
          is_dir: boolean;
          file_type: string | null;
          size: number | null;
          modified: string | null;
        }[] = await invoke('list_directory', { path: selectedPath });

        // 3. Convert to the ExplorerTreeNode shape expected by the front‑end
        const treeNodes = entries.map((e) => ({
          name: e.name,
          path: e.path,
          is_dir: e.is_dir,
          children: [],
        }));

        // 4. Update the workspace store
        const { setRootPath, setTree } = useWorkspaceStore.getState();
        setRootPath(selectedPath);
        setTree(treeNodes);

        // 5. Open the explorer panel
        useWorkbenchStore.getState().togglePanel('explorer');
      }
    } catch (e) {
      console.error('Failed to open workspace:', e);
    }
  };

  const menus: { label: string; items: MenuItem[] }[] = [
    {
      label: 'File',
      items: [
        { label: 'Open Workspace', action: handleOpenWorkspace },
        { label: 'New File', action: () => {} },
        { label: 'Save', action: () => {} },
      ],
    },
    {
      label: 'Edit',
      items: [
        { label: 'Undo', action: () => {} },
        { label: 'Redo', action: () => {} },
      ],
    },
    {
      label: 'View',
      items: [
        { label: 'Toggle Sidebar', action: () => {} },
      ],
    },
    {
      label: 'Tools',
      items: [
        { label: 'Settings', action: () => {} },
      ],
    },
  ];

  const toggleMenu = (label: string) => {
    if (openMenu === label) {
      setOpenMenu(null);
    } else {
      setOpenMenu(label);
    }
  };

  const closeAll = () => setOpenMenu(null);

  return (
    <div className="flex items-center h-10 bg-title-bar text-title-bar-foreground select-none" onMouseLeave={closeAll}>
      {menus.map((menu) => (
        <div key={menu.label} className="relative">
          <button
            className={cn(
              'px-3 py-1 text-xs font-medium hover:bg-hover-bg rounded-sm transition-colors',
              openMenu === menu.label && 'bg-hover-bg'
            )}
            onClick={() => toggleMenu(menu.label)}
          >
            {menu.label}
          </button>
          {openMenu === menu.label && (
            <div className="absolute top-full left-0 mt-0 bg-panel shadow-lg border border-border rounded-md py-1 z-50 min-w-[180px]">
              {menu.items.map((item) => (
                <button
                  key={item.label}
                  className="w-full px-3 py-1.5 text-left text-sm hover:bg-hover-bg transition-colors"
                  onClick={() => {
                    item.action();
                    closeAll();
                  }}
                >
                  {item.label}
                </button>
              ))}
            </div>
          )}
        </div>
      ))}
    </div>
  );
}
