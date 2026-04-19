import { create } from 'zustand'
import { workspaceService } from '../services/workspace-service'

interface WorkspaceStore {
  currentWorkspace: any | null
  fileTree: any[]
  isLoading: boolean
  error: string | null
  
  openWorkspace: (path: string) => Promise<void>
  refreshTree: () => Promise<void>
}

export const useWorkspaceStore = create<WorkspaceStore>((set, get) => ({
  currentWorkspace: null,
  fileTree: [],
  isLoading: false,
  error: null,
  
  openWorkspace: async (path: string) => {
    set({ isLoading: true, error: null })
    try {
      const workspace = await workspaceService.open(path)
      const tree = await workspaceService.getTree()
      set({ 
        currentWorkspace: workspace, 
        fileTree: tree, 
        isLoading: false 
      })
    } catch (error: any) {
      set({ error: error.message, isLoading: false })
    }
  },
  
  refreshTree: async () => {
    const tree = await workspaceService.getTree()
    set({ fileTree: tree })
  },
}))
