import type { SystemInfo, SystemPing } from "../../../shared/ipc/system";

export function systemInfo(version: string, platform: string, arch: string): SystemInfo {
  return { version, platform, arch };
}

export function systemPing(): SystemPing {
  return { message: "pong" };
}
