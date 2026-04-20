import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { WorkspaceService, type OpenWorkspaceResponse, type DirectoryEntryDto } from '../services/workspaceService';

interface Workspace {
  id: string;
  name: string;
  rootPath: string;
}

interface WorkspaceStore {
  // State
  currentWorkspace: Workspace | null;
  fileTree: DirectoryEntryDto[];
  isLoading: boolean;
  error: string | null;
  
  // Actions
  openWorkspace: (path: string) => Promise<void>;
  refreshFileTree: () => Promise<void>;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setCurrentWorkspace: (workspace: Workspace | null) => void;
  setFileTree: (tree: DirectoryEntryDto[]) => void;
}

export const useWorkspaceStore = create<WorkspaceStore>()(
  devtools(
    persist(
      (set, get) => ({
        currentWorkspace: null,
        fileTree: [],
        isLoading: false,
        error: null,
        
        openWorkspace: async (path: string) => {
          set({ isLoading: true, error: null });
          try {
            const response = await WorkspaceService.openWorkspace({ path });
            
            const workspace: Workspace = {
              id: response.workspaceId,
              name: path.split(/[\\/]/).pop() || 'Workspace',
              rootPath: response.rootPath,
            };
            
            set({ 
              currentWorkspace: workspace,
              isLoading: false 
            });
            
            // Refresh file tree after opening
            await get().refreshFileTree();
          } catch (error) {
            set({ 
              error: error instanceof Error ? error.message : 'Unknown error',
              isLoading: false 
            });
          }
        },
        
        refreshFileTree: async () => {
          const { currentWorkspace } = get();
          if (!currentWorkspace) return;
          
          try {
            const entries = await WorkspaceService.listDirectory({ 
              path: currentWorkspace.rootPath 
            });
            
            set({ fileTree: entries });
          } catch (error) {
            console.error('Failed to refresh file tree:', error);
          }
        },
        
        setLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
        setCurrentWorkspace: (workspace) => set({ currentWorkspace: workspace }),
        setFileTree: (tree) => set({ fileTree: tree }),
      }),
      {
        name: 'workspace-storage',
        partialize: (state) => ({
          currentWorkspace: state.currentWorkspace,
        }),
      }
    )
  )
);
