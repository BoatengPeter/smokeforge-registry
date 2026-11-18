import type { Result } from "./result";
// smokeforge:anchor desktop-api-imports
import type { SystemInfo, SystemPing } from "../ipc/system";

export type SystemApi = {
  getVersion: () => Promise<Result<SystemInfo>>;
  ping: () => Promise<Result<SystemPing>>;
};

export type DesktopApi = {
  system: SystemApi;
  // smokeforge:anchor desktop-api-properties
};
