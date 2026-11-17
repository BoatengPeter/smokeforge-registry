import { mkdirSync } from "node:fs";
import { dirname } from "node:path";

import Database from "better-sqlite3";

import { userDataPath } from "../infra/paths";

export type LocalDatabase = Database.Database;

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

export function openLocalDatabase(path = userDataPath("local.sqlite")): LocalDatabase {
  mkdirSync(dirname(path), { recursive: true, mode: 0o700 });

  const database = new Database(path);
  database.pragma("journal_mode = WAL");
  database.pragma("foreign_keys = ON");
  database.pragma("busy_timeout = 5000");

  const current = database.pragma("user_version", { simple: true }) as number;

  for (const migration of migrations) {
    if (migration.version <= current) continue;

    database.transaction(() => {
      database.exec(migration.sql);
      database.pragma(`user_version = ${migration.version}`);
    })();
  }

  return database;
}
