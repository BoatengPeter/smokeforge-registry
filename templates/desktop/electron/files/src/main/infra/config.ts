import { app } from "electron";

export type AppConfig = {
  apiBaseUrl: string;
  isDevelopment: boolean;
};

export function loadConfig(): AppConfig {
  return {
    apiBaseUrl: process.env.DESKTOP_API_URL?.trim() || "http://localhost:8080",
    isDevelopment: !app.isPackaged,
  };
}
