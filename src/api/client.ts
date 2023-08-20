import { grpc } from '@improbable-eng/grpc-web';
import { GrpcWebImpl, MusyncServiceClientImpl } from '~/generated/protos/musync';
import type { Token } from '~/generated/protos/musync';
import { useAccountStore } from '~/store/user';

export class ApiClient {
  grpcClient: MusyncServiceClientImpl;
  private constructor(private addr: string, token?: Token) {
    const rpc = new GrpcWebImpl(addr, {
      metadata: token !== undefined ? new grpc.Metadata({ authorization: `${token?.data}` }) : undefined,
    });
    this.grpcClient = new MusyncServiceClientImpl(rpc);
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
    return `${this.addr}/assets/track/${id}`;
  }

  cover_uri(id?: number): string {
    return id ? `${this.addr}/assets/track/${id}/cover` : '';
  }
}
