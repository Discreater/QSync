import { invoke as tauriInvoke } from '@tauri-apps/api/primitives';
import { open as tauriOpen } from '@tauri-apps/plugin-dialog';

export const open = tauriOpen;
export function getServer(): Promise<string> {
  return tauriInvoke('get_server');
}
