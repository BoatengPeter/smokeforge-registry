import { BrowserWindow, app } from "electron";
import { autoUpdater, type UpdateInfo } from "electron-updater";

import { updateChannels } from "../../../shared/ipc/updates";
import type { UpdateState } from "../../../shared/ipc/updates";
import { log } from "../../infra/logger";
import { appSettings } from "../settings/settings.ipc";

const idle: UpdateState = {
  status: app.isPackaged ? "idle" : "unsupported",
  version: null,
  progress: 0,
  message: app.isPackaged ? "" : "Update checks are only available in packaged builds.",
  releaseNotes: "",
};

let state: UpdateState = idle;
let initialized = false;

function publish(next: UpdateState): UpdateState {
  state = next;
  for (const window of BrowserWindow.getAllWindows()) {
    if (!window.isDestroyed()) window.webContents.send(updateChannels.event, state);
  }

  return state;
}

function releaseNotes(info: UpdateInfo): string {
  if (typeof info.releaseNotes === "string") return info.releaseNotes;
  if (Array.isArray(info.releaseNotes)) return info.releaseNotes.map((note) => note.note ?? "").join("\n");
  return "";
}

function initialize(): void {
  if (initialized || !app.isPackaged) return;
  initialized = true;
  autoUpdater.autoDownload = false;
  autoUpdater.autoInstallOnAppQuit = true;
  autoUpdater.on("checking-for-update", () => publish({ ...state, status: "checking", message: "Checking for updates…" }));
  autoUpdater.on("update-available", (info) =>
    publish({ ...state, status: "available", version: info.version, releaseNotes: releaseNotes(info), message: "An update is available." }),
  );
  autoUpdater.on("update-not-available", () =>
    publish({ ...state, status: "up-to-date", message: "You are up to date." }),
  );
  autoUpdater.on("download-progress", (progress) =>
    publish({ ...state, status: "downloading", progress: Math.round(progress.percent), message: "Downloading update…" }),
  );
  autoUpdater.on("update-downloaded", (info) =>
    publish({ ...state, status: "downloaded", version: info.version, progress: 100, message: "Restart to install the update." }),
  );
  autoUpdater.on("error", (error) =>
    publish({ ...state, status: "error", message: error.message }),
  );
}

export function getUpdateState(): UpdateState {
  return state;
}

export async function checkForUpdates(): Promise<UpdateState> {
  initialize();
  if (!app.isPackaged) return publish(idle);

  const settings = await appSettings().read();
  autoUpdater.channel = settings.updateChannel;
  try {
    await autoUpdater.checkForUpdates();
  } catch (error) {
    const message = error instanceof Error ? error.message : "Update check failed.";
    log("warn", "update check failed", { error: message });
    publish({ ...state, status: "error", message });
  }

  return state;
}

export async function downloadUpdate(): Promise<UpdateState> {
  initialize();
  if (!app.isPackaged) return state;
  try {
    await autoUpdater.downloadUpdate();
  } catch (error) {
    publish({ ...state, status: "error", message: error instanceof Error ? error.message : "Download failed." });
  }

  return state;
}

export function installUpdate(): UpdateState {
  if (state.status === "downloaded") autoUpdater.quitAndInstall();

  return state;
}
