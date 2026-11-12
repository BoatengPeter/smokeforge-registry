import "server-only";
import { toErrorResponse } from "@/lib/api/error-response";

export type ApiHandler = (request: Request) => Response | Promise<Response>;

export function withApi(handler: ApiHandler): ApiHandler {
  return async (request) => {
    try {
      return await handler(request);
    } catch (error) {
      return toErrorResponse(error);
    }
  };
}
