import { z } from "zod";

export const localDataChannels = {
  get: "local-data:get",
  set: "local-data:set",
  delete: "local-data:delete",
} as const;

export const localDataSchemas = {
  get: z.object({ key: z.string().trim().min(1).max(256) }),
  set: z.object({ key: z.string().trim().min(1).max(256), value: z.unknown() }),
  delete: z.object({ key: z.string().trim().min(1).max(256) }),
};
