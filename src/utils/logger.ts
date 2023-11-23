import { attachConsole, debug, error, info, trace, warn } from '@tauri-apps/plugin-log';
import { getPlatform, inTauri } from '~/platforms';

export async function attachLogger() {
  await inTauri(async () => {
    return await attachConsole();
  });
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
  if (getPlatform() === 'tauri') {
    return {
      trace: message => trace(message, { file: name }),
      info: message => info(message, { file: name }),
      error: message => error(message, { file: name }),
      warn: message => warn(message, { file: name }),
      debug: message => debug(message, { file: name }),
      getSubLogger: subName => createLogger(`${name}/${subName}`),
    };
  } else {
    return {
      // eslint-disable-next-line no-console
      trace: message => console.trace(`[${name}] ${message}`),
      // eslint-disable-next-line no-console
      info: message => console.info(`[${name}] ${message}`),
      error: message => console.error(`[${name}] ${message}`),
      warn: message => console.warn(`[${name}] ${message}`),
      // eslint-disable-next-line no-console
      debug: message => console.debug(`[${name}] ${message}`),
      getSubLogger: subName => createLogger(`${name}/${subName}`),
    };
  }
}

export const logger = createLogger('QSync');
