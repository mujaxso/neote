import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';

export function StatusBar() {
  const { currentWorkspace, isLoading } = useWorkspaceStore();
  
  return (
    <div className="h-6 border-t border-divider bg-status-bar flex items-center justify-between px-3 text-xs font-sans">
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-1">
          <Icon name="workspace" size={12} className="text-muted" label="Workspace" />
          <span className="text-muted">Workspace:</span>
          <span className="font-medium text-primary">
            {currentWorkspace ? currentWorkspace.name : 'No workspace open'}
          </span>
          {currentWorkspace && (
            <span className="text-muted ml-1 font-mono text-[10px]">
              ({currentWorkspace.rootPath})
            </span>
          )}
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-1">
            <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
            <span className="text-muted">Loading...</span>
          </div>
        )}
      </div>
      
      <div className="flex items-center space-x-4 font-mono">
        <div className="flex items-center space-x-1">
          <Icon name="file-code" size={12} className="text-muted" label="Encoding" />
          <span className="text-primary">UTF-8</span>
        </div>
        <div className="flex items-center space-x-1">
          <Icon name="indent" size={12} className="text-muted" label="Indentation" />
          <span className="text-primary">Spaces: 2</span>
        </div>
        <div className="flex items-center space-x-1">
          <Icon name="cursor" size={12} className="text-muted" label="Cursor position" />
          <span className="text-primary">LN 1, COL 1</span>
        </div>
        {/* Font test indicator */}
        <div className="flex items-center space-x-1">
          <Icon name="check" size={12} className="text-success" label="Font loaded" />
          <span className="text-primary text-[10px]">Nerd Font</span>
        </div>
      </div>
    </div>
  );
}
