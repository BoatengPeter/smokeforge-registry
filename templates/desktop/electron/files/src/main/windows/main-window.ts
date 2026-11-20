import { BrowserWindow, app, shell } from "electron";
import windowStateKeeper from "electron-window-state";
import { join } from "node:path";

import { windowChrome } from "../../shared/design-tokens";
import { isTrustedRendererUrl } from "../../shared/security";
import { loadAppIcon } from "./app-icon";

const titleBarOverlay = windowChrome.titleBarOverlay;

export function createMainWindow(): BrowserWindow {
  const devUrl = process.env.ELECTRON_RENDERER_URL;
  const icon = loadAppIcon();
  const windowState = windowStateKeeper({
    defaultWidth: 1180,
    defaultHeight: 760,
    file: "window-state.json",
  });

  const window = new BrowserWindow({
    x: windowState.x,
    y: windowState.y,
    width: windowState.width,
    height: windowState.height,
    minWidth: 760,
    minHeight: 520,
    show: false,
    icon,
    backgroundColor: windowChrome.background,
    titleBarStyle: process.platform === "darwin" ? "hiddenInset" : "hidden",
    ...(process.platform === "darwin"
      ? { trafficLightPosition: { x: 14, y: 14 } }
      : { titleBarOverlay }),
    webPreferences: {
      preload: join(__dirname, "../preload/index.js"),
      contextIsolation: true,
      sandbox: true,
      nodeIntegration: false,
      webSecurity: true,
      devTools: !app.isPackaged,
    },
  });

  windowState.manage(window);

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
