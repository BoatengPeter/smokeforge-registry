import { z } from "zod";

import type { Result } from "../types/result";

export const activityChannels = {
  start: "activity:start",
  cancel: "activity:cancel",
  retry: "activity:retry",
  get: "activity:get",
  event: "activity:event",
} as const;

export const activitySchemas = {
  start: z.void(),
  cancel: z.void(),
  retry: z.void(),
  get: z.void(),
};

export type ActivityStatus = "idle" | "running" | "complete" | "cancelled" | "failed";

export type ActivityState = {
  jobId: string | null;
  status: ActivityStatus;
  progress: number;
  step: string;
  message: string;
  startedAt: string | null;
  finishedAt: string | null;
};

export type ActivityApi = {
  start: () => Promise<Result<ActivityState>>;
  cancel: () => Promise<Result<ActivityState>>;
  retry: () => Promise<Result<ActivityState>>;
  get: () => Promise<Result<ActivityState>>;
  onEvent: (listener: (state: ActivityState) => void) => () => void;
};
