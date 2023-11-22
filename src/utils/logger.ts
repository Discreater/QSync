import { attachConsole, debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

export async function attachLogger() {
  await attachConsole();
}

export interface Logger {
  trace: (message: string) => void
  info: (message: string) => void
  error: (message: string) => void
  warn: (message: string) => void
  debug: (message: string) => void
  getSubLogger: (subName: string) => Logger
}

export function createLogger(name: string): Logger {
  return {
    trace: message => trace(message, { file: name }),
    info: message => info(message, { file: name }),
    error: message => error(message, { file: name }),
    warn: message => warn(message, { file: name }),
    debug: message => debug(message, { file: name }),
    getSubLogger: subName => createLogger(`${name}/${subName}`),
  };
}

export const logger = createLogger('QSync');
