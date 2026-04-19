import React from 'react'
import { ActivityRail } from '../../features/workspace/components/ActivityRail'
import { ExplorerPanel } from '../../features/workspace/components/ExplorerPanel'
import { EditorTabs } from '../../features/editor/components/EditorTabs'
import { AssistantPanel } from '../../features/assistant/components/AssistantPanel'
import { StatusBar } from '../../components/status-bar/StatusBar'
import { CommandPalette } from '../../components/command-palette/CommandPalette'

export function AppShell() {
  return (
    <div className="h-screen flex flex-col bg-gray-950 text-gray-100">
      {/* Top bar */}
      <div className="h-8 border-b border-gray-800 flex items-center px-4">
        <div className="text-sm font-medium">Zaroxi Studio</div>
      </div>
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity rail */}
        <ActivityRail />
        
        {/* Explorer panel */}
        <div className="w-64 border-r border-gray-800">
          <ExplorerPanel />
        </div>
        
        {/* Main editor area */}
        <div className="flex-1 flex flex-col">
          <EditorTabs />
          <div className="flex-1 bg-gray-900 p-4">
            <div className="text-gray-400">
              Editor content will appear here
            </div>
          </div>
        </div>
        
        {/* Assistant panel */}
        <div className="w-80 border-l border-gray-800">
          <AssistantPanel />
        </div>
      </div>
      
      {/* Status bar */}
      <StatusBar />
      
      {/* Command palette (hidden by default) */}
      <CommandPalette />
    </div>
  )
}
