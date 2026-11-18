import { contextBridge, ipcRenderer, webUtils } from "electron";

import { commandChannel, type CommandId } from "../shared/commands";
import { activityChannels } from "../shared/ipc/activity";
import { systemChannels } from "../shared/ipc/channels";
import { settingsChannels } from "../shared/ipc/settings";
import { windowChannels } from "../shared/ipc/windows";
import { updateChannels } from "../shared/ipc/updates";
import type { DesktopApi } from "../shared/types/desktop-api";
// smokeforge:anchor preload-imports

const desktopApi: DesktopApi = {
  system: {
    getVersion: () => ipcRenderer.invoke(systemChannels.getVersion),
    ping: () => ipcRenderer.invoke(systemChannels.ping),
    openLogs: () => ipcRenderer.invoke(systemChannels.openLogs),
    exportDiagnostics: () => ipcRenderer.invoke(systemChannels.exportDiagnostics),
  },
  settings: {
    get: () => ipcRenderer.invoke(settingsChannels.get),
    update: (patch) => ipcRenderer.invoke(settingsChannels.update, patch),
  },
  commands: {
    on: (listener) => {
      const handler = (_event: Electron.IpcRendererEvent, id: CommandId) => listener(id);
      ipcRenderer.on(commandChannel, handler);

      return () => ipcRenderer.removeListener(commandChannel, handler);
    },
  },
  activity: {
    start: () => ipcRenderer.invoke(activityChannels.start),
    cancel: () => ipcRenderer.invoke(activityChannels.cancel),
    retry: () => ipcRenderer.invoke(activityChannels.retry),
    get: () => ipcRenderer.invoke(activityChannels.get),
    onEvent: (listener) => {
      const handler = (_event: Electron.IpcRendererEvent, state: Parameters<typeof listener>[0]) => listener(state);
      ipcRenderer.on(activityChannels.event, handler);

      return () => ipcRenderer.removeListener(activityChannels.event, handler);
    },
  },
  windows: {
    openLibraryItem: (id: string) => ipcRenderer.invoke(windowChannels.openLibraryItem, { id }),
  },
  files: {
    pathForFile: (file: File) => webUtils.getPathForFile(file),
  },
  updates: {
    check: () => ipcRenderer.invoke(updateChannels.check),
    download: () => ipcRenderer.invoke(updateChannels.download),
    install: () => ipcRenderer.invoke(updateChannels.install),
    get: () => ipcRenderer.invoke(updateChannels.get),
    onEvent: (listener) => {
      const handler = (_event: Electron.IpcRendererEvent, state: Parameters<typeof listener>[0]) => listener(state);
      ipcRenderer.on(updateChannels.event, handler);

      return () => ipcRenderer.removeListener(updateChannels.event, handler);
    },
  },
  // smokeforge:anchor preload-properties
};

contextBridge.exposeInMainWorld("desktopApi", desktopApi);
