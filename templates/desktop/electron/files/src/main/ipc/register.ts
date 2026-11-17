import { registerSystemHandlers } from "../features/system/system.ipc";
// smokeforge:anchor ipc-feature-imports

export function registerIpcHandlers(): void {
  registerSystemHandlers();
  // smokeforge:anchor ipc-feature-registrations
}
