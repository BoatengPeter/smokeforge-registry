import type { Result } from "@shared/types/result";

export class DesktopApiError extends Error {
  readonly code: string;
  readonly fields?: Record<string, string[]>;

  constructor(code: string, message: string, fields?: Record<string, string[]>) {
    super(message);
    this.name = "DesktopApiError";
    this.code = code;
    this.fields = fields;
  }
}

export async function invoke<T>(operation: Promise<Result<T>>): Promise<T> {
  const result = await operation;

  if (!result.ok) {
    throw new DesktopApiError(result.error.code, result.error.message, result.error.fields);
  }

  return result.data;
}
