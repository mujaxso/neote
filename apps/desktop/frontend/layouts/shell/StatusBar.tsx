import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';

export function StatusBar() {
  const { currentWorkspace, isLoading } = useWorkspaceStore();
  
  return (
    <div className="h-7 border-t border-divider bg-status-bar flex items-center justify-between px-4 text-xs font-sans">
      <div className="flex items-center space-x-6">
        <div className="flex items-center space-x-2">
          <Icon name="workspace" size={13} className="text-secondary" label="Workspace" />
          <span className="text-secondary">Workspace:</span>
          <span className="font-medium text-primary">
            {currentWorkspace ? currentWorkspace.name : 'No workspace open'}
          </span>
          {currentWorkspace && (
            <span className="text-muted ml-2 font-mono text-[11px]">
              ({currentWorkspace.rootPath})
            </span>
          )}
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-2">
            <div className="w-2.5 h-2.5 rounded-full bg-accent animate-pulse" />
            <span className="text-secondary">Loading...</span>
          </div>
        )}
      </div>
      
      <div className="flex items-center space-x-6 font-mono">
        <div className="flex items-center space-x-2">
          <Icon name="file-code" size={13} className="text-secondary" label="Encoding" />
          <span className="text-primary">UTF-8</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="indent" size={13} className="text-secondary" label="Indentation" />
          <span className="text-primary">Spaces: 2</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="cursor" size={13} className="text-secondary" label="Cursor position" />
          <span className="text-primary">LN 1, COL 1</span>
        </div>
        {/* Font test indicator */}
        <div className="flex items-center space-x-2">
          <Icon name="check" size={13} className="text-success" label="Font loaded" />
          <span className="text-primary text-[11px]">Nerd Font</span>
        </div>
      </div>
    </div>
  );
}
