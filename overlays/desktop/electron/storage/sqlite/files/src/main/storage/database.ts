import { mkdirSync } from "node:fs";
import { dirname } from "node:path";
import { join } from "node:path";

import Database from "better-sqlite3";
import { app } from "electron";
import { drizzle, type BetterSQLite3Database } from "drizzle-orm/better-sqlite3";
import { migrate } from "drizzle-orm/better-sqlite3/migrator";

import { userDataPath } from "../infra/paths";
import * as schema from "./schema";

export type LocalDatabase = Database.Database;
export type AppDatabase = BetterSQLite3Database<typeof schema>;

const migrations = [
  {
    version: 1,
    sql: `CREATE TABLE IF NOT EXISTS local_kv (
      key TEXT PRIMARY KEY NOT NULL,
      value TEXT NOT NULL,
      updated_at TEXT NOT NULL
    );`,
  },
];

let database: LocalDatabase | null = null;
let client: AppDatabase | null = null;

export function openLocalDatabase(path = userDataPath("local.sqlite")): LocalDatabase {
  if (database) return database;

  mkdirSync(dirname(path), { recursive: true, mode: 0o700 });

  const next = new Database(path);
  next.pragma("journal_mode = WAL");
  next.pragma("foreign_keys = ON");
  next.pragma("busy_timeout = 5000");

  const current = next.pragma("user_version", { simple: true }) as number;

  for (const migration of migrations) {
    if (migration.version <= current) continue;

    next.transaction(() => {
      next.exec(migration.sql);
      next.pragma(`user_version = ${migration.version}`);
    })();
  }

  database = next;

  return database;
}

export function databaseClient(): AppDatabase {
  if (client) return client;

  const sqlite = openLocalDatabase();
  client = drizzle(sqlite, { schema });
  const folder = app.isPackaged
    ? join(process.resourcesPath, "drizzle")
    : join(app.getAppPath(), "drizzle");
  migrate(client, { migrationsFolder: folder });

  return client;
}
