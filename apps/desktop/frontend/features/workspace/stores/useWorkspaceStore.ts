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
  currentDirectory: string | null;
  fileTree: DirectoryEntryDto[];
  isLoading: boolean;
  error: string | null;
  
  // Actions
  openWorkspace: (path: string) => Promise<void>;
  openWorkspaceViaDialog: () => Promise<void>;
  refreshFileTree: (path?: string) => Promise<void>;
  openFolder: (path: string) => Promise<void>;
  navigateToParent: () => Promise<void>;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  setCurrentWorkspace: (workspace: Workspace | null) => void;
  setCurrentDirectory: (path: string | null) => void;
  setFileTree: (tree: DirectoryEntryDto[]) => void;
}

export const useWorkspaceStore = create<WorkspaceStore>()(
  devtools(
    persist(
      (set, get) => ({
        currentWorkspace: null,
        currentDirectory: null,
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
              currentDirectory: response.rootPath,
              isLoading: false 
            });
            
            // Refresh file tree after opening
            await get().refreshFileTree(response.rootPath);
          } catch (error) {
            set({ 
              error: error instanceof Error ? error.message : 'Unknown error',
              isLoading: false 
            });
          }
        },
        
        // Add a method to open workspace via file dialog
        openWorkspaceViaDialog: async () => {
          set({ isLoading: true, error: null });
          try {
            const dialogResult = await WorkspaceService.openFileDialog();
            
            if (!dialogResult.selectedPath) {
              set({ isLoading: false });
              return; // User cancelled
            }
            
            const path = dialogResult.selectedPath;
            const response = await WorkspaceService.openWorkspace({ path });
            
            const workspace: Workspace = {
              id: response.workspaceId,
              name: path.split(/[\\/]/).pop() || 'Workspace',
              rootPath: response.rootPath,
            };
            
            set({ 
              currentWorkspace: workspace,
              currentDirectory: response.rootPath,
              isLoading: false 
            });
            
            // Refresh file tree after opening
            await get().refreshFileTree(response.rootPath);
          } catch (error) {
            set({ 
              error: error instanceof Error ? error.message : 'Unknown error',
              isLoading: false 
            });
          }
        },
        
        refreshFileTree: async (path?: string) => {
          const { currentWorkspace } = get();
          if (!currentWorkspace) return;
          
          const targetPath = path || currentWorkspace.rootPath;
          
          try {
            const entries = await WorkspaceService.listDirectory({ 
              path: targetPath 
            });
            
            set({ 
              fileTree: entries,
              currentDirectory: targetPath 
            });
          } catch (error) {
            console.error('Failed to refresh file tree:', error);
          }
        },
        
        setLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
        setCurrentWorkspace: (workspace) => set({ currentWorkspace: workspace }),
        openFolder: async (path: string) => {
          try {
            await get().refreshFileTree(path);
          } catch (error) {
            console.error('Failed to open folder:', error);
          }
        },
        
        navigateToParent: async () => {
          const { currentDirectory } = get();
          if (!currentDirectory) return;
          
          // Get parent directory
          const parentPath = currentDirectory.split(/[\\/]/).slice(0, -1).join('/');
          if (parentPath) {
            await get().refreshFileTree(parentPath);
          }
        },
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
