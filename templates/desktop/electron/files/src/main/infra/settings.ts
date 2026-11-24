import { mkdir, readFile, rename, unlink, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

import { z } from "zod";

import { zoom } from "../../shared/zoom";

const settingsSchema = z.object({
  schemaVersion: z.literal(1),
  theme: z.enum(["system", "light", "dark"]).default("system"),
  language: z.string().default("en"),
  sidebarOpen: z.boolean().default(true),
  lastRoute: z.string().default("/"),
  launchAtLogin: z.boolean().default(false),
  logLevel: z.enum(["debug", "info", "warn", "error"]).default("info"),
  updateChannel: z.enum(["stable", "beta"]).default("stable"),
  autoCheckUpdates: z.boolean().default(true),
  onboardingComplete: z.boolean().default(false),
  zoom: z.number().min(zoom.min).max(zoom.max).default(zoom.default),
  lastSeenVersion: z.string().default(""),
  scheduledMaintenance: z.boolean().default(false),
  dashboardOrder: z
    .array(z.enum(["active", "total", "archived"]))
    .length(3)
    .refine((items) => new Set(items).size === 3, "Dashboard order must contain each card once.")
    .default(["active", "total", "archived"]),
});

export type Settings = z.infer<typeof settingsSchema>;

const defaults: Settings = {
  schemaVersion: 1,
  theme: "system",
  language: "en",
  sidebarOpen: true,
  lastRoute: "/",
  launchAtLogin: false,
  logLevel: "info",
  updateChannel: "stable",
  autoCheckUpdates: true,
  onboardingComplete: false,
  zoom: zoom.default,
  lastSeenVersion: "",
  scheduledMaintenance: false,
  dashboardOrder: ["active", "total", "archived"],
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
