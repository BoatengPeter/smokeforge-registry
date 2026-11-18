import { updateChannels, updateSchemas } from "../../../shared/ipc/updates";
import { handle } from "../../ipc/handle";
import { checkForUpdates, downloadUpdate, getUpdateState, installUpdate } from "./update.service";

export function registerUpdateHandlers(): void {
  handle(updateChannels.check, updateSchemas.check, () => checkForUpdates());
  handle(updateChannels.download, updateSchemas.download, () => downloadUpdate());
  handle(updateChannels.install, updateSchemas.install, () => installUpdate());
  handle(updateChannels.get, updateSchemas.get, () => getUpdateState());
}
