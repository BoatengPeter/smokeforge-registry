import "server-only";
import { NextResponse } from "next/server";
import { ZodError } from "zod";
import { AppError } from "@/lib/api/app-error";
import { failure } from "@/lib/api/envelope";
import { logger } from "@/server/logger";

export function toErrorResponse(error: unknown): NextResponse {
  if (error instanceof AppError) {
    return NextResponse.json(
      failure(error.code, error.message, error.fields),
      { status: error.status },
    );
  }

  if (error instanceof ZodError) {
    const fields: Record<string, string[]> = {};

    for (const issue of error.issues) {
      const key = issue.path.join(".") || "body";
      fields[key] = [...(fields[key] ?? []), issue.message];
    }

    return NextResponse.json(
      failure("validation_failed", "Request validation failed.", fields),
      { status: 422 },
    );
  }

  logger.error({ err: error }, "unhandled api error");

  return NextResponse.json(
    failure("internal_error", "An unexpected error occurred."),
    { status: 500 },
  );
}
