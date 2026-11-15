import "server-only";

import { cookies } from "next/headers";
import { NextResponse } from "next/server";
import { isSameOrigin, parseTokens, type TokenResponse } from "@/lib/auth/tokens";

export { parseTokens };
export type { TokenResponse };

const ACCESS_COOKIE = "sf_access";
const REFRESH_COOKIE = "sf_refresh";
const ACCESS_MAX_AGE = 15 * 60;
const REFRESH_MAX_AGE = 30 * 24 * 60 * 60;

export type AuthUser = {
  id: string;
  email: string;
  role: string;
  email_verified?: boolean;
  created_at: string;
};

export type AuthErrorBody = {
  error: {
    code: string;
    message: string;
    fields?: Record<string, string[]>;
  };
};

function apiBase(): string {
  const value = process.env.API_URL?.trim();
  if (!value) {
    throw new Error("API_URL is required for the fullstack auth BFF.");
  }

  return value.replace(/\/$/, "");
}

function cookieOptions(maxAge: number) {
  return {
    httpOnly: true,
    sameSite: "lax" as const,
    secure: process.env.NODE_ENV === "production",
    path: "/",
    maxAge,
  };
}

export function authError(
  status: number,
  code: string,
  message: string,
  fields?: Record<string, string[]>,
): NextResponse<AuthErrorBody> {
  return NextResponse.json({ error: { code, message, fields } }, { status });
}

export function assertSameOrigin(request: Request): NextResponse | null {
  const origin = request.headers.get("origin");
  if (!origin) {
    return null;
  }

  const expected =
    process.env.NEXT_PUBLIC_APP_URL?.replace(/\/$/, "") ??
    new URL(request.url).origin;

  if (!isSameOrigin(origin, expected, request.url)) {
    return authError(403, "forbidden_origin", "Request origin is not allowed.");
  }

  return null;
}

export async function readBody(response: Response): Promise<unknown> {
  return response.json().catch(() => null);
}

export function apiError(response: Response, body: unknown): NextResponse {
  const problem = body as {
    title?: string;
    detail?: string;
    message?: string;
    errors?: Record<string, string[]>;
  } | null;

  return authError(
    response.status === 422 ? 422 : response.status,
    response.status === 401 ? "unauthorized" : "auth_failed",
    problem?.detail ?? problem?.message ?? problem?.title ?? "Authentication failed.",
    problem?.errors,
  );
}

export function setSession(response: NextResponse, tokens: TokenResponse): void {
  response.cookies.set(
    ACCESS_COOKIE,
    tokens.access_token,
    cookieOptions(tokens.expires_in > 0 ? tokens.expires_in : ACCESS_MAX_AGE),
  );

  if (tokens.refresh_token) {
    response.cookies.set(REFRESH_COOKIE, tokens.refresh_token, cookieOptions(REFRESH_MAX_AGE));
  }
}

export function clearSession(response: NextResponse): void {
  response.cookies.set(ACCESS_COOKIE, "", cookieOptions(0));
  response.cookies.set(REFRESH_COOKIE, "", cookieOptions(0));
}

export async function apiPost(
  path: string,
  body?: unknown,
  accessToken?: string,
): Promise<Response> {
  return fetch(`${apiBase()}${path}`, {
    method: "POST",
    cache: "no-store",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
      "X-Refresh-Token-Transport": "body",
      "X-Requested-With": "XMLHttpRequest",
      ...(accessToken ? { Authorization: `Bearer ${accessToken}` } : {}),
    },
    body: body === undefined ? undefined : JSON.stringify(body),
  });
}

export async function apiGet(path: string, accessToken: string): Promise<Response> {
  return fetch(`${apiBase()}${path}`, {
    cache: "no-store",
    headers: {
      Accept: "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
  });
}

export async function refreshTokens(): Promise<TokenResponse | null> {
  const store = await cookies();
  const refreshToken = store.get(REFRESH_COOKIE)?.value;
  if (!refreshToken) {
    return null;
  }

  const response = await apiPost("/api/v1/auth/refresh", { refresh_token: refreshToken });
  if (!response.ok) {
    return null;
  }

  return parseTokens(await readBody(response));
}

export async function currentUser(accessToken: string): Promise<AuthUser | null> {
  const response = await apiGet("/api/v1/auth/me", accessToken);
  if (!response.ok) {
    return null;
  }

  const body = await readBody(response);
  return body as AuthUser;
}

export async function accessToken(): Promise<string | undefined> {
  const store = await cookies();

  return store.get(ACCESS_COOKIE)?.value;
}

export async function refreshCookie(): Promise<string | undefined> {
  const store = await cookies();

  return store.get(REFRESH_COOKIE)?.value;
}
