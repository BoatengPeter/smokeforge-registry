import { app } from "electron";

import { systemChannels } from "../../../shared/ipc/channels";
import { systemSchemas } from "../../../shared/ipc/system";
import { handle } from "../../ipc/handle";
import { systemInfo, systemPing } from "./system.service";

export function registerSystemHandlers(): void {
  handle(systemChannels.getVersion, systemSchemas.getVersion, () =>
    systemInfo(app.getVersion(), process.platform, process.arch),
  );

  handle(systemChannels.ping, systemSchemas.ping, () => systemPing());
}
