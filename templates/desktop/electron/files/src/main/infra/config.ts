import { app } from "electron";

export type AppConfig = {
  apiBaseUrl: string;
  syncEnabled: boolean;
  isDevelopment: boolean;
};

export function loadConfig(): AppConfig {
  return {
    apiBaseUrl: process.env.DESKTOP_API_URL?.trim() || "http://localhost:8080",
    syncEnabled: process.env.DESKTOP_SYNC_ENABLED === "true",
    isDevelopment: !app.isPackaged,
  };
}
