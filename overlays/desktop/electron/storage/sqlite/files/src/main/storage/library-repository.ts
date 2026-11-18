import { randomUUID } from "node:crypto";

import { and, asc, count, desc, eq, isNull, like, or, type SQL } from "drizzle-orm";

import type {
  LibraryCreateInput,
  LibraryItem,
  LibraryListInput,
  LibraryStats,
  LibraryUpdateInput,
} from "../../shared/ipc/library";
import { databaseClient, type AppDatabase } from "./database";
import { libraryItems } from "./schema";
import { enqueueLocalSync } from "./sync-bridge";
import { loadConfig } from "../infra/config";

type Row = typeof libraryItems.$inferSelect;

function toItem(row: Row): LibraryItem {
  return {
    id: row.id,
    title: row.title,
    description: row.description,
    status: row.status,
    tags: row.tags,
    version: row.version,
    syncStatus: row.syncStatus,
    createdAt: new Date(row.createdAt).toISOString(),
    updatedAt: new Date(row.updatedAt).toISOString(),
  };
}

const seedItems = [
  ["Field notes", "Research notes for the next release.", "active", ["research", "product"]],
  ["Release checklist", "Signing, notarization, and update smoke tests.", "active", ["release"]],
  ["Offline sync", "Outbox retry and conflict policy notes.", "draft", ["sync", "draft"]],
  ["Keyboard map", "Shortcut registry and native menu parity.", "active", ["ux"]],
  ["Import format", "Portable JSON archive and per-row validation.", "draft", ["data"]],
  ["Old dashboard", "Superseded by the command-first home view.", "archived", ["design"]],
  ["Diagnostics bundle", "Logs, versions, and paths for support.", "active", ["support"]],
  ["Window chrome", "Traffic lights, overlay controls, and drag regions.", "active", ["platform"]],
  ["PKCE notes", "System browser, state validation, safeStorage.", "draft", ["auth"]],
  ["Library columns", "Title, status, and updated date.", "active", ["library"]],
  ["Update channels", "Stable and beta release channels.", "draft", ["release"]],
  ["Notification rules", "Import complete and update ready only.", "archived", ["platform"]],
] as const;

function seed(db: AppDatabase): void {
  if (loadConfig().syncEnabled) return;

  const [{ value }] = db.select({ value: count() }).from(libraryItems).all();
  if (value > 0) return;

  const now = Date.now();
  db.insert(libraryItems)
    .values(
      seedItems.map(([title, description, status, tags], index) => ({
        id: randomUUID(),
        title,
        description,
        status,
        tags: [...tags],
        createdAt: now - index * 86_400_000,
        updatedAt: now - index * 3_600_000,
      })),
    )
    .run();
}

function db(): AppDatabase {
  const client = databaseClient();
  seed(client);

  return client;
}

export function listLibraryItems(input: LibraryListInput): LibraryItem[] {
  const conditions: SQL[] = [isNull(libraryItems.deletedAt)];

  if (input.search) {
    const term = `%${input.search}%`;
    const search = or(like(libraryItems.title, term), like(libraryItems.description, term));
    if (search) conditions.push(search);
  }

  if (input.status !== "all") conditions.push(eq(libraryItems.status, input.status));

  const column =
    input.sort === "title"
      ? libraryItems.title
      : input.sort === "created"
        ? libraryItems.createdAt
        : libraryItems.updatedAt;
  const order = input.direction === "asc" ? asc(column) : desc(column);

  return db()
    .select()
    .from(libraryItems)
    .where(and(...conditions))
    .orderBy(order)
    .all()
    .map(toItem);
}

export function getLibraryItem(id: string): LibraryItem | null {
  const row = db()
    .select()
    .from(libraryItems)
    .where(and(eq(libraryItems.id, id), isNull(libraryItems.deletedAt)))
    .get();

  return row ? toItem(row) : null;
}

export function createLibraryItem(input: LibraryCreateInput): LibraryItem {
  const now = Date.now();
  const row = {
    id: randomUUID(),
    title: input.title,
    description: input.description ?? "",
    status: input.status ?? "draft",
    tags: input.tags ?? [],
    createdAt: now,
    updatedAt: now,
  };

  db().insert(libraryItems).values(row).run();
  const item = toItem({ ...row, version: 0, syncStatus: "local", ownerId: null, deletedAt: null });
  enqueueLocalSync({ operation: "create", item, ownerId: "local" });

  return item;
}

export function updateLibraryItem(input: LibraryUpdateInput): LibraryItem | null {
  const { id, ...patch } = input;
  const updatedAt = Date.now();

  db()
    .update(libraryItems)
    .set({ ...patch, updatedAt })
    .where(and(eq(libraryItems.id, id), isNull(libraryItems.deletedAt)))
    .run();
  const item = getLibraryItem(id);
  if (item) enqueueLocalSync({ operation: "update", item, ownerId: "local" });

  return item;
}

export function deleteLibraryItem(id: string): boolean {
  const current = getLibraryItem(id);
  const result = db()
    .update(libraryItems)
    .set({ deletedAt: Date.now(), updatedAt: Date.now(), syncStatus: "pending" })
    .where(and(eq(libraryItems.id, id), isNull(libraryItems.deletedAt)))
    .run();
  if (current) enqueueLocalSync({ operation: "delete", item: current, ownerId: "local" });

  return result.changes > 0;
}

export function restoreLibraryItem(id: string): LibraryItem | null {
  db()
    .update(libraryItems)
    .set({ deletedAt: null, updatedAt: Date.now(), syncStatus: "pending" })
    .where(eq(libraryItems.id, id))
    .run();
  const item = getLibraryItem(id);
  if (item) enqueueLocalSync({ operation: "update", item, ownerId: "local" });

  return item;
}

export function libraryStats(): LibraryStats {
  const client = db();
  const total = client
    .select({ value: count() })
    .from(libraryItems)
    .where(isNull(libraryItems.deletedAt))
    .get()?.value ?? 0;
  const active = client
    .select({ value: count() })
    .from(libraryItems)
    .where(and(isNull(libraryItems.deletedAt), eq(libraryItems.status, "active")))
    .get()?.value ?? 0;
  const archived = client
    .select({ value: count() })
    .from(libraryItems)
    .where(and(isNull(libraryItems.deletedAt), eq(libraryItems.status, "archived")))
    .get()?.value ?? 0;
  const recent = client
    .select()
    .from(libraryItems)
    .where(isNull(libraryItems.deletedAt))
    .orderBy(desc(libraryItems.updatedAt))
    .limit(5)
    .all()
    .map(toItem);

  return { total, active, archived, recent };
}

export function allLibraryItems(): LibraryItem[] {
  return db()
    .select()
    .from(libraryItems)
    .where(isNull(libraryItems.deletedAt))
    .orderBy(desc(libraryItems.updatedAt))
    .all()
    .map(toItem);
}

type ImportItem = Pick<LibraryItem, "title" | "description" | "status" | "tags">;

export function importLibraryItems(items: ImportItem[]): number {
  const existing = new Set(
    db().select({ title: libraryItems.title }).from(libraryItems).all().map((row) => row.title),
  );
  let imported = 0;

  for (const item of items) {
    if (existing.has(item.title)) continue;
    createLibraryItem({
      title: item.title,
      description: item.description,
      status: item.status,
      tags: item.tags,
    });
    existing.add(item.title);
    imported += 1;
  }

  return imported;
}
