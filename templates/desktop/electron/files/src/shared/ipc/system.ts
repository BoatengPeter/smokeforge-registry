import { z } from "zod";

export { systemChannels } from "./channels";

export const systemSchemas = {
  getVersion: z.void(),
  ping: z.void(),
  openLogs: z.void(),
  exportDiagnostics: z.void(),
};

export type SystemInfo = {
  version: string;
  platform: string;
  arch: string;
  electron: string;
  chrome: string;
  node: string;
  userDataPath: string;
  logsPath: string;
};

export type SystemPing = {
  message: "pong";
};
