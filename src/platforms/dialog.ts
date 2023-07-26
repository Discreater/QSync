import type { OpenDialogOptions as TauriOpenDialogOptions } from '@tauri-apps/api/dialog';
import { open as tauriOpen } from '@tauri-apps/api/dialog';

export function open(options: TauriOpenDialogOptions) {
  return tauriOpen(options);
}