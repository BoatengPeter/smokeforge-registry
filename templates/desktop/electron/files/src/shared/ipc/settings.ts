import { z } from "zod";

import { zoom } from "../zoom";

export const settingsChannels = {
  get: "settings:get",
  update: "settings:update",
} as const;

export const settingsPatchSchema = z
  .object({
    theme: z.enum(["system", "light", "dark"]).optional(),
    language: z.string().trim().min(2).max(32).optional(),
    sidebarOpen: z.boolean().optional(),
    lastRoute: z.string().trim().max(512).optional(),
    launchAtLogin: z.boolean().optional(),
    logLevel: z.enum(["debug", "info", "warn", "error"]).optional(),
    updateChannel: z.enum(["stable", "beta"]).optional(),
    autoCheckUpdates: z.boolean().optional(),
    onboardingComplete: z.boolean().optional(),
    zoom: z.number().min(zoom.min).max(zoom.max).optional(),
    lastSeenVersion: z.string().trim().max(64).optional(),
    scheduledMaintenance: z.boolean().optional(),
    dashboardOrder: z
      .array(z.enum(["active", "total", "archived"]))
      .length(3)
      .refine((items) => new Set(items).size === 3, "Dashboard order must contain each card once.")
      .optional(),
  })
  .strict();

export const settingsSchemas = {
  get: z.void(),
  update: settingsPatchSchema,
} as const;

export type SettingsPatch = z.infer<typeof settingsPatchSchema>;

export type AppSettings = SettingsPatch & {
  schemaVersion: 1;
  theme: "system" | "light" | "dark";
  language: string;
  sidebarOpen: boolean;
  lastRoute: string;
  launchAtLogin: boolean;
  logLevel: "debug" | "info" | "warn" | "error";
  updateChannel: "stable" | "beta";
  autoCheckUpdates: boolean;
  onboardingComplete: boolean;
  zoom: number;
  lastSeenVersion: string;
  dashboardOrder: ("active" | "total" | "archived")[];
  scheduledMaintenance: boolean;
};
