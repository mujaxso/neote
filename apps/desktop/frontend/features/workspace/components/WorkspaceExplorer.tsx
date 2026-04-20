import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useWorkspaceStore } from '../stores/useWorkspaceStore';

interface FileItem {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
}

interface WorkspaceExplorerProps {
  files: FileItem[];
  isLoading: boolean;
  onOpenFile: (path: string) => void;
  onOpenFolder: (path: string) => void;
}

export function WorkspaceExplorer({ 
  files, 
  isLoading, 
  onOpenFile, 
  onOpenFolder 
}: WorkspaceExplorerProps) {
  const { openWorkspaceViaDialog, currentWorkspace, currentDirectory, navigateToParent } = useWorkspaceStore();

  const handleOpenWorkspace = async () => {
    await openWorkspaceViaDialog();
  };

  if (!currentWorkspace) {
    return (
      <div className="p-4">
        <div className="text-center">
          <p className="text-muted-foreground mb-4">No workspace open</p>
          <button
            onClick={handleOpenWorkspace}
            className="px-4 py-2 bg-primary text-primary-foreground rounded hover:bg-primary/90 transition-colors"
            disabled={isLoading}
          >
            {isLoading ? 'Opening...' : 'Open Workspace'}
          </button>
          <p className="text-xs text-muted-foreground mt-4">
            Select a directory to open as a workspace
          </p>
        </div>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="p-4">
        <div className="flex items-center justify-between mb-4">
          <div className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            Explorer
          </div>
          <button
            onClick={handleOpenWorkspace}
            className="text-xs px-2 py-1 bg-muted rounded hover:bg-muted/80"
            disabled={true}
          >
            Change
          </button>
        </div>
        <div className="animate-pulse space-y-2">
          <div className="h-4 bg-muted rounded w-3/4"></div>
          <div className="h-4 bg-muted rounded w-1/2"></div>
        </div>
      </div>
    );
  }

  if (files.length === 0) {
    return (
      <div className="p-4">
        <div className="flex items-center justify-between mb-4">
          <div className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
            Explorer
          </div>
          <button
            onClick={handleOpenWorkspace}
            className="text-xs px-2 py-1 bg-muted rounded hover:bg-muted/80"
          >
            Change
          </button>
        </div>
        <div className="text-center text-muted-foreground py-8">
          <p>No files to display</p>
          <p className="text-sm mt-1">Directory: {currentDirectory || currentWorkspace.rootPath}</p>
        </div>
      </div>
    );
  }

  const isAtRoot = currentDirectory === currentWorkspace.rootPath;

  return (
    <div className="py-2">
      <div className="flex items-center justify-between px-3 py-2">
        <div className="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
          Explorer
        </div>
        <div className="flex items-center space-x-2">
          <span className="text-xs text-muted-foreground truncate max-w-[100px]">
            {currentWorkspace.name}
          </span>
          <button
            onClick={handleOpenWorkspace}
            className="text-xs px-2 py-1 bg-muted rounded hover:bg-muted/80"
          >
            Change
          </button>
        </div>
      </div>
      
      {/* Current directory path and navigation */}
      <div className="px-3 py-2 border-b border-border">
        <div className="flex items-center justify-between">
          <div className="text-xs text-muted-foreground truncate">
            {currentDirectory || currentWorkspace.rootPath}
          </div>
          {!isAtRoot && (
            <button
              onClick={navigateToParent}
              className="text-xs px-2 py-1 bg-muted rounded hover:bg-muted/80"
              title="Go up"
            >
              <Icon name="arrow-up" size={12} />
            </button>
          )}
        </div>
      </div>
      
      <div className="space-y-1 mt-2">
        {files.map((file) => (
          <button
            key={file.path}
            onClick={() => file.isDir ? onOpenFolder(file.path) : onOpenFile(file.path)}
            className={cn(
              'w-full flex items-center px-3 py-1.5 text-sm hover:bg-muted transition-colors',
              'focus:outline-none focus:bg-muted'
            )}
          >
            <Icon 
              name={file.isDir ? 'folder' : 'file'} 
              size={16}
              className="mr-2 text-muted-foreground"
            />
            <span className="truncate">{file.name}</span>
            {file.fileType && (
              <span className="ml-auto text-xs text-muted-foreground">
                {file.fileType}
              </span>
            )}
          </button>
        ))}
      </div>
    </div>
  );
}
