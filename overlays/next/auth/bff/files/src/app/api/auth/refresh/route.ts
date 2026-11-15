import { NextResponse } from "next/server";
import {
  assertSameOrigin,
  authError,
  clearSession,
  currentUser,
  refreshTokens,
  setSession,
} from "@/lib/auth/bff";

export async function POST(request: Request) {
  const originError = assertSameOrigin(request);
  if (originError) return originError;

  const tokens = await refreshTokens();
  if (!tokens) {
    const result = authError(401, "unauthorized", "Refresh session is invalid.");
    clearSession(result);

    return result;
  }

  const user = await currentUser(tokens.access_token);
  if (!user) {
    return authError(401, "unauthorized", "Authentication required.");
  }

  const result = NextResponse.json({ data: user });
  setSession(result, tokens);

  return result;
}
