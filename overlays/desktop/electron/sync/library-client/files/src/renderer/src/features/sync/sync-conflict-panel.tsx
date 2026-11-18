import { useEffect, useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import type { LibraryItem } from "@shared/ipc/library";
import type { SyncConflict } from "@shared/ipc/sync";
import { invoke } from "@renderer/lib/desktop-api";

export function SyncConflictPanel({ item }: { item: LibraryItem }) {
  const { t } = useTranslation();
  const queryClient = useQueryClient();
  const [conflict, setConflict] = useState<SyncConflict | null>(null);
  const [busy, setBusy] = useState(false);

  useEffect(() => {
    if (item.syncStatus !== "conflict") return;
    void invoke(window.desktopApi.sync.conflicts()).then((items) => {
      setConflict(items.find((value) => value.entityId === item.id) ?? null);
    }).catch(() => undefined);
  }, [item.id, item.syncStatus]);

  const active = item.syncStatus === "conflict" && conflict?.entityId === item.id ? conflict : null;
  if (!active) return null;

  const resolve = async (resolution: "local" | "server") => {
    setBusy(true);
    try {
      await invoke(window.desktopApi.sync.resolve({ id: active.id, resolution }));
      await queryClient.invalidateQueries({ queryKey: ["library"] });
      setConflict(null);
    } finally {
      setBusy(false);
    }
  };

  return (
    <div className="mb-4 rounded-lg border border-warning/40 bg-warning/10 p-3 text-sm" data-testid="sync-conflict">
      <p className="font-medium">{t("sync.conflictTitle")}</p>
      <p className="mt-1 text-xs text-muted-foreground">{t("sync.conflictBody")}</p>
      <div className="mt-3 flex gap-2">
        <Button disabled={busy} onClick={() => void resolve("local")} size="sm" variant="outline">
          {t("sync.keepLocal")}
        </Button>
        <Button disabled={busy} onClick={() => void resolve("server")} size="sm">
          {t("sync.useServer")}
        </Button>
      </div>
    </div>
  );
}
