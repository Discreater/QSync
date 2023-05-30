import type { Api, RecommendedServerInfo } from '@jellyfin/sdk';
import { Jellyfin } from '@jellyfin/sdk';
import { getSystemApi } from '@jellyfin/sdk/lib/utils/api/system-api';
import { getUserApi } from '@jellyfin/sdk/lib/utils/api/user-api';
import { getLibraryApi } from '@jellyfin/sdk/lib/utils/api/library-api';
import { logger } from './util';

export interface JellyfinClientOptions {
  server: string
  user: string
  pwd: string
}

export class JellyfinClient {
  jellyfin: Jellyfin;
  api?: Api;
  bestServer?: RecommendedServerInfo;
  constructor(private opt: JellyfinClientOptions, deviceName: string, deviceId: string) {
    this.jellyfin = new Jellyfin({
      clientInfo: {
        name: 'QSync',
        version: '0.0.1',
      },
      deviceInfo: {
        name: deviceName,
        id: deviceId,
      },
    });
  }

  async testConnect() {
    const server = await this.jellyfin.discovery.getRecommendedServerCandidates(this.opt.server);
    const best = this.jellyfin.discovery.findBestServer(server);
    if (!best) {
      logger.error('Cannot connect to server');
      return undefined;
    }
    return best;
  }

  async connect() {
    if (this.api)
      return;
    if (!this.bestServer) {
      const best = await this.testConnect();
      if (!best)
        throw new Error('Cannot connect to server');

      this.bestServer = best;
    }

    const api = this.jellyfin.createApi(this.bestServer.address);

    // Fetch the public system info
    const info = await getSystemApi(api).getPublicSystemInfo();
    logger.info('Info => ', info.data);

    // Fetch the list of public users
    const users = await getUserApi(api).getPublicUsers();
    logger.info('Users => ', users.data);

    // A helper method for authentication has been added to the SDK because
    // the default method exposed in the generated Axios client is rather
    // cumbersome to use.
    const auth = await api.authenticateUserByName(this.opt.user, this.opt.pwd);
    logger.info('Auth => ', auth.data);

    // Authentication state is stored internally in the Api class, so now
    // requests that require authentication can be made normally
    const libraries = await getLibraryApi(api).getMediaFolders();
    logger.info('Lib => ', libraries.data);

    // A helper method for logging out the current user has been added to the
    // SDK so the internal state is updated correctly.
    await api.logout();
  }
}
