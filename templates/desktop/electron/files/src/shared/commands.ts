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
];

export const commandChannel = "app:command";
