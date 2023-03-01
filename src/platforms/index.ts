export function getPlatform() {
  if (window.__TAURI_METADATA__)
    return 'tauri';
  else
    return 'web';
}

export function inTauri<T>(m: () => T): T | undefined {
  if (window.__TAURI_METADATA__)
    return m();
}
