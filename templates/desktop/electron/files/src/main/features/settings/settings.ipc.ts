import { app, nativeTheme } from "electron";

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

function applySideEffects(patch: { theme?: string; launchAtLogin?: boolean; logLevel?: string }): void {
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
