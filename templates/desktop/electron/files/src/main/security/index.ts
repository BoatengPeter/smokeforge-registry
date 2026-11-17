import type { Session } from "electron";

import type { AppConfig } from "../infra/config";

export function installSessionSecurity(session: Session, config: AppConfig): void {
  const connectSource = config.isDevelopment
    ? "connect-src 'self' http://localhost:* ws://localhost:*"
    : "connect-src 'self'";
  const scriptSource = config.isDevelopment
    ? "script-src 'self' 'unsafe-inline'"
    : "script-src 'self'";
  const csp = [
    "default-src 'self'",
    scriptSource,
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data:",
    connectSource,
    "font-src 'self'",
    "object-src 'none'",
    "base-uri 'self'",
    "frame-ancestors 'none'",
  ].join("; ");

  session.setPermissionRequestHandler((_webContents, _permission, callback) => callback(false));
  session.setPermissionCheckHandler(() => false);

  session.webRequest.onHeadersReceived((details, callback) => {
    callback({
      responseHeaders: {
        ...details.responseHeaders,
        "Content-Security-Policy": [csp],
      },
    });
  });
}
