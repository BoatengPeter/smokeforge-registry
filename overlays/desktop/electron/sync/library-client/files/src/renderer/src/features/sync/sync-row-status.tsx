import { useTranslation } from "react-i18next";

import { Badge } from "@/components/ui/badge";
import type { LibraryItem } from "@shared/ipc/library";

export function SyncRowStatus({ item }: { item: LibraryItem }) {
  const { t } = useTranslation();
  if (item.syncStatus === "synced") return null;

  return (
    <Badge className="ml-2 capitalize" variant={item.syncStatus === "conflict" ? "destructive" : "secondary"}>
      {t(`sync.status.${item.syncStatus}`)}
    </Badge>
  );
}
