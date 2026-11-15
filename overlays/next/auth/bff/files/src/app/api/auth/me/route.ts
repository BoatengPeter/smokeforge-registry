import { NextResponse } from "next/server";
import {
  accessToken,
  assertSameOrigin,
  authError,
  clearSession,
  currentUser,
  refreshTokens,
  setSession,
} from "@/lib/auth/bff";

export async function GET(request: Request) {
  const originError = assertSameOrigin(request);
  if (originError) return originError;

  const token = await accessToken();
  if (token) {
    const user = await currentUser(token);
    if (user) {
      return NextResponse.json({ data: user });
    }
  }

  const tokens = await refreshTokens();
  if (!tokens) {
    const result = authError(401, "unauthorized", "Authentication required.");
    clearSession(result);

    return result;
  }

  const user = await currentUser(tokens.access_token);
  if (!user) {
    const result = authError(401, "unauthorized", "Authentication required.");
    clearSession(result);

    return result;
  }

  const result = NextResponse.json({ data: user });
  setSession(result, tokens);

  return result;
}
