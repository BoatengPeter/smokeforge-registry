import { randomUUID } from "node:crypto";

import { and, asc, eq, isNull, lte, or } from "drizzle-orm";
import { loadConfig } from "../../infra/config";
import { log } from "../../infra/logger";
import { databaseClient } from "../../storage/database";
import { libraryItems, syncConflicts, syncCursor, syncOutbox } from "../../storage/schema";
import { registerLocalSyncHandler } from "../../storage/sync-bridge";
import type { LocalSyncMutation } from "../../storage/sync-bridge";

type FetchApi = (path: string, init?: RequestInit) => Promise<Response>;

const MAX_ATTEMPTS = 10;
let ownerId: string | null = null;
let fetchApi: FetchApi | null = null;
let running = false;
let timer: NodeJS.Timeout | null = null;

function enabled(): boolean {
  return loadConfig().syncEnabled;
}

function backoff(attempts: number): number {
  return Math.min(300, 2 ** Math.min(attempts, 8)) * 1000 + Math.floor(Math.random() * 500);
}

export function configureLibrarySync(userId: string, apiFetch: FetchApi): void {
  if (!enabled()) return;
  ownerId = userId;
  fetchApi = apiFetch;
  registerLocalSyncHandler(handleLocalMutation);
  if (timer) clearInterval(timer);
  timer = setInterval(() => void syncNow(), 30_000);
  timer.unref();
  void syncNow();
}

export function stopLibrarySync(): void {
  ownerId = null;
  fetchApi = null;
  registerLocalSyncHandler(null);
  if (timer) clearInterval(timer);
  timer = null;
}

async function handleLocalMutation(mutation: LocalSyncMutation): Promise<void> {
  if (!ownerId) return;
  const db = databaseClient();
  const now = Date.now();
  db.delete(syncOutbox).where(and(
    eq(syncOutbox.ownerId, ownerId),
    eq(syncOutbox.entityId, mutation.item.id),
    eq(syncOutbox.status, "pending"),
  )).run();
  db.insert(syncOutbox).values({
    id: randomUUID(),
    ownerId,
    entity: "library",
    entityId: mutation.item.id,
    operation: mutation.operation,
    payload: JSON.stringify({
      id: mutation.item.id,
      title: mutation.item.title,
      description: mutation.item.description,
      status: mutation.item.status,
      tags: mutation.item.tags,
    }),
    baseVersion: mutation.item.version,
    idempotencyKey: randomUUID(),
    status: "pending",
    attempts: 0,
    nextAttemptAt: 0,
    lastError: null,
    createdAt: now,
    updatedAt: now,
  }).run();
  db.update(libraryItems).set({ syncStatus: "pending", ownerId }).where(eq(libraryItems.id, mutation.item.id)).run();
  void syncNow();
}

export async function syncNow(): Promise<void> {
  if (!ownerId || !fetchApi || running || !enabled()) return;
  running = true;
  try {
    await pull();
    await push();
    await pull();
  } catch (error) {
    log("warn", "library sync failed", { error: error instanceof Error ? error.message : "unknown error" });
  } finally {
    running = false;
  }
}

async function pull(): Promise<void> {
  if (!ownerId || !fetchApi) return;
  const db = databaseClient();
  const cursorRow = db.select().from(syncCursor).where(eq(syncCursor.ownerId, ownerId)).get();
  const cursor = cursorRow?.cursor ?? 0;
  const response = await fetchApi(`/api/v1/library/changes?cursor=${cursor}&limit=200`);
  if (!response.ok) return;
  const body = (await response.json()) as {
    changes: Array<{ operation: string; item: Record<string, unknown> }>;
    cursor: number;
  };

  for (const change of body.changes) {
    const item = change.item as {
      id: string; title: string; description: string; status: "draft" | "active" | "archived";
      tags: string[]; version: number; updated_at: string;
    };
    const pending = db.select().from(syncOutbox).where(and(
      eq(syncOutbox.ownerId, ownerId),
      eq(syncOutbox.entityId, item.id),
      or(eq(syncOutbox.status, "pending"), eq(syncOutbox.status, "inflight")),
    )).get();
    if (pending) continue;

    const existing = db.select().from(libraryItems).where(eq(libraryItems.id, item.id)).get();
    const values = {
      id: item.id,
      title: item.title,
      description: item.description,
      status: item.status,
      tags: item.tags,
      version: item.version,
      syncStatus: "synced" as const,
      ownerId,
      createdAt: new Date(item.updated_at).getTime(),
      updatedAt: new Date(item.updated_at).getTime(),
      deletedAt: change.operation === "delete" ? Date.now() : null,
    };
    if (existing) db.update(libraryItems).set(values).where(eq(libraryItems.id, item.id)).run();
    else db.insert(libraryItems).values(values).run();
  }

  db.insert(syncCursor).values({ ownerId, cursor: body.cursor, updatedAt: Date.now() })
    .onConflictDoUpdate({ target: syncCursor.ownerId, set: { cursor: body.cursor, updatedAt: Date.now() } })
    .run();
}

async function push(): Promise<void> {
  if (!ownerId || !fetchApi) return;
  const db = databaseClient();
  const rows = db.select().from(syncOutbox).where(and(
    eq(syncOutbox.ownerId, ownerId),
    or(eq(syncOutbox.status, "pending"), eq(syncOutbox.status, "failed")),
    lte(syncOutbox.nextAttemptAt, Date.now()),
  )).orderBy(asc(syncOutbox.createdAt)).all();

  for (const row of rows) {
    if (row.attempts >= MAX_ATTEMPTS) {
      db.update(syncOutbox).set({ status: "failed", updatedAt: Date.now() }).where(eq(syncOutbox.id, row.id)).run();
      continue;
    }
    let response: Response;
    try {
      response = await send(row);
    } catch (error) {
      const attempts = row.attempts + 1;
      db.update(syncOutbox).set({
        status: "failed",
        attempts,
        nextAttemptAt: Date.now() + backoff(attempts),
        lastError: error instanceof Error ? error.message : "network error",
        updatedAt: Date.now(),
      }).where(eq(syncOutbox.id, row.id)).run();
      continue;
    }
    if (response.ok) {
      const body = (await response.json().catch(() => null)) as { item?: { version: number } } | null;
      if (body?.item && row.operation !== "delete") {
        db.update(libraryItems).set({
          version: body.item.version,
          syncStatus: "synced",
        }).where(eq(libraryItems.id, row.entityId)).run();
      } else {
        db.update(libraryItems).set({ syncStatus: "synced" }).where(eq(libraryItems.id, row.entityId)).run();
      }
      db.delete(syncOutbox).where(eq(syncOutbox.id, row.id)).run();
      continue;
    }

    if (response.status === 409) {
      const remote = await fetchRemote(row.entityId);
      db.update(syncOutbox).set({ status: "conflict", lastError: "version_conflict", updatedAt: Date.now() }).where(eq(syncOutbox.id, row.id)).run();
      db.update(libraryItems).set({ syncStatus: "conflict" }).where(eq(libraryItems.id, row.entityId)).run();
      db.insert(syncConflicts).values({
        id: randomUUID(),
        ownerId,
        entityId: row.entityId,
        localPayload: row.payload,
        remotePayload: JSON.stringify(remote ?? {}),
        createdAt: Date.now(),
      }).run();
      continue;
    }

    const retryable = response.status === 408 || response.status === 429 || response.status >= 500;
    const attempts = row.attempts + 1;
    const retryAfter = Number(response.headers.get("retry-after"));
    const delay = Number.isFinite(retryAfter) && retryAfter > 0
      ? retryAfter * 1000
      : backoff(attempts);
    db.update(syncOutbox).set({
      status: retryable ? "failed" : "failed",
      attempts,
      nextAttemptAt: retryable ? Date.now() + delay : Date.now() + 86_400_000,
      lastError: `HTTP ${response.status}`,
      updatedAt: Date.now(),
    }).where(eq(syncOutbox.id, row.id)).run();
  }
}

async function fetchRemote(id: string): Promise<unknown> {
  if (!fetchApi) return null;
  const response = await fetchApi(`/api/v1/library/${id}`);
  return response.ok ? response.json() : null;
}

async function send(row: typeof syncOutbox.$inferSelect): Promise<Response> {
  if (!fetchApi) throw new Error("sync transport unavailable");
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
    "Idempotency-Key": row.idempotencyKey,
  };
  if (row.operation === "create") {
    return fetchApi("/api/v1/library", { method: "POST", headers, body: row.payload });
  }
  headers["If-Match"] = String(row.baseVersion);
  if (row.operation === "update") {
    return fetchApi(`/api/v1/library/${row.entityId}`, { method: "PATCH", headers, body: row.payload });
  }
  return fetchApi(`/api/v1/library/${row.entityId}`, { method: "DELETE", headers });
}

export function pendingConflictCount(): number {
  if (!ownerId) return 0;
  const db = databaseClient();
  return db.select().from(syncConflicts).where(and(eq(syncConflicts.ownerId, ownerId), isNull(syncConflicts.resolvedAt))).all().length;
}

export function getSyncStatus(): { pending: number; conflicts: number; running: boolean } {
  if (!ownerId) return { pending: 0, conflicts: 0, running: false };
  const db = databaseClient();
  const pending = db.select().from(syncOutbox).where(and(eq(syncOutbox.ownerId, ownerId), or(eq(syncOutbox.status, "pending"), eq(syncOutbox.status, "inflight"), eq(syncOutbox.status, "failed")))).all().length;
  return {
    pending,
    conflicts: pendingConflictCount(),
    running,
  };
}

export function listSyncConflicts() {
  if (!ownerId) return [];
  const db = databaseClient();
  return db.select().from(syncConflicts).where(and(eq(syncConflicts.ownerId, ownerId), isNull(syncConflicts.resolvedAt))).all().map((row) => ({
    id: row.id,
    entityId: row.entityId,
    localPayload: JSON.parse(row.localPayload) as unknown,
    remotePayload: JSON.parse(row.remotePayload) as unknown,
    createdAt: new Date(row.createdAt).toISOString(),
  }));
}

export async function resolveSyncConflict(conflictId: string, resolution: "local" | "server"): Promise<void> {
  if (!ownerId) return;
  const db = databaseClient();
  const conflict = db.select().from(syncConflicts).where(and(eq(syncConflicts.id, conflictId), eq(syncConflicts.ownerId, ownerId))).get();
  if (!conflict) return;
  const remote = JSON.parse(conflict.remotePayload) as { id?: string; version?: number; title?: string; description?: string; status?: "draft" | "active" | "archived"; tags?: string[]; updated_at?: string };
  const outbox = db.select().from(syncOutbox).where(and(eq(syncOutbox.ownerId, ownerId), eq(syncOutbox.entityId, conflict.entityId))).get();
  if (outbox) {
    if (resolution === "local") {
      db.update(syncOutbox).set({
        status: "pending",
        baseVersion: remote.version ?? outbox.baseVersion,
        idempotencyKey: randomUUID(),
        attempts: 0,
        nextAttemptAt: 0,
        lastError: null,
        updatedAt: Date.now(),
      }).where(eq(syncOutbox.id, outbox.id)).run();
      db.update(libraryItems).set({ syncStatus: "pending" }).where(eq(libraryItems.id, conflict.entityId)).run();
    } else {
      db.delete(syncOutbox).where(eq(syncOutbox.id, outbox.id)).run();
      if (remote.id) {
        db.update(libraryItems).set({
          title: remote.title ?? "",
          description: remote.description ?? "",
          status: remote.status ?? "draft",
          tags: remote.tags ?? [],
          version: remote.version ?? 0,
          syncStatus: "synced",
        }).where(eq(libraryItems.id, conflict.entityId)).run();
      }
    }
  }
  db.update(syncConflicts).set({ resolvedAt: Date.now() }).where(eq(syncConflicts.id, conflict.id)).run();
  if (resolution === "local") await syncNow();
}
