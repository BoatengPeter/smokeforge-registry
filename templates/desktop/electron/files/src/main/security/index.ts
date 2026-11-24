import type { Session } from "electron";

import { rendererContentSecurityPolicy } from "../../shared/csp";
import type { AppConfig } from "../infra/config";

export function installSessionSecurity(session: Session, config: AppConfig): void {
  const csp = rendererContentSecurityPolicy(config.isDevelopment);

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
