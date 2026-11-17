import { z } from "zod";

export { systemChannels } from "./channels";


export const systemSchemas = {
  getVersion: z.void(),
  ping: z.void(),
};

export type SystemInfo = {
  version: string;
  platform: string;
  arch: string;
};

export type SystemPing = {
  message: "pong";
};
