import { z } from "zod";

import type { Result } from "../types/result";

export const updateChannels = {
  check: "updates:check",
  download: "updates:download",
  install: "updates:install",
  get: "updates:get",
  event: "updates:event",
} as const;

export const updateSchemas = {
  check: z.void(),
  download: z.void(),
  install: z.void(),
  get: z.void(),
};

export type UpdateStatus =
  | "idle"
  | "unsupported"
  | "checking"
  | "available"
  | "downloading"
  | "downloaded"
  | "up-to-date"
  | "error";

export type UpdateState = {
  status: UpdateStatus;
  version: string | null;
  progress: number;
  message: string;
  releaseNotes: string;
};

export type UpdatesApi = {
  check: () => Promise<Result<UpdateState>>;
  download: () => Promise<Result<UpdateState>>;
  install: () => Promise<Result<UpdateState>>;
  get: () => Promise<Result<UpdateState>>;
  onEvent: (listener: (state: UpdateState) => void) => () => void;
};
