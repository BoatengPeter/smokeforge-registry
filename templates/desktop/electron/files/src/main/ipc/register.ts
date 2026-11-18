import { registerActivityHandlers } from "../features/activity/activity.ipc";
import { registerSettingsHandlers } from "../features/settings/settings.ipc";
import { registerUpdateHandlers } from "../features/updates/update.ipc";
import { registerWindowHandlers } from "../features/windows/windows.ipc";
import { registerSystemHandlers } from "../features/system/system.ipc";
// smokeforge:anchor ipc-feature-imports

export function registerIpcHandlers(): void {
  registerSystemHandlers();
  registerActivityHandlers();
  registerSettingsHandlers();
  registerWindowHandlers();
  registerUpdateHandlers();
  // smokeforge:anchor ipc-feature-registrations
}
