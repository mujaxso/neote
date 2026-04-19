import React, { useState } from 'react'
import { FileTree } from './FileTree'
import { useWorkspaceStore } from '../stores/workspace-store'

export function ExplorerPanel() {
  const [expandedPaths, setExpandedPaths] = useState<Set<string>>(new Set())
  const { fileTree, isLoading, openWorkspace } = useWorkspaceStore()
  
  const handleToggleFolder = (path: string) => {
    const newExpanded = new Set(expandedPaths)
    if (newExpanded.has(path)) {
      newExpanded.delete(path)
    } else {
      newExpanded.add(path)
    }
    setExpandedPaths(newExpanded)
  }
  
  const handleSelectFile = (path: string) => {
    console.log('Selected file:', path)
    // This would dispatch to editor store
  }
  
  return (
    <div className="h-full flex flex-col">
      <div className="p-3 border-b border-gray-800">
        <h2 className="text-sm font-semibold">EXPLORER</h2>
      </div>
      
      <div className="flex-1 overflow-y-auto p-2">
        {isLoading ? (
          <div className="text-gray-400 text-sm p-4">Loading workspace...</div>
        ) : fileTree.length === 0 ? (
          <div className="p-4">
            <button
              onClick={() => openWorkspace('/path/to/workspace')}
              className="text-sm text-blue-400 hover:text-blue-300"
            >
              Open Workspace
            </button>
          </div>
        ) : (
          <FileTree
            nodes={fileTree}
            onSelectFile={handleSelectFile}
            onToggleFolder={handleToggleFolder}
            expandedPaths={expandedPaths}
          />
        )}
      </div>
    </div>
  )
}
