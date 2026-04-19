import { tauriClient } from '../../../lib/tauri/client'

export class WorkspaceService {
  async open(path: string): Promise<any> {
    return tauriClient.workspace.open(path)
  }

  async getTree(): Promise<any[]> {
    return tauriClient.workspace.getTree()
  }
}

export const workspaceService = new WorkspaceService()
