export type TokenResponse = {
  access_token: string;
  refresh_token?: string;
  token_type?: string;
  expires_in: number;
};

export function parseTokens(body: unknown): TokenResponse | null {
  if (typeof body !== "object" || body === null || !("access_token" in body)) {
    return null;
  }

  const tokens = body as Partial<TokenResponse>;
  if (typeof tokens.access_token !== "string" || typeof tokens.expires_in !== "number") {
    return null;
  }

  return {
    access_token: tokens.access_token,
    refresh_token: typeof tokens.refresh_token === "string" ? tokens.refresh_token : undefined,
    token_type: tokens.token_type,
    expires_in: tokens.expires_in,
  };
}

export function isSameOrigin(
  origin: string,
  expected: string | undefined,
  requestUrl: string,
): boolean {
  const target = (expected ?? new URL(requestUrl).origin).replace(/\/$/, "");

  return origin.replace(/\/$/, "") === target;
}
