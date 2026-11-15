import { AppError } from "@/lib/api/app-error";
import { env } from "@/lib/env";
import type { ApiErrorBody } from "@/lib/api/envelope";


export async function apiFetch<T>(
  path: string,
  init: RequestInit = {},
): Promise<T> {
  if (!path.startsWith("/")) {
    throw new Error("API path must start with /");
  }

  const response = await fetch(`${env.NEXT_PUBLIC_API_URL ?? ""}${path}`, {
    ...init,
    headers: {
      Accept: "application/json",
      ...(init.body ? { "Content-Type": "application/json" } : {}),
      ...init.headers,
    },
  });

  if (response.status === 204) {
    return undefined as T;
  }

  const body: unknown = await response.json().catch(() => null);

  if (!response.ok) {
    const errorBody = body as ApiErrorBody | null;
    throw new AppError(
      response.status,
      errorBody?.error.code ?? "request_failed",
      errorBody?.error.message ?? `Request failed with status ${response.status}.`,
      errorBody?.error.fields,
    );
  }

  return (body as { data: T }).data;
}
