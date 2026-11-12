export type AuthOTPType =
  | "email-verification"
  | "sign-in"
  | "forget-password"
  | "change-email";

export type AuthFieldErrors = Record<string, string[]>;
