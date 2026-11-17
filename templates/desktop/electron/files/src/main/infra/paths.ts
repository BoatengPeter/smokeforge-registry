import { join } from "node:path";

import { app } from "electron";

export function userDataPath(...parts: string[]): string {
  return join(app.getPath("userData"), ...parts);
}

export function logsPath(...parts: string[]): string {
  return join(app.getPath("logs"), ...parts);
}

export function resourcesPath(...parts: string[]): string {
  return join(process.resourcesPath, ...parts);
}

export function rendererPath(...parts: string[]): string {
  return join(app.getAppPath(), "out", "renderer", ...parts);
}
