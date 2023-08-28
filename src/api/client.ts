import { grpc } from '@improbable-eng/grpc-web';
import { GrpcWebImpl, MusyncServiceClientImpl } from '~/generated/protos/musync';
import type { Token, UpdatePlayQueueEvent, UpdatePlayerEvent, UpdatePlayerRequest } from '~/generated/protos/musync';
import { useAccountStore } from '~/store/user';
import { logger } from '~/utils/logger';

type ServerMsg = 'AuthSuccess' | {
  UpdatePlayer: UpdatePlayerEvent
} | {
  UpdatePlayQueue: UpdatePlayQueueEvent
};

type ClientMsg = 'Pong' | {
  UpdatePlayer: UpdatePlayerRequest
};

export class WsClient {
  // We will only have one socket per client, so we can use static variables.
  static updatePlayerListeners: Map<number, (update: UpdatePlayerEvent) => void> = new Map();
  static updatePlayQueueListeners: Map<number, (update: UpdatePlayQueueEvent) => void> = new Map();

  authed: boolean = false;
  msgQueue: ClientMsg[] = [];
  constructor(private wsClient: WebSocket, token: Token) {
    this.wsClient.onopen = () => {
      this.wsClient.send(JSON.stringify({
        Auth: token?.data,
      }));
    };
    this.wsClient.onmessage = (e) => {
      (e.data as Blob).text().then((data) => {
        const msg = JSON.parse(data) as ServerMsg;
        logger.trace(`ws got message: ${data}`);
        if (msg === 'AuthSuccess') {
          this.authed = true;
          this.msgQueue.forEach((msg) => {
            logger.trace(`ws send message in message queue: ${JSON.stringify(msg)}`);
            this.wsClient.send(JSON.stringify(msg));
          });
        } else if ('UpdatePlayer' in msg) {
          WsClient.updatePlayerListeners.forEach((fn) => {
            fn(msg.UpdatePlayer);
          });
        } else if ('UpdatePlayQueue' in msg) {
          WsClient.updatePlayQueueListeners.forEach((fn) => {
            fn(msg.UpdatePlayQueue);
          });
        }
      });
    };
  }

  sendMsg(msg: ClientMsg) {
    if (!this.authed) {
      logger.trace(`store in message queue: ${JSON.stringify(msg)}`);
      this.msgQueue.push(msg);
      return;
    }
    logger.trace(`ws send message: ${JSON.stringify(msg)}`);
    this.wsClient.send(JSON.stringify(msg));
  }

  static listenOnUpdatePlayer(fn: (update: UpdatePlayerEvent) => void): number {
    const id = WsClient.getEventId();
    WsClient.updatePlayerListeners.set(id, fn);
    return id;
  }

  static listenOnUpdatePlayQueue(fn: (update: UpdatePlayQueueEvent) => void): number {
    const id = WsClient.getEventId();
    WsClient.updatePlayQueueListeners.set(id, fn);
    return id;
  }

  static cancelOnUpdatePlayer(id: number): void {
    WsClient.updatePlayerListeners.delete(id);
  }

  static cancelOnUpdatePlayQueue(id: number): void {
    WsClient.updatePlayQueueListeners.delete(id);
  }

  static getEventId() {
    return WsClient.eventId++;
  }

  private static eventId = 0;
}

export class ApiClient {
  grpcClient: MusyncServiceClientImpl;
  wsClient: WsClient | undefined;
  private constructor(private addr: string, token?: Token) {
    logger.trace(addr);
    const rpc = new GrpcWebImpl(this.http_addr, {
      metadata: token !== undefined ? new grpc.Metadata({ authorization: `${token?.data}` }) : undefined,
    });
    this.grpcClient = new MusyncServiceClientImpl(rpc);
    if (token !== undefined)
      this.wsClient = new WsClient(new WebSocket(this.ws_addr), token);
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

  static ws(): WsClient | undefined {
    return ApiClient.get().wsClient;
  }

  static set(addr: string) {
    const accountStore = useAccountStore();
    ApiClient.apiClient = new ApiClient(addr, accountStore.token);
  }

  static reset() {
    const addr = ApiClient.get().addr;
    ApiClient.apiClient = new ApiClient(addr);
  }

  track_uri(id: number): string {
    return `${this.http_addr}/assets/track/${id}`;
  }

  cover_uri(id?: number): string {
    return id ? `${this.http_addr}/assets/track/${id}/cover` : '';
  }
}
