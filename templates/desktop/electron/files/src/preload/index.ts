import { contextBridge, ipcRenderer } from "electron";

import { systemChannels } from "../shared/ipc/channels";
import type { DesktopApi } from "../shared/types/desktop-api";

const desktopApi: DesktopApi = {
  system: {
    getVersion: () => ipcRenderer.invoke(systemChannels.getVersion),
    ping: () => ipcRenderer.invoke(systemChannels.ping),
  },
};

contextBridge.exposeInMainWorld("desktopApi", desktopApi);
