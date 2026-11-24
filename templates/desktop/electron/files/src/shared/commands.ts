export const commandIds = {
  palette: "app:palette",
  home: "nav:home",
  library: "nav:library",
  activity: "nav:activity",
  settings: "nav:settings",
  account: "nav:account",
  about: "nav:about",
  newItem: "library:new",
  toggleSidebar: "app:toggle-sidebar",
  theme: "app:toggle-theme",
  checkUpdates: "updates:check",
  zoomIn: "view:zoom-in",
  zoomOut: "view:zoom-out",
  zoomReset: "view:zoom-reset",
  welcome: "help:welcome",
} as const;

export type CommandId = (typeof commandIds)[keyof typeof commandIds];

export type CommandDefinition = {
  id: CommandId;
  labelKey: string;
  accelerator?: string;
};

export const appCommands: CommandDefinition[] = [
  { id: commandIds.palette, labelKey: "commands.palette", accelerator: "CmdOrCtrl+K" },
  { id: commandIds.home, labelKey: "nav.home", accelerator: "CmdOrCtrl+1" },
  { id: commandIds.library, labelKey: "nav.library", accelerator: "CmdOrCtrl+2" },
  { id: commandIds.activity, labelKey: "nav.activity", accelerator: "CmdOrCtrl+3" },
  { id: commandIds.settings, labelKey: "nav.settings", accelerator: "CmdOrCtrl+," },
  { id: commandIds.account, labelKey: "nav.account" },
  { id: commandIds.about, labelKey: "nav.about" },
  { id: commandIds.newItem, labelKey: "commands.newItem", accelerator: "CmdOrCtrl+N" },
  { id: commandIds.toggleSidebar, labelKey: "commands.toggleSidebar", accelerator: "CmdOrCtrl+B" },
  { id: commandIds.theme, labelKey: "commands.toggleTheme" },
  { id: commandIds.checkUpdates, labelKey: "commands.checkUpdates" },
  { id: commandIds.zoomIn, labelKey: "commands.zoomIn", accelerator: "CmdOrCtrl+=" },
  { id: commandIds.zoomOut, labelKey: "commands.zoomOut", accelerator: "CmdOrCtrl+-" },
  { id: commandIds.zoomReset, labelKey: "commands.zoomReset", accelerator: "CmdOrCtrl+0" },
  { id: commandIds.welcome, labelKey: "commands.welcome" },
];

export const commandChannel = "app:command";

/**
 * An Electron accelerator as people read it: `CmdOrCtrl+K` is `⌘K` on macOS
 * and `Ctrl+K` elsewhere. Anything that shows a shortcut uses this on the
 * accelerators above, never a typed string.
 */
export function formatAccelerator(accelerator: string, isMac: boolean): string {
  const parts = accelerator.split("+").map((part) => {
    switch (part) {
      case "CmdOrCtrl":
      case "CommandOrControl":
        return isMac ? "⌘" : "Ctrl";
      case "Shift":
        return isMac ? "⇧" : "Shift";
      case "Alt":
        return isMac ? "⌥" : "Alt";
      default:
        return part;
    }
  });

  return isMac ? parts.join("") : parts.join("+");
}

export function acceleratorFor(id: CommandId): string | undefined {
  return appCommands.find((command) => command.id === id)?.accelerator;
}
