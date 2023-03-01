import { logger as mainLogger } from '~/utils/logger';

export const logger = mainLogger.getSubLogger({
  name: 'jellyfin',
});
