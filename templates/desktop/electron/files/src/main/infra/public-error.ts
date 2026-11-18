export class PublicError extends Error {
  readonly code: string;

  constructor(code: string, message: string) {
    super(message);
    this.name = "PublicError";
    this.code = code;
  }
}
