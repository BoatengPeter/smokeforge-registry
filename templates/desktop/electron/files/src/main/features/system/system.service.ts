import type { SystemInfo, SystemPing } from "../../../shared/ipc/system";

export function systemInfo(values: SystemInfo): SystemInfo {
  return values;
}

export function systemPing(): SystemPing {
  return { message: "pong" };
}
