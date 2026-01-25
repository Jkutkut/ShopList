import { apiConfig, type ApiConfig } from "./conf";
import { fetchTransport, type FetchTransportError } from "./transport/fetchTransport";
import { HttpMethod, APIVersion, type Transport } from "./types";

class HttpClient<E> {
  protected readonly config: ApiConfig;
  protected readonly transport: Transport<E>;
  protected readonly headers: Record<string, string>;
  protected readonly version: APIVersion;

  public constructor(
    transport: Transport<E>,
    config: ApiConfig,
    headers: Record<string, string>,
    version: APIVersion,
  ){
    this.config = config;
    this.transport = transport;
    this.headers = headers;
    this.version = version
  }

  public static create({
    transport = fetchTransport,
    config = apiConfig,
    headers = {},
    version = APIVersion.V1,
  }: {
    transport?: Transport<FetchTransportError>;
    config?: ApiConfig;
    headers?: Record<string, string>;
    version?: APIVersion;
  }) {
    return new HttpClient(transport, config, headers, version);
  }

  protected toUrl(endpoint: string) {
    return `/${this.config.basePath}/${this.version}${endpoint}`
  }

  protected formatHeaders(headers?: Record<string, string>) {
    return {
      ...this.headers,
      ...(headers || {}),
    };
  }

  get<T>(endpoint: string, headers?: Record<string, string>) {
    return this.transport.request<null, T>({
      url: this.toUrl(endpoint),
      method: HttpMethod.GET,
      headers: this.formatHeaders(headers),
    });
  }

  post<R, T>(endpoint: string, body?: R, headers?: Record<string, string>) {
    return this.transport.request<R, T>({
      url: this.toUrl(endpoint),
      method: HttpMethod.POST,
      headers: this.formatHeaders(headers),
      body,
    });
  }

  put<R, T>(endpoint: string, body?: R, headers?: Record<string, string>) {
    return this.transport.request<R, T>({
      url: this.toUrl(endpoint),
      method: HttpMethod.PUT,
      headers: this.formatHeaders(headers),
      body,
    });
  }

  delete<T>(endpoint: string, headers?: Record<string, string>) {
    return this.transport.request<null, T>({
      url: this.toUrl(endpoint),
      method: HttpMethod.DELETE,
      headers: this.formatHeaders(headers),
    });
  }

  patch<R, T>(endpoint: string, body?: R, headers?: Record<string, string>) {
    return this.transport.request<R, T>({
      url: this.toUrl(endpoint),
      method: HttpMethod.PATCH,
      headers: this.formatHeaders(headers),
      body,
    });
  }
};

export { HttpClient };
