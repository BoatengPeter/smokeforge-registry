type Level = "debug" | "info" | "warn" | "error";

const redactKeys = new Set(["token", "access_token", "refresh_token", "password", "secret"]);

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

  const entry = JSON.stringify({
    level,
    message,
    time: new Date().toISOString(),
    ...(safeContext && typeof safeContext === "object" ? safeContext : {}),
  });

  if (level === "error") {
    console.error(entry);
    return;
  }

  if (level === "warn") {
    console.warn(entry);
    return;
  }

  console.log(entry);
}
