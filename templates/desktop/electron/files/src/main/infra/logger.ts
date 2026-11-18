import { app } from "electron";
import electronLog from "electron-log/main";

type Level = "debug" | "info" | "warn" | "error";

const redactKeys = new Set(["token", "access_token", "refresh_token", "password", "secret"]);

export const logger = electronLog;

export function configureLogging(level: Level = "info"): void {
  electronLog.initialize();
  electronLog.transports.file.level = level;
  electronLog.transports.file.maxSize = 5 * 1024 * 1024;
  electronLog.transports.file.resolvePathFn = () => `${app.getPath("logs")}/main.log`;
  electronLog.transports.console.level = level;
}

export function setLogLevel(level: Level): void {
  electronLog.transports.file.level = level;
  electronLog.transports.console.level = level;
}

function sanitize(value: unknown): unknown {
  if (!value || typeof value !== "object") {
    return value;
  }

  if (Array.isArray(value)) {
    return value.map(sanitize);
  }

  return Object.fromEntries(
    Object.entries(value).map(([key, item]) => [
      key,
      redactKeys.has(key.toLowerCase()) ? "[redacted]" : sanitize(item),
    ]),
  );
}

export function log(level: Level, message: string, context: Record<string, unknown> = {}): void {
  const safeContext = sanitize(context);

  electronLog[level](message, safeContext && typeof safeContext === "object" ? safeContext : {});
}
