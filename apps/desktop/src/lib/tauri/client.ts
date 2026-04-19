import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

export class TauriClient {
  async invoke<T>(cmd: string, payload?: any): Promise<T> {
    return invoke(cmd, payload)
  }

  on<E>(eventName: string, handler: (payload: E) => void) {
    return listen<E>(eventName, ({ payload }) => handler(payload))
  }

  // Domain-specific APIs
  workspace = {
    open: (path: string) => this.invoke<any>('open_workspace', { path }),
    getTree: () => this.invoke<any[]>('get_file_tree'),
  }

  editor = {
    openFile: (path: string) => this.invoke<any>('open_file', { path }),
    save: (tabId: string, content: string) => 
      this.invoke<boolean>('save_file', { tabId, content }),
  }
}

export const tauriClient = new TauriClient()
