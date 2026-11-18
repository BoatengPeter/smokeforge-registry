import { z } from "zod";

import type { Result } from "../types/result";

export const syncChannels = {
  status: "sync:status",
  conflicts: "sync:conflicts",
  resolve: "sync:resolve",
} as const;

export const syncSchemas = {
  status: z.void(),
  conflicts: z.void(),
  resolve: z.object({
    id: z.string().uuid(),
    resolution: z.enum(["local", "server"]),
  }).strict(),
};

export type SyncStatus = {
  pending: number;
  conflicts: number;
  running: boolean;
};

export type SyncConflict = {
  id: string;
  entityId: string;
  localPayload: unknown;
  remotePayload: unknown;
  createdAt: string;
};

export type SyncApi = {
  status: () => Promise<Result<SyncStatus>>;
  conflicts: () => Promise<Result<SyncConflict[]>>;
  resolve: (input: { id: string; resolution: "local" | "server" }) => Promise<Result<{ resolved: true }>>;
};
