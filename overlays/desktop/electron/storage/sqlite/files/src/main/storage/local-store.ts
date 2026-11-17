import type { LocalDatabase } from "./database";

export type LocalValue = unknown;

export class LocalStore {
  constructor(private readonly database: LocalDatabase) {}

  get(key: string): LocalValue | null {
    const row = this.database
      .prepare("SELECT value FROM local_kv WHERE key = ?")
      .get(key) as { value: string } | undefined;

    return row ? (JSON.parse(row.value) as LocalValue) : null;
  }

  set(key: string, value: LocalValue): void {
    this.database
      .prepare(
        `INSERT INTO local_kv (key, value, updated_at)
         VALUES (?, ?, ?)
         ON CONFLICT (key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at`,
      )
      .run(key, JSON.stringify(value), new Date().toISOString());
  }

  delete(key: string): void {
    this.database.prepare("DELETE FROM local_kv WHERE key = ?").run(key);
  }
}
