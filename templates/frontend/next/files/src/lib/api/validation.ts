import { z } from "zod";
import { AppError } from "@/lib/api/app-error";

export async function parseJson<T extends z.ZodType>(
  request: Request,
  schema: T,
): Promise<z.infer<T>> {
  let input: unknown;

  try {
    input = await request.json();
  } catch {
    throw new AppError(400, "invalid_json", "Request body must be valid JSON.");
  }

  const result = schema.safeParse(input);
  if (!result.success) {
    const fields: Record<string, string[]> = {};

    for (const issue of result.error.issues) {
      const key = issue.path.join(".") || "body";
      fields[key] = [...(fields[key] ?? []), issue.message];
    }

    throw new AppError(
      422,
      "validation_failed",
      "Request validation failed.",
      fields,
    );
  }

  return result.data;
}
