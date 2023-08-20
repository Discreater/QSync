import { grpc } from '@improbable-eng/grpc-web';
import { GrpcWebImpl, MusyncServiceClientImpl } from '~/generated/protos/musync';
import type { Token } from '~/generated/protos/musync';
import { useAccountStore } from '~/store/user';
import { logger } from '~/utils/logger';

class WsClient {
  constructor(private wsClient: WebSocket) {
    this.wsClient.onopen = () => {
    };
    this.wsClient.onmessage = (e) => {
      (e.data as Blob).text().then((data) => {
        logger.trace(`ws got message: ${data}`);
        this.wsClient.send(JSON.stringify(
          'Pong',
        ));
      });
    };
  }
}

export class ApiClient {
  grpcClient: MusyncServiceClientImpl;
  wsClient: WsClient;
  private constructor(private addr: string, token?: Token) {
    logger.trace(addr);
    const rpc = new GrpcWebImpl(this.http_addr, {
      metadata: token !== undefined ? new grpc.Metadata({ authorization: `${token?.data}` }) : undefined,
    });
    this.grpcClient = new MusyncServiceClientImpl(rpc);
    this.wsClient = new WsClient(new WebSocket(this.ws_addr));
  }

  public get http_addr(): string {
    return `http://${this.addr}`;
  }

  public get ws_addr(): string {
    return `ws://${this.addr}/ws`;
  }

  setToken(token: Token) {
    const rpc = new GrpcWebImpl(this.addr, {
      metadata: new grpc.Metadata({ authorization: `${token.data}` }),
    });
    this.grpcClient = new MusyncServiceClientImpl(rpc);
  }

  static apiClient: ApiClient | undefined;
  static get(): ApiClient {
    if (ApiClient.apiClient === undefined)
      throw new Error('ApiClient not initialized');

    return ApiClient.apiClient;
  }

  static grpc(): MusyncServiceClientImpl {
    return ApiClient.get().grpcClient;
  }

  static set(addr: string) {
    const accountStore = useAccountStore();
    ApiClient.apiClient = new ApiClient(addr, accountStore.token);
  }

  track_uri(id: number): string {
    return `${this.http_addr}/assets/track/${id}`;
  }

  cover_uri(id?: number): string {
    return id ? `${this.http_addr}/assets/track/${id}/cover` : '';
  }
}
