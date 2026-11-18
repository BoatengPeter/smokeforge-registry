import { BrowserWindow, Menu, Tray, app, nativeImage } from "electron";

import { commandChannel, commandIds } from "../../shared/commands";
import { createMainWindow } from "../windows/main-window";

const icon = nativeImage.createFromDataURL(
  "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAYAAAAf8/9hAAAAOElEQVR42mNgoDbQX73qPz5MkWa8hhCrGashpGrGMIQmBvz+8RkFj0QD6BsLFCckqiRlqmQmcgAA9R3GNFgSnfcAAAAASUVORK5CYII=",
);

let tray: Tray | null = null;

function mainWindow(): BrowserWindow {
  return BrowserWindow.getAllWindows()[0] ?? createMainWindow();
}

function toggleWindow(): void {
  const window = mainWindow();
  if (window.isVisible()) {
    window.hide();
  } else {
    window.show();
    window.focus();
  }
}

export function installTray(): void {
  if (tray) return;
  tray = new Tray(icon);
  tray.setToolTip(app.name);
  tray.setContextMenu(
    Menu.buildFromTemplate([
      { label: "Show/Hide", click: toggleWindow },
      {
        label: "New library item",
        click: () => mainWindow().webContents.send(commandChannel, commandIds.newItem),
      },
      {
        label: "Settings",
        click: () => mainWindow().webContents.send(commandChannel, commandIds.settings),
      },
      { type: "separator" },
      { label: "Quit", role: "quit" },
    ]),
  );
  tray.on("click", toggleWindow);
}

export function hasTray(): boolean {
  return tray !== null;
}
