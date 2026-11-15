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
  password: z.string().min(12).max(256),
});

export async function POST(request: Request) {
  const originError = assertSameOrigin(request);
  if (originError) return originError;

  const parsed = schema.safeParse(await request.json().catch(() => null));
  if (!parsed.success) {
    return authError(422, "validation_failed", "Use a valid email and a 12+ character password.");
  }

  let response = await apiPost("/api/v1/auth/register", parsed.data);
  let body = await readBody(response);

  if (!response.ok) {
    return apiError(response, body);
  }

  let tokens = parseTokens(body);

  // Go returns the created user; Rust returns tokens. Normalize both by
  // logging in after registration when tokens are absent.
  if (!tokens) {
    response = await apiPost("/api/v1/auth/login", parsed.data);
    body = await readBody(response);

    if (!response.ok) {
      return apiError(response, body);
    }

    tokens = parseTokens(body);
  }

  if (!tokens) {
    return authError(502, "auth_unavailable", "The API did not return a session.");
  }

  const user = await currentUser(tokens.access_token);
  if (!user) {
    return authError(502, "auth_unavailable", "The API did not return the current user.");
  }

  const result = NextResponse.json({ data: user }, { status: 201 });
  setSession(result, tokens);

  return result;
}
