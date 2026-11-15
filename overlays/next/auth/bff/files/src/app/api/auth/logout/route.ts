import { NextResponse } from "next/server";
import {
  apiPost,
  assertSameOrigin,
  clearSession,
  refreshCookie,
} from "@/lib/auth/bff";

export async function POST(request: Request) {
  const originError = assertSameOrigin(request);
  if (originError) return originError;

  const refreshToken = await refreshCookie();
  if (refreshToken) {
    await apiPost("/api/v1/auth/logout", { refresh_token: refreshToken }).catch(() => null);
  }

  const result = new NextResponse(null, { status: 204 });
  clearSession(result);

  return result;
}
