import type { ScheduledTask } from "node-cron";

import { log } from "../infra/logger";
import { appSettings } from "../features/settings/settings.ipc";
import { openLocalDatabase } from "./database";

let task: ScheduledTask | null = null;

export async function startMaintenanceScheduler(enabled?: boolean): Promise<void> {
  task?.stop();
  task = null;
  const active = enabled ?? (await appSettings().read()).scheduledMaintenance;
  if (!active) return;

  const cron = await import("node-cron");
  log("info", "scheduled maintenance enabled", { schedule: "0 3 * * *" });
  task = cron.schedule("0 3 * * *", () => {
    try {
      openLocalDatabase().pragma("wal_checkpoint(TRUNCATE)");
      log("info", "scheduled maintenance completed");
    } catch (error) {
      log("warn", "scheduled maintenance failed", {
        error: error instanceof Error ? error.message : "unknown error",
      });
    }
  });
}

export function stopMaintenanceScheduler(): void {
  task?.stop();
  task = null;
}
