/** @jest-environment node */

import { AppError } from "@/lib/api/app-error";
import { failure } from "@/lib/api/envelope";
import { toErrorResponse } from "@/lib/api/error-response";

describe("error responses", () => {
  it("maps AppError to the shared error shape", async () => {
    const response = toErrorResponse(
      new AppError(409, "conflict", "Already exists."),
    );

    expect(response.status).toBe(409);
    await expect(response.json()).resolves.toEqual(
      failure("conflict", "Already exists."),
    );
  });

  it("maps validation errors to field errors", async () => {
    const response = toErrorResponse(
      new AppError(422, "validation_failed", "Invalid.", {
        email: ["Required"],
      }),
    );

    expect(response.status).toBe(422);
    await expect(response.json()).resolves.toEqual(
      failure("validation_failed", "Invalid.", { email: ["Required"] }),
    );
  });

  it("hides unexpected internal errors", async () => {
    const response = toErrorResponse(new Error("database exploded"));

    expect(response.status).toBe(500);
    await expect(response.json()).resolves.toEqual(
      failure("internal_error", "An unexpected error occurred."),
    );
  });
});
