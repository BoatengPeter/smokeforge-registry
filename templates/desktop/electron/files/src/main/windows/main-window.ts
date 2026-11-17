import { BrowserWindow, app, shell } from "electron";
import { join } from "node:path";

import { isTrustedRendererUrl } from "../../shared/security";

export function createMainWindow(): BrowserWindow {
  const devUrl = process.env.ELECTRON_RENDERER_URL;
  const window = new BrowserWindow({
    width: 1180,
    height: 760,
    minWidth: 760,
    minHeight: 520,
    show: false,
    backgroundColor: "#0f172a",
    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      contextIsolation: true,
      sandbox: true,
      nodeIntegration: false,
      webSecurity: true,
      devTools: !app.isPackaged,
    },
  });

  window.webContents.setWindowOpenHandler(({ url }) => {
    if (url.startsWith("https://")) {
      void shell.openExternal(url);
    }

    return { action: "deny" };
  });

  window.webContents.on("will-navigate", (event, url) => {
    if (!isTrustedRendererUrl(url, devUrl)) {
      event.preventDefault();
    }
  });

  window.once("ready-to-show", () => window.show());

  if (devUrl) {
    void window.loadURL(devUrl);
  } else {
    void window.loadURL("app://bundle/index.html");
  }

  return window;
}
