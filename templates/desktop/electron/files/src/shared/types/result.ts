export type FieldErrors = Record<string, string[]>;

export type AppError = {
  code: string;
  message: string;
  fields?: FieldErrors;
};

export type Result<T> =
  | { ok: true; data: T }
  | { ok: false; error: AppError };

export function ok<T>(data: T): Result<T> {
  return { ok: true, data };
}

export function fail<T = never>(
  code: string,
  message: string,
  fields?: FieldErrors,
): Result<T> {
  return { ok: false, error: { code, message, fields } };
}
