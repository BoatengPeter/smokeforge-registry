import { index, integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const libraryItems = sqliteTable(
  "library_items",
  {
    id: text("id").primaryKey(),
    title: text("title").notNull(),
    description: text("description").notNull().default(""),
    status: text("status", { enum: ["draft", "active", "archived"] }).notNull().default("draft"),
    tags: text("tags", { mode: "json" }).$type<string[]>().notNull(),
    createdAt: integer("created_at").notNull(),
    updatedAt: integer("updated_at").notNull(),
    deletedAt: integer("deleted_at"),
  },
  (table) => [
    index("library_items_status_idx").on(table.status),
    index("library_items_updated_idx").on(table.updatedAt),
  ],
);
