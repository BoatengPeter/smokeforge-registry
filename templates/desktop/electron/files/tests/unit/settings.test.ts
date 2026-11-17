import { mkdtemp, readFile, writeFile } from "node:fs/promises";
import { join } from "node:path";
import { tmpdir } from "node:os";

import { describe, expect, it } from "vitest";

import { SettingsStore } from "../../src/main/infra/settings";

describe("SettingsStore", () => {
  it("returns defaults when the file is missing", async () => {
    const dir = await mkdtemp(join(tmpdir(), "smokeforge-settings-"));
    const store = new SettingsStore(join(dir, "settings.json"));

    await expect(store.read()).resolves.toMatchObject({
      schemaVersion: 1,
      theme: "system",
      sidebarOpen: false,
    });
  });

  it("writes validated settings atomically and recovers from invalid JSON", async () => {
    const dir = await mkdtemp(join(tmpdir(), "smokeforge-settings-"));
    const file = join(dir, "settings.json");
    const store = new SettingsStore(file);

    await expect(store.update({ theme: "dark", sidebarOpen: true })).resolves.toMatchObject({
      theme: "dark",
      sidebarOpen: true,
    });
    expect(JSON.parse(await readFile(file, "utf8"))).toMatchObject({ theme: "dark" });

    await writeFile(file, "{not-json", "utf8");
    await expect(store.read()).resolves.toMatchObject({ theme: "system", sidebarOpen: false });
  });
});
