import type { LibraryItem } from "../../shared/ipc/library";

export type LocalSyncMutation = {
  operation: "create" | "update" | "delete";
  item: LibraryItem;
  ownerId: string;
};

type Handler = (mutation: LocalSyncMutation) => Promise<void>;

let handler: Handler | null = null;

export function registerLocalSyncHandler(next: Handler | null): void {
  handler = next;
}

export function enqueueLocalSync(mutation: LocalSyncMutation): void {
  if (!handler) return;
  void handler(mutation);
}
