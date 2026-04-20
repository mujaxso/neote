import { useState } from 'react';
import { WorkspaceExplorer } from '../components/WorkspaceExplorer';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { useWorkspaceStore } from '../stores/useWorkspaceStore';

export function WorkspaceContainer() {
  const { fileTree, isLoading, openFolder } = useWorkspaceStore();
  const [activeFilePath, setActiveFilePath] = useState<string | undefined>();

  const handleOpenFile = (path: string) => {
    setActiveFilePath(path);
  };

  const handleOpenFolder = (path: string) => {
    openFolder(path);
  };

  // Convert fileTree to FileItem format expected by WorkspaceExplorer
  const files = fileTree.map(item => ({
    path: item.path,
    name: item.name,
    isDir: item.isDir,
    fileType: item.fileType,
  }));

  return (
    <div className="flex h-full">
      {/* Sidebar with workspace explorer */}
      <div className="w-64 border-r border-border bg-sidebar overflow-y-auto">
        <WorkspaceExplorer
          files={files}
          isLoading={isLoading}
          onOpenFile={handleOpenFile}
          onOpenFolder={handleOpenFolder}
        />
      </div>
      
      {/* Main editor area */}
      <div className="flex-1 overflow-hidden">
        <EditorContainer filePath={activeFilePath} />
      </div>
    </div>
  );
}
