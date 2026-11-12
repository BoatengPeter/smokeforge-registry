import { withApi } from "@/lib/api/handler";
import { jsonOk } from "@/lib/api/response";

export const GET = withApi(() => {
  return jsonOk({
    status: "ok",
    service: "web",
  });
});
