import { contextBridge, ipcRenderer } from "electron";

import { systemChannels } from "../shared/ipc/channels";
import type { DesktopApi } from "../shared/types/desktop-api";
// smokeforge:anchor preload-imports

const desktopApi: DesktopApi = {
  system: {
    getVersion: () => ipcRenderer.invoke(systemChannels.getVersion),
    ping: () => ipcRenderer.invoke(systemChannels.ping),
  },
  // smokeforge:anchor preload-properties
};

contextBridge.exposeInMainWorld("desktopApi", desktopApi);
