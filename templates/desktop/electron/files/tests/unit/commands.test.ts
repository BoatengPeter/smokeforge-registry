import { describe, expect, it } from "vitest";

import { acceleratorFor, appCommands, commandIds, formatAccelerator } from "../../src/shared/commands";

describe("application commands", () => {
  it("uses unique command ids", () => {
    const ids = appCommands.map((command) => command.id);

    expect(new Set(ids).size).toBe(ids.length);
    expect(appCommands.some((command) => command.id === commandIds.palette)).toBe(true);
  });

  it("formats accelerators per platform", () => {
    expect(formatAccelerator("CmdOrCtrl+K", false)).toBe("Ctrl+K");
    expect(formatAccelerator("CmdOrCtrl+K", true)).toBe("⌘K");
    expect(formatAccelerator("CmdOrCtrl+Shift+L", false)).toBe("Ctrl+Shift+L");
    expect(acceleratorFor(commandIds.palette)).toBe("CmdOrCtrl+K");
    expect(acceleratorFor(commandIds.welcome)).toBeUndefined();
  });
});
