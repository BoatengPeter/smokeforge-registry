import { BrowserWindow, Menu, app, type MenuItemConstructorOptions } from "electron";

import { commandChannel, commandIds, type CommandId } from "../../shared/commands";

function send(id: CommandId): void {
  BrowserWindow.getFocusedWindow()?.webContents.send(commandChannel, id);
}

function commandItem(label: string, id: CommandId, accelerator?: string): MenuItemConstructorOptions {
  return { label, accelerator, click: () => send(id) };
}

export function installApplicationMenu(): void {
  const isMac = process.platform === "darwin";

  const template: MenuItemConstructorOptions[] = [
    ...(isMac
      ? [
          {
            label: app.name,
            submenu: [
              { role: "about" as const },
              { type: "separator" as const },
              commandItem("Settings…", commandIds.settings, "CmdOrCtrl+,"),
              { type: "separator" as const },
              { role: "services" as const },
              { type: "separator" as const },
              { role: "hide" as const },
              { role: "hideOthers" as const },
              { role: "unhide" as const },
              { type: "separator" as const },
              { role: "quit" as const },
            ],
          },
        ]
      : []),
    {
      label: "File",
      submenu: [
        commandItem("New library item", commandIds.newItem, "CmdOrCtrl+N"),
        { type: "separator" },
        ...(!isMac ? [commandItem("Settings", commandIds.settings, "CmdOrCtrl+,")] as MenuItemConstructorOptions[] : []),
        ...(!isMac ? [{ type: "separator" } as MenuItemConstructorOptions, { role: "quit" } as MenuItemConstructorOptions] : []),
      ],
    },
    {
      label: "Edit",
      submenu: [
        { role: "undo" },
        { role: "redo" },
        { type: "separator" },
        { role: "cut" },
        { role: "copy" },
        { role: "paste" },
        { role: "selectAll" },
      ],
    },
    {
      label: "View",
      submenu: [
        commandItem("Command palette", commandIds.palette, "CmdOrCtrl+K"),
        commandItem("Toggle sidebar", commandIds.toggleSidebar, "CmdOrCtrl+B"),
        { type: "separator" },
        // Interface size is a setting, so these go through the renderer
        // (settings.update) instead of Electron's zoom roles. The renderer's
        // key handler owns the shortcuts; the menu only shows them.
        { ...commandItem("Actual size", commandIds.zoomReset, "CmdOrCtrl+0"), registerAccelerator: false },
        { ...commandItem("Zoom in", commandIds.zoomIn, "CmdOrCtrl+="), registerAccelerator: false },
        { ...commandItem("Zoom out", commandIds.zoomOut, "CmdOrCtrl+-"), registerAccelerator: false },
        { type: "separator" },
        { role: "togglefullscreen" },
        ...(app.isPackaged ? [] : [{ type: "separator" } as MenuItemConstructorOptions, { role: "reload" } as MenuItemConstructorOptions, { role: "toggleDevTools" } as MenuItemConstructorOptions]),
      ],
    },
    {
      label: "Go",
      submenu: [
        commandItem("Home", commandIds.home, "CmdOrCtrl+1"),
        commandItem("Library", commandIds.library, "CmdOrCtrl+2"),
        commandItem("Activity", commandIds.activity, "CmdOrCtrl+3"),
        commandItem("Account", commandIds.account),
        commandItem("About", commandIds.about),
      ],
    },
    {
      label: "Window",
      submenu: [{ role: "minimize" }, { role: "zoom" }, ...(isMac ? [{ type: "separator" } as MenuItemConstructorOptions, { role: "front" } as MenuItemConstructorOptions] : [])],
    },
  ];

  Menu.setApplicationMenu(Menu.buildFromTemplate(template));
}
