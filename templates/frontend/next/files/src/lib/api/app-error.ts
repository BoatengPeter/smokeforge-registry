export type ErrorFields = Record<string, string[]>;

export class AppError extends Error {
  readonly status: number;
  readonly code: string;
  readonly fields?: ErrorFields;

  constructor(
    status: number,
    code: string,
    message: string,
    fields?: ErrorFields,
  ) {
    super(message);
    this.name = "AppError";
    this.status = status;
    this.code = code;
    this.fields = fields;
  }
}

export function badRequest(message: string): AppError {
  return new AppError(400, "bad_request", message);
}

export function unauthorized(message = "Authentication required."): AppError {
  return new AppError(401, "unauthorized", message);
}

export function forbidden(message = "You do not have access."): AppError {
  return new AppError(403, "forbidden", message);
}

export function notFound(message = "Resource not found."): AppError {
  return new AppError(404, "not_found", message);
}

export function conflict(message: string): AppError {
  return new AppError(409, "conflict", message);
}

export function validationFailed(fields: ErrorFields): AppError {
  return new AppError(
    422,
    "validation_failed",
    "Request validation failed.",
    fields,
  );
}
