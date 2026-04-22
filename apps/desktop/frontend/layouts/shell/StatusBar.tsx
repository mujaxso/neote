import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { Icon } from '@/components/ui/Icon';
import { cn } from '@/lib/utils';
import { useEffect, useState } from 'react';

interface StatusBarProps {
  className?: string;
}

export function StatusBar({ className }: StatusBarProps) {
  const { currentWorkspace, isLoading } = useWorkspaceStore();
  const [currentTime, setCurrentTime] = useState(new Date());
  const [branchName, setBranchName] = useState<string | null>(null);
  const [fileInfo, setFileInfo] = useState<string | null>(null);

  // Update time every minute
  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentTime(new Date());
    }, 60000); // Update every minute

    return () => clearInterval(timer);
  }, []);

  // Format time as HH:MM
  const formattedTime = currentTime.toLocaleTimeString([], { 
    hour: '2-digit', 
    minute: '2-digit',
    hour12: false 
  });

  return (
    <div 
      className={cn(
        "h-7 flex items-center justify-between px-3 text-xs font-sans",
        "text-primary font-medium",
        className
      )}
      style={{
        backgroundColor: 'var(--status-bar-background)',
      }}
    >
      {/* Left section: Workspace info */}
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-2">
          <Icon name="workspace" size={12} className="text-primary" label="Workspace" />
          <span className="text-primary font-semibold">
            {currentWorkspace ? currentWorkspace.name : 'No workspace'}
          </span>
          {currentWorkspace && (
            <span className="text-primary/80 ml-1 font-mono text-[10px] hidden md:inline">
              ({currentWorkspace.rootPath.split('/').pop()})
            </span>
          )}
        </div>
        
        {isLoading && (
          <div className="flex items-center space-x-2">
            <div className="w-2 h-2 rounded-full bg-accent animate-pulse" />
            <span className="text-primary/80">Loading...</span>
          </div>
        )}

        {/* Git branch info */}
        {branchName && (
          <div className="flex items-center space-x-2">
            <Icon name="git-branch" size={12} className="text-primary" label="Git branch" />
            <span className="text-primary font-semibold">{branchName}</span>
          </div>
        )}
      </div>
      
      {/* Center section: File info */}
      <div className="flex items-center space-x-4">
        {fileInfo && (
          <div className="flex items-center space-x-2">
            <Icon name="file" size={12} className="text-primary" label="File info" />
            <span className="text-primary font-semibold">{fileInfo}</span>
          </div>
        )}
      </div>
      
      {/* Right section: Editor status and time */}
      <div className="flex items-center space-x-4 font-mono">
        <div className="flex items-center space-x-2">
          <Icon name="file-code" size={12} className="text-primary" label="Encoding" />
          <span className="text-primary font-semibold">UTF-8</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="indent" size={12} className="text-primary" label="Indentation" />
          <span className="text-primary font-semibold">Spaces: 2</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="cursor" size={12} className="text-primary" label="Cursor position" />
          <span className="text-primary font-semibold">Ln 1, Col 1</span>
        </div>
        <div className="flex items-center space-x-2">
          <Icon name="clock" size={12} className="text-primary" label="Current time" />
          <span className="text-primary font-semibold">{formattedTime}</span>
        </div>
      </div>
    </div>
  );
}
