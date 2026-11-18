import type { CommandId } from "../commands";
import type { ActivityApi } from "../ipc/activity";
import type { AppSettings, SettingsPatch } from "../ipc/settings";
// smokeforge:anchor desktop-api-imports
import type { SystemInfo, SystemPing } from "../ipc/system";
import type { WindowApi } from "../ipc/windows";
import type { UpdatesApi } from "../ipc/updates";
import type { Result } from "./result";

export type SystemApi = {
  getVersion: () => Promise<Result<SystemInfo>>;
  ping: () => Promise<Result<SystemPing>>;
  openLogs: () => Promise<Result<{ opened: boolean }>>;
  exportDiagnostics: () => Promise<Result<{ path: string } | null>>;
};

export type SettingsApi = {
  get: () => Promise<Result<AppSettings>>;
  update: (patch: SettingsPatch) => Promise<Result<AppSettings>>;
};

export type CommandsApi = {
  on: (listener: (id: CommandId) => void) => () => void;
};

export type FilesApi = {
  pathForFile: (file: File) => string;
};

export type DesktopApi = {
  system: SystemApi;
  settings: SettingsApi;
  commands: CommandsApi;
  activity: ActivityApi;
  windows: WindowApi;
  updates: UpdatesApi;
  files: FilesApi;
  // smokeforge:anchor desktop-api-properties
};
