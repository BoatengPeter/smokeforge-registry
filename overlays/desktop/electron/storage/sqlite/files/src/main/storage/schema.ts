import { index, integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const libraryItems = sqliteTable(
  "library_items",
  {
    id: text("id").primaryKey(),
    title: text("title").notNull(),
    description: text("description").notNull().default(""),
    status: text("status", { enum: ["draft", "active", "archived"] }).notNull().default("draft"),
    tags: text("tags", { mode: "json" }).$type<string[]>().notNull(),
    version: integer("version").notNull().default(0),
    syncStatus: text("sync_status", { enum: ["local", "pending", "synced", "conflict", "failed"] })
      .notNull()
      .default("local"),
    ownerId: text("owner_id"),
    createdAt: integer("created_at").notNull(),
    updatedAt: integer("updated_at").notNull(),
    deletedAt: integer("deleted_at"),
  },
  (table) => [
    index("library_items_status_idx").on(table.status),
    index("library_items_updated_idx").on(table.updatedAt),
  ],
);

export const syncOutbox = sqliteTable(
  "sync_outbox",
  {
    id: text("id").primaryKey(),
    ownerId: text("owner_id").notNull(),
    entity: text("entity").notNull(),
    entityId: text("entity_id").notNull(),
    operation: text("operation", { enum: ["create", "update", "delete"] }).notNull(),
    payload: text("payload").notNull(),
    baseVersion: integer("base_version").notNull(),
    idempotencyKey: text("idempotency_key").notNull(),
    status: text("status", { enum: ["pending", "inflight", "failed", "conflict"] })
      .notNull()
      .default("pending"),
    attempts: integer("attempts").notNull().default(0),
    nextAttemptAt: integer("next_attempt_at").notNull().default(0),
    lastError: text("last_error"),
    createdAt: integer("created_at").notNull(),
    updatedAt: integer("updated_at").notNull(),
  },
  (table) => [index("sync_outbox_owner_status_idx").on(table.ownerId, table.status)],
);

export const syncCursor = sqliteTable("sync_cursor", {
  ownerId: text("owner_id").primaryKey(),
  cursor: integer("cursor").notNull().default(0),
  updatedAt: integer("updated_at").notNull(),
});

export const syncConflicts = sqliteTable(
  "sync_conflicts",
  {
    id: text("id").primaryKey(),
    ownerId: text("owner_id").notNull(),
    entityId: text("entity_id").notNull(),
    localPayload: text("local_payload").notNull(),
    remotePayload: text("remote_payload").notNull(),
    createdAt: integer("created_at").notNull(),
    resolvedAt: integer("resolved_at"),
  },
  (table) => [index("sync_conflicts_owner_idx").on(table.ownerId)],
);
