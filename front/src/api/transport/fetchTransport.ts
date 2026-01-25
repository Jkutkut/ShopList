import { Result } from '../../utils/monad';
import type { Transport, TransportError, TransportRequest, TransportResponse } from '../types';

type FetchTransportError = {
  message: string;
  detail: any;
};

class FetchTransport implements Transport<FetchTransportError> {
  async request<R, T>(
    {
      url, method,
      headers = {},
      body
    }: TransportRequest<R>
  ): Promise<Result<
    TransportResponse<T>,
    TransportError<FetchTransportError>
  >> {
    console.debug(`Fetching ${method} ${url}`);
    const response = await fetch(url, {
      method,
      headers: {
        'Content-Type': 'application/json',
        ...headers,
      },
      body: body ? JSON.stringify(body) : undefined,
    });

    // Both ok and fail request return json
    const data = await response.json().catch(() => {});

    if (!response.ok) {
      return Result.error({
        status: response.status,
        detail: {
          message: `HTTP ${response.status}: ${response.statusText}`,
          detail: data
        } as FetchTransportError
      });
    }

    return Result.ok({
      data,
      status: response.status
    });
  }
}

export const fetchTransport = new FetchTransport();
export type { FetchTransportError }
