import { app, dialog, shell } from "electron";
import { readFile, writeFile } from "node:fs/promises";

import { systemChannels } from "../../../shared/ipc/channels";
import { systemSchemas } from "../../../shared/ipc/system";
import { handle } from "../../ipc/handle";
import { systemInfo, systemPing } from "./system.service";

export function registerSystemHandlers(): void {
  handle(systemChannels.getVersion, systemSchemas.getVersion, () =>
    systemInfo({
      version: app.getVersion(),
      platform: process.platform,
      arch: process.arch,
      electron: process.versions.electron,
      chrome: process.versions.chrome,
      node: process.versions.node,
      userDataPath: app.getPath("userData"),
      logsPath: app.getPath("logs"),
    }),
  );

  handle(systemChannels.ping, systemSchemas.ping, () => systemPing());
  handle(systemChannels.openLogs, systemSchemas.openLogs, async () => {
    const error = await shell.openPath(app.getPath("logs"));

    return { opened: error === "" };
  });
  handle(systemChannels.exportDiagnostics, systemSchemas.exportDiagnostics, async () => {
    const result = await dialog.showSaveDialog({
      title: "Export diagnostics",
      defaultPath: "smokeforge-diagnostics.json",
      filters: [{ name: "JSON", extensions: ["json"] }],
    });
    if (result.canceled || !result.filePath) return null;
    const logPath = `${app.getPath("logs")}/main.log`;
    const log = await readFile(logPath, "utf8").catch(() => "");
    await writeFile(result.filePath, JSON.stringify({
      generatedAt: new Date().toISOString(),
      app: app.getVersion(),
      electron: process.versions.electron,
      chrome: process.versions.chrome,
      node: process.versions.node,
      platform: process.platform,
      arch: process.arch,
      userData: app.getPath("userData"),
      logs: app.getPath("logs"),
      recentLog: log.split("\n").slice(-200).join("\n"),
    }, null, 2));

    return { path: result.filePath };
  });
}
