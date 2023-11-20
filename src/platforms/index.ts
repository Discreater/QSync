export function getPlatform() {
  // @ts-expect-error tauri should define this
  if (window.__TAURI_INTERNALS__)
    return 'tauri';
  else
    return 'web';
}

export function inTauri<T>(m: () => T): T | undefined {
  // @ts-expect-error tauri should define this
  if (window.__TAURI_INTERNALS__)
    return m();
}
