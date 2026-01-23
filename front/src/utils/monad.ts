class Result<T, E> {
  private readonly data?: T;
  private readonly error?: E;

  private constructor(data?: T, error?: E) {
    this.data = data;
    this.error = error;
  }

  public static of<T, E>(data?: T, error?: E): Result<T, E> {
    return new Result(data, error);
  }

  public static error<T, E>(error: E): Result<T, E> {
    return new Result<T, E>(undefined, error);
  }

  public static ok<T, E>(data: T): Result<T, E> {
    return new Result(data);
  }

  public isOk(): boolean {
    return this.data !== undefined;
  }

  public isErr(): boolean {
    return this.error !== undefined;
  }

  public unwrap(): T {
    if (this.data === undefined) {
      throw new Error(`Unwrap an error result: ${this.error}`);
    }
    return this.data;
  }

  public unwrapErr(): E {
    if (this.error === undefined) {
      throw new Error(`Unwrap an ok result: ${this.data}`);
    }
    return this.error;
  }
};

class Option<T> {
  private readonly data?: T;

  private constructor(data?: T) {
    this.data = data;
  }

  public static of<T>(data?: T): Option<T> {
    return new Option(data);
  }

  public static none<T>(): Option<T> {
    return new Option<T>(undefined);
  }

  public isSome(): boolean {
    return this.data !== undefined;
  }

  public isNone(): boolean {
    return this.data === undefined;
  }

  public unwrap(): T {
    if (this.data === undefined) {
      throw new Error(`Unwrap an none option`);
    }
    return this.data;
  }
};

export { Result, Option };
