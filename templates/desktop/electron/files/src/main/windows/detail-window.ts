import { BrowserWindow, app, shell } from "electron";
import { join } from "node:path";

import { windowChrome } from "../../shared/design-tokens";
import { isTrustedRendererUrl } from "../../shared/security";
import { devRendererUrl } from "../infra/dev-server";
import { loadAppIcon } from "./app-icon";

export function openLibraryItemWindow(itemId: string): void {
  const devUrl = devRendererUrl();
  const icon = loadAppIcon();
  const window = new BrowserWindow({
    width: 920,
    height: 680,
    minWidth: 620,
    minHeight: 460,
    show: false,
    icon,
    backgroundColor: windowChrome.background,
    titleBarStyle: process.platform === "darwin" ? "hiddenInset" : "default",
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
    if (url.startsWith("https://")) void shell.openExternal(url);
    return { action: "deny" };
  });
  window.webContents.on("will-navigate", (event, url) => {
    if (!isTrustedRendererUrl(url, devUrl)) event.preventDefault();
  });
  window.once("ready-to-show", () => window.show());

  const route = `#/library/${itemId}`;
  if (devUrl) {
    void window.loadURL(`${devUrl}/${route}`);
  } else {
    void window.loadURL(`app://bundle/index.html${route}`);
  }
}
