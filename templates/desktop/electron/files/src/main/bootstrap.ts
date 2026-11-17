import { BrowserWindow, app, session } from "electron";

import { createMainWindow } from "./windows/main-window";
import { registerAppProtocol, registerAppScheme } from "./app/protocol";
import { loadConfig } from "./infra/config";
import { log } from "./infra/logger";
import { registerIpcHandlers } from "./ipc/register";
import { installSessionSecurity } from "./security";

export async function bootstrap(): Promise<void> {
  registerAppScheme();

  if (!app.requestSingleInstanceLock()) {
    app.quit();
    return;
  }

  app.on("second-instance", () => {
    const window = BrowserWindow.getAllWindows()[0];
    if (window) {
      if (window.isMinimized()) window.restore();
      window.focus();
    }
  });

  app.on("window-all-closed", () => {
    if (process.platform !== "darwin") {
      app.quit();
    }
  });

  await app.whenReady();

  registerAppProtocol();
  installSessionSecurity(session.defaultSession, loadConfig());
  registerIpcHandlers();
  createMainWindow();

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createMainWindow();
    }
  });

  log("info", "desktop app ready");
}
