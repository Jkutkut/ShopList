import type { Result } from "../utils/monad";

enum HttpMethod {
  GET = 'GET',
  POST = 'POST',
  PUT = 'PUT',
  PATCH = 'PATCH',
  DELETE = 'DELETE',
};

interface TransportRequest<R> {
  url: string;
  method: HttpMethod;
  headers: Record<string, string>;
  body?: R | undefined;
}

type TransportResponse<T> = {
  data: T;
  status: number;
}

type TransportError<E> = {
  detail: E;
  status: number;
}

interface Transport<E> {
  request<R, T>(req: TransportRequest<R>): Promise<Result<TransportResponse<T>, TransportError<E>>>;
}

type ApiConfig = {
  basePath: string;
};

enum APIVersion {
  V1 = "v1",
}

export { HttpMethod, APIVersion };
export type {
  ApiConfig,
  Transport,
  TransportRequest,
  TransportResponse,
  TransportError,
};
