import { attachConsole, debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

export async function attachLogger() {
  await attachConsole();
}

export const logger = {
  trace,
  info,
  error,
  warn,
  debug,
  getSubLogger: (_options: { name: string }) => { return logger; },
};
