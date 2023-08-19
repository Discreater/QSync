import { GrpcWebImpl, MusyncServiceClientImpl } from '~/generated/protos/musync';

export class ApiClient {
  grpcClient: MusyncServiceClientImpl;
  constructor(private addr: string) {
    const rpc = new GrpcWebImpl(addr, {});
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
    ApiClient.apiClient = new ApiClient(addr);
  }

  track_uri(id: number): string {
    return `${this.addr}/assets/track/${id}`;
  }

  cover_uri(id?: number): string {
    return id ? `${this.addr}/assets/track/${id}/cover` : '';
  }
}
