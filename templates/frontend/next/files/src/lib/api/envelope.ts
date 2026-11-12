export type ApiErrorBody = {
  error: {
    code: string;
    message: string;
    fields?: Record<string, string[]>;
  };
};

export type ApiSuccess<T> = {
  data: T;
};

export type ApiFailure = ApiErrorBody;

export function success<T>(data: T): ApiSuccess<T> {
  return { data };
}

export function failure(
  code: string,
  message: string,
  fields?: Record<string, string[]>,
): ApiFailure {
  return {
    error: {
      code,
      message,
      ...(fields ? { fields } : {}),
    },
  };
}
