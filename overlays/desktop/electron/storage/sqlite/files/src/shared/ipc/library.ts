import { z } from "zod";

import type { Result } from "../types/result";

export const libraryStatuses = ["draft", "active", "archived"] as const;
export const libraryStatusSchema = z.enum(libraryStatuses);
export type LibraryStatus = z.infer<typeof libraryStatusSchema>;

export const libraryItemSchema = z.object({
  id: z.string(),
  title: z.string(),
  description: z.string(),
  status: libraryStatusSchema,
  tags: z.array(z.string()),
  createdAt: z.string(),
  updatedAt: z.string(),
});

export type LibraryItem = z.infer<typeof libraryItemSchema>;

export const libraryChannels = {
  list: "library:list",
  get: "library:get",
  create: "library:create",
  update: "library:update",
  delete: "library:delete",
  restore: "library:restore",
  stats: "library:stats",
  exportAll: "library:export",
  importAll: "library:import",
  importPath: "library:import-path",
  backup: "library:backup",
  maintenance: "library:maintenance",
  changed: "library:changed",
} as const;

const tagsSchema = z
  .array(z.string().trim().min(1).max(32))
  .max(12)
  .default([])
  .transform((tags) => Array.from(new Set(tags)));

export const librarySchemas = {
  list: z
    .object({
      search: z.string().trim().max(200).default(""),
      status: z.enum(["all", ...libraryStatuses]).default("all"),
      sort: z.enum(["updated", "created", "title"]).default("updated"),
      direction: z.enum(["asc", "desc"]).default("desc"),
    })
    .strict(),
  get: z.object({ id: z.string().uuid() }).strict(),
  create: z
    .object({
      title: z.string().trim().min(1).max(160),
      description: z.string().trim().max(5000).default(""),
      status: libraryStatusSchema.default("draft"),
      tags: tagsSchema,
    })
    .strict(),
  update: z
    .object({
      id: z.string().uuid(),
      title: z.string().trim().min(1).max(160).optional(),
      description: z.string().trim().max(5000).optional(),
      status: libraryStatusSchema.optional(),
      tags: z
        .array(z.string().trim().min(1).max(32))
        .max(12)
        .transform((tags) => Array.from(new Set(tags)))
        .optional(),
    })
    .strict()
    .refine((value) => Object.keys(value).some((key) => key !== "id"), "No changes supplied."),
  delete: z.object({ id: z.string().uuid() }).strict(),
  restore: z.object({ id: z.string().uuid() }).strict(),
  stats: z.void(),
  exportAll: z.void(),
  importAll: z.void(),
  importPath: z.object({ path: z.string().trim().min(1).max(4096) }).strict(),
  backup: z.void(),
  maintenance: z.object({ enabled: z.boolean() }).strict(),
};

export type LibraryListInput = z.infer<typeof librarySchemas.list>;
export type LibraryCreateInput = z.infer<typeof librarySchemas.create>;
export type LibraryUpdateInput = z.infer<typeof librarySchemas.update>;

export type LibraryStats = {
  total: number;
  active: number;
  archived: number;
  recent: LibraryItem[];
};

export type LibraryApi = {
  list: (input: LibraryListInput) => Promise<Result<LibraryItem[]>>;
  get: (id: string) => Promise<Result<LibraryItem | null>>;
  create: (input: LibraryCreateInput) => Promise<Result<LibraryItem>>;
  update: (input: LibraryUpdateInput) => Promise<Result<LibraryItem | null>>;
  delete: (id: string) => Promise<Result<{ deleted: boolean }>>;
  restore: (id: string) => Promise<Result<LibraryItem | null>>;
  stats: () => Promise<Result<LibraryStats>>;
  exportAll: () => Promise<Result<{ path: string; count: number } | null>>;
  importAll: () => Promise<Result<{ imported: number; skipped: number } | null>>;
  importPath: (path: string) => Promise<Result<{ imported: number; skipped: number }>>;
  backup: () => Promise<Result<{ path: string } | null>>;
  setMaintenance: (enabled: boolean) => Promise<Result<{ enabled: boolean }>>;
  onChanged: (listener: () => void) => () => void;
};

export const libraryImportSchema = z.object({
  items: z
    .array(
      z.object({
        title: z.string().trim().min(1).max(160),
        description: z.string().trim().max(5000).default(""),
        status: libraryStatusSchema.default("draft"),
        tags: z.array(z.string().trim().min(1).max(32)).max(12).default([]),
      }),
    )
    .max(5000),
});
