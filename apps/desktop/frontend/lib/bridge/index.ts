/**
 * Tauri bridge layer - provides typed wrappers for Tauri operations
 * 
 * This layer handles:
 * - Typed invoke calls
 * - Event listening/unsubscribing
 * - Error normalization
 * - Connection lifecycle
 */

import { invoke, type InvokeArgs } from '@tauri-apps/api/core';
import { listen, type UnlistenFn, type Event } from '@tauri-apps/api/event';

export type BridgeResult<T> = 
  | { success: true; data: T }
  | { success: false; error: string };

export class BridgeError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
    public readonly details?: unknown
  ) {
    super(message);
    this.name = 'BridgeError';
  }
}

/**
 * Typed invoke wrapper with normalized error handling
 */
export async function bridgeInvoke<T>(
  command: string,
  args?: InvokeArgs
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    const message =
      typeof error === 'string'
        ? error
        : error instanceof Error
          ? error.message
          : 'Unknown error';
    throw new BridgeError(message, undefined, error);
  }
}

/**
 * Typed event listener wrapper
 */
export function bridgeListen<T>(
  event: string,
  handler: (payload: T) => void
): Promise<UnlistenFn> {
  return listen<T>(event, (event: Event<T>) => {
    handler(event.payload);
  });
}

/**
 * Bridge commands namespace for type safety
 */
export const bridge = {
  invoke: bridgeInvoke,
  listen: bridgeListen,
};
