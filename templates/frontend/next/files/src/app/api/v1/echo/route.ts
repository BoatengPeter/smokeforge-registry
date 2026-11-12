import { z } from "zod";
import { withApi } from "@/lib/api/handler";
import { jsonOk } from "@/lib/api/response";
import { parseJson } from "@/lib/api/validation";

const echoSchema = z.object({
  message: z
    .string()
    .trim()
    .min(1, "Message is required.")
    .max(200, "Message must be 200 characters or fewer."),
});

export const POST = withApi(async (request) => {
  const input = await parseJson(request, echoSchema);

  return jsonOk({ message: input.message });
});
