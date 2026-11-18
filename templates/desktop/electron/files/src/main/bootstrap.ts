import { BrowserWindow, app, nativeTheme, session } from "electron";

import { installApplicationMenu } from "./app/menu";
import { appSettings } from "./features/settings/settings.ipc";
import { checkForUpdates } from "./features/updates/update.service";
import { createMainWindow } from "./windows/main-window";
import { registerAppProtocol, registerAppScheme } from "./app/protocol";
import { loadConfig } from "./infra/config";
import { configureLogging, log, setLogLevel } from "./infra/logger";
import { registerIpcHandlers } from "./ipc/register";
import { installSessionSecurity } from "./security";
// smokeforge:anchor deep-link-import
// smokeforge:anchor tray-import

export async function bootstrap(): Promise<void> {
  // smokeforge:anchor deep-link-register
  registerAppScheme();

  if (!app.requestSingleInstanceLock()) {
    app.quit();
    return;
  }

  app.on("second-instance", () => {
    // smokeforge:anchor deep-link-second-instance
    const window = BrowserWindow.getAllWindows()[0];
    if (window) {
      if (window.isMinimized()) window.restore();
      window.focus();
    }
  });

  app.on("window-all-closed", () => {
    if (process.platform !== "darwin") {
      // smokeforge:anchor tray-window-all-closed
      app.quit();
    }
  });

  await app.whenReady();

  const settings = await appSettings().read();
  configureLogging(settings.logLevel);
  setLogLevel(settings.logLevel);
  nativeTheme.themeSource = settings.theme;
  app.setLoginItemSettings({ openAtLogin: settings.launchAtLogin });
  installApplicationMenu();
  // smokeforge:anchor tray-install

  registerAppProtocol();
  installSessionSecurity(session.defaultSession, loadConfig());
  registerIpcHandlers();
  createMainWindow();

  if (app.isPackaged && settings.autoCheckUpdates) {
    setTimeout(() => void checkForUpdates(), 5_000);
  }

  app.on("activate", () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createMainWindow();
    }
  });

  log("info", "desktop app ready");
}
