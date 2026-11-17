import { mkdir, readFile, rename, unlink, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

import { z } from "zod";

const settingsSchema = z.object({
  schemaVersion: z.literal(1),
  theme: z.enum(["system", "light", "dark"]).default("system"),
  sidebarOpen: z.boolean().default(false),
  lastRoute: z.string().default("/"),
});

export type Settings = z.infer<typeof settingsSchema>;

const defaults: Settings = {
  schemaVersion: 1,
  theme: "system",
  sidebarOpen: false,
  lastRoute: "/",
};

export class SettingsStore {
  constructor(private readonly filePath: string) {}

  async read(): Promise<Settings> {
    try {
      const raw = await readFile(this.filePath, "utf8");
      const parsed = settingsSchema.safeParse(JSON.parse(raw));

      if (parsed.success) {
        return parsed.data;
      }
    } catch (error) {
      if ((error as NodeJS.ErrnoException).code !== "ENOENT") {
        await this.quarantine();
      }
    }

    return { ...defaults };
  }

  async update(patch: Partial<Omit<Settings, "schemaVersion">>): Promise<Settings> {
    const current = await this.read();
    const next = settingsSchema.parse({
      ...current,
      ...patch,
      schemaVersion: 1,
    });

    await this.write(next);

    return next;
  }

  private async write(settings: Settings): Promise<void> {
    await mkdir(dirname(this.filePath), { recursive: true, mode: 0o700 });

    const temporary = `${this.filePath}.${process.pid}.tmp`;

    try {
      await writeFile(temporary, `${JSON.stringify(settings, null, 2)}\n`, {
        encoding: "utf8",
        mode: 0o600,
      });
      await rename(temporary, this.filePath);
    } catch (error) {
      await unlink(temporary).catch(() => undefined);
      throw error;
    }
  }

  private async quarantine(): Promise<void> {
    try {
      await rename(this.filePath, `${this.filePath}.corrupt-${Date.now()}`);
    } catch {
      // A missing or unreadable file still falls back to defaults.
    }
  }
}
