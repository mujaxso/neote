import { useTabsStore } from './store';
import { cn } from '@/lib/utils';
import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';

interface TabItemProps {
  tab: { id: string; title: string; isDirty: boolean };
  isActive: boolean;
}

export function TabItem({ tab, isActive }: TabItemProps) {
  const { setActiveTab, closeTab } = useTabsStore();

  const handleTabClick = async () => {
    setActiveTab(tab.id);
    useWorkspaceStore.getState().setActiveFilePath(tab.id);
    try {
      await WorkspaceService.openFileInEditor(tab.id);
    } catch (error) {
      console.error('Failed to load file:', error);
    }
  };

  const handleMiddleClick = (e: React.MouseEvent) => {
    if (e.button === 1) {
      e.preventDefault();
      closeTab(tab.id);
    }
  };

  return (
    <div
      className={cn(
        'group relative flex items-center gap-1.5 px-3 py-2 text-sm font-mono cursor-default select-none border-b-2 transition-colors',
        isActive
          ? 'bg-panel text-editor-foreground border-b-accent'
          : 'bg-panel text-muted-foreground hover:bg-elevated-panel border-b-transparent hover:border-b-hover'
      )}
      onClick={handleTabClick}
      onMouseDown={handleMiddleClick}
      data-tab-id={tab.id}
      data-no-drag="true"
    >
      {/* file name – truncated */}
      <span className="truncate max-w-[180px]" title={tab.title}>
        {tab.title}
      </span>

      {/* dirty indicator */}
      {tab.isDirty && (
        <span className="w-1.5 h-1.5 rounded-full bg-amber-400 flex-shrink-0" />
      )}

      {/* close button – visible on hover */}
      <button
        className={cn(
          'ml-auto flex-shrink-0 rounded-sm p-0.5 hover:bg-hover-bg text-muted-foreground/50 hover:text-foreground transition-opacity',
          isActive ? 'opacity-70 hover:opacity-100' : 'opacity-0 group-hover:opacity-70'
        )}
        onClick={(e) => {
          e.stopPropagation();
          closeTab(tab.id);
        }}
        aria-label="Close tab"
        data-no-drag="true"
      >
        <svg
          className="w-3 h-3"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
        >
          <path d="M18 6L6 18M6 6l12 12" />
        </svg>
      </button>
    </div>
  );
}
