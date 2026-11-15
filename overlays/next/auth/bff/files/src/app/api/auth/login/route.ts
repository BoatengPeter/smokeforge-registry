import { NextResponse } from "next/server";
import { z } from "zod";
import {
  apiError,
  apiPost,
  assertSameOrigin,
  authError,
  currentUser,
  parseTokens,
  readBody,
  setSession,
} from "@/lib/auth/bff";

const schema = z.object({
  email: z.string().email(),
  password: z.string().min(1),
});

export async function POST(request: Request) {
  const originError = assertSameOrigin(request);
  if (originError) return originError;

  const parsed = schema.safeParse(await request.json().catch(() => null));
  if (!parsed.success) {
    return authError(422, "validation_failed", "Enter a valid email and password.");
  }

  const response = await apiPost("/api/v1/auth/login", parsed.data);
  const body = await readBody(response);

  if (!response.ok) {
    return apiError(response, body);
  }

  const tokens = parseTokens(body);
  if (!tokens) {
    return authError(401, "mfa_required", "Complete the second-factor challenge.");
  }

  const user = await currentUser(tokens.access_token);
  if (!user) {
    return authError(502, "auth_unavailable", "The API did not return the current user.");
  }

  const result = NextResponse.json({ data: user });
  setSession(result, tokens);

  return result;
}
