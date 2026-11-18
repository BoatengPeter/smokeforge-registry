import { BrowserWindow, dialog } from "electron";
import { openLocalDatabase } from "../../storage/database";
import { startMaintenanceScheduler } from "../../storage/maintenance-scheduler";
import { readJson, writeJson } from "fs-extra";

import { libraryChannels, libraryImportSchema, librarySchemas } from "../../../shared/ipc/library";
import { PublicError } from "../../infra/public-error";
import { appSettings } from "../settings/settings.ipc";
import { handle } from "../../ipc/handle";
import {
  allLibraryItems,
  createLibraryItem,
  deleteLibraryItem,
  getLibraryItem,
  libraryStats,
  importLibraryItems,
  listLibraryItems,
  restoreLibraryItem,
  updateLibraryItem,
} from "../../storage/library-repository";

function broadcast(): void {
  for (const window of BrowserWindow.getAllWindows()) {
    if (!window.isDestroyed()) {
      window.webContents.send(libraryChannels.changed);
    }
  }
}

export function registerLibraryHandlers(): void {
  void startMaintenanceScheduler();
  handle(libraryChannels.list, librarySchemas.list, (input) => listLibraryItems(input));
  handle(libraryChannels.get, librarySchemas.get, (input) => getLibraryItem(input.id));
  handle(libraryChannels.create, librarySchemas.create, (input) => {
    const item = createLibraryItem(input);
    broadcast();

    return item;
  });
  handle(libraryChannels.update, librarySchemas.update, (input) => {
    const item = updateLibraryItem(input);
    broadcast();

    return item;
  });
  handle(libraryChannels.delete, librarySchemas.delete, (input) => {
    const deleted = deleteLibraryItem(input.id);
    broadcast();

    return { deleted };
  });
  handle(libraryChannels.restore, librarySchemas.restore, (input) => {
    const item = restoreLibraryItem(input.id);
    broadcast();

    return item;
  });
  handle(libraryChannels.stats, librarySchemas.stats, () => libraryStats());
  handle(libraryChannels.exportAll, librarySchemas.exportAll, async () => {
    const result = await dialog.showSaveDialog({
      title: "Export library",
      defaultPath: "library-export.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (result.canceled || !result.filePath) return null;
    const items = allLibraryItems();
    await writeJson(result.filePath, { version: 1, exportedAt: new Date().toISOString(), items }, { spaces: 2 });

    return { path: result.filePath, count: items.length };
  });
  handle(libraryChannels.importAll, librarySchemas.importAll, async () => {
    const result = await dialog.showOpenDialog({
      title: "Import library",
      properties: ["openFile"],
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (result.canceled || !result.filePaths[0]) return null;
    const raw = await readJson(result.filePaths[0]);
    const parsed = libraryImportSchema.safeParse(raw);
    if (!parsed.success) throw new PublicError("invalid_import", "The selected file is not a valid library export.");
    const imported = importLibraryItems(parsed.data.items);
    broadcast();

    return { imported, skipped: parsed.data.items.length - imported };
  });
  handle(libraryChannels.importPath, librarySchemas.importPath, async (input) => {
    const raw = await readJson(input.path);
    const parsed = libraryImportSchema.safeParse(raw);
    if (!parsed.success) throw new PublicError("invalid_import", "The selected file is not a valid library export.");
    const imported = importLibraryItems(parsed.data.items);
    broadcast();

    return { imported, skipped: parsed.data.items.length - imported };
  });
  handle(libraryChannels.maintenance, librarySchemas.maintenance, async (input) => {
    await appSettings().update({ scheduledMaintenance: input.enabled });
    await startMaintenanceScheduler(input.enabled);

    return { enabled: input.enabled };
  });
  handle(libraryChannels.backup, librarySchemas.backup, async () => {
    const result = await dialog.showSaveDialog({
      title: "Back up local database",
      defaultPath: "local-backup.sqlite",
      filters: [{ name: "SQLite", extensions: ["sqlite"] }],
    });
    if (result.canceled || !result.filePath) return null;
    await openLocalDatabase().backup(result.filePath);

    return { path: result.filePath };
  });
}
