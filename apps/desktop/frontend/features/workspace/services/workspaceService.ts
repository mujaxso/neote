import { invoke } from '@tauri-apps/api/core';

export interface OpenWorkspaceRequest {
  path: string;
}

export interface OpenWorkspaceResponse {
  workspaceId: string;
  rootPath: string;
  fileCount: number;
}

export interface ListDirectoryRequest {
  path: string;
}

export interface DirectoryEntryDto {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
}

export interface OpenFileRequest {
  path: string;
}

export interface OpenFileResponse {
  content: string;
  language?: string;
}

export interface SaveFileRequest {
  path: string;
  content: string;
}

export class WorkspaceService {
  static async openWorkspace(request: OpenWorkspaceRequest): Promise<OpenWorkspaceResponse> {
    return await invoke<OpenWorkspaceResponse>('open_workspace', { request });
  }

  static async listDirectory(request: ListDirectoryRequest): Promise<DirectoryEntryDto[]> {
    return await invoke<DirectoryEntryDto[]>('list_directory', { request });
  }

  static async openFile(request: OpenFileRequest): Promise<OpenFileResponse> {
    return await invoke<OpenFileResponse>('open_file', { request });
  }

  static async saveFile(request: SaveFileRequest): Promise<void> {
    return await invoke<void>('save_file', { request });
  }
}
