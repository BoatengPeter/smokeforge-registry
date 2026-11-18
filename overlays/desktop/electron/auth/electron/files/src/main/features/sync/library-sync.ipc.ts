import { syncChannels, syncSchemas } from "../../../shared/ipc/sync";
import { handle } from "../../ipc/handle";
import { getSyncStatus, listSyncConflicts, resolveSyncConflict } from "./library-sync";

export function registerSyncHandlers(): void {
  handle(syncChannels.status, syncSchemas.status, () => getSyncStatus());
  handle(syncChannels.conflicts, syncSchemas.conflicts, () => listSyncConflicts());
  handle(syncChannels.resolve, syncSchemas.resolve, async (input) => {
    await resolveSyncConflict(input.id, input.resolution);
    return { resolved: true };
  });
}
