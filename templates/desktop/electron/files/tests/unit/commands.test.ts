import { describe, expect, it } from "vitest";

import { appCommands, commandIds } from "../../src/shared/commands";

describe("application commands", () => {
  it("uses unique command ids", () => {
    const ids = appCommands.map((command) => command.id);

    expect(new Set(ids).size).toBe(ids.length);
    expect(appCommands.some((command) => command.id === commandIds.palette)).toBe(true);
  });
});
