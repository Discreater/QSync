import { invoke as tauriInvoke } from '@tauri-apps/api/primitives';
import type { OpenDialogOptions as TauriOpenDialogOptions } from '@tauri-apps/plugin-dialog';
import { open as tauriOpen } from '@tauri-apps/plugin-dialog';

type OpenDialogOptions = TauriOpenDialogOptions & {
  multiple?: false
  directory?: false
};

export function open(options: OpenDialogOptions) {
  return tauriOpen(options);
}

export function getServer(): Promise<string> {
  return tauriInvoke('get_server');
}
