import { BrowserWindow, app, nativeTheme } from "electron";

import { settingsChannels, settingsSchemas } from "../../../shared/ipc/settings";
import { userDataPath } from "../../infra/paths";
import { setLogLevel } from "../../infra/logger";
import { SettingsStore } from "../../infra/settings";
import { handle } from "../../ipc/handle";

let store: SettingsStore | null = null;

export function appSettings(): SettingsStore {
  store ??= new SettingsStore(userDataPath("settings.json"));

  return store;
}

/** Applies the interface size to every open window. */
export function applyZoom(factor: number): void {
  for (const window of BrowserWindow.getAllWindows()) {
    window.webContents.setZoomFactor(factor);
  }
}

function applySideEffects(patch: { theme?: string; launchAtLogin?: boolean; logLevel?: string; zoom?: number }): void {
  if (typeof patch.zoom === "number") {
    applyZoom(patch.zoom);
  }

  if (patch.theme === "light" || patch.theme === "dark" || patch.theme === "system") {
    nativeTheme.themeSource = patch.theme;
  }

  if (typeof patch.launchAtLogin === "boolean") {
    app.setLoginItemSettings({ openAtLogin: patch.launchAtLogin });
  }

  if (patch.logLevel === "debug" || patch.logLevel === "info" || patch.logLevel === "warn" || patch.logLevel === "error") {
    setLogLevel(patch.logLevel);
  }
}

export function registerSettingsHandlers(): void {
  handle(settingsChannels.get, settingsSchemas.get, () => appSettings().read());
  handle(settingsChannels.update, settingsSchemas.update, async (patch) => {
    const next = await appSettings().update(patch);
    applySideEffects(patch);

    return next;
  });
}
