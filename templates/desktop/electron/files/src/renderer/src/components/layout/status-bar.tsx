import { CloudOff, Database, Wifi } from "lucide-react";
import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";

import { Progress } from "@/components/ui/progress";
import { Kbd } from "@/components/ui/kbd";
import { useActivity } from "@renderer/features/activity/activity-context";
import { useUpdates } from "@renderer/features/updates/updates-context";
import { useSystemVersion } from "@renderer/lib/query/hooks";

export function StatusBar() {
  const { t } = useTranslation();
  const [online, setOnline] = useState(navigator.onLine);
  const { data: info } = useSystemVersion();
  const { state } = useActivity();
  const updates = useUpdates();

  useEffect(() => {
    const update = () => setOnline(navigator.onLine);
    window.addEventListener("online", update);
    window.addEventListener("offline", update);

    return () => {
      window.removeEventListener("online", update);
      window.removeEventListener("offline", update);
    };
  }, []);

  return (
    <footer className="flex h-8 shrink-0 items-center gap-4 border-t border-border bg-card px-4 text-[11px] text-muted-foreground">
      <span className="inline-flex items-center gap-1.5">
        {online ? <Wifi className="size-3 text-success" /> : <CloudOff className="size-3 text-warning" />}
        {online ? t("common.online") : t("common.offline")}
      </span>
      <span className="inline-flex items-center gap-1.5">
        <Database className="size-3" />
        {t("common.sqlite")}
      </span>
      {updates.state?.status === "downloaded" ? (
        <span className="text-success">{t("common.updateReady")}</span>
      ) : updates.state?.status === "available" ? (
        <span className="text-warning">{t("common.updateAvailable")}</span>
      ) : null}
      {state?.status === "running" ? (
        <span className="inline-flex w-44 items-center gap-2">
          <Progress className="h-1.5" value={state.progress} />
          <span className="w-8 text-right">{state.progress}%</span>
        </span>
      ) : null}
      <span className="ml-auto inline-flex items-center gap-1.5">
        v{info?.version ?? "0.1.0"}
        <span className="text-border">/</span>
        <Kbd>⌘K</Kbd>
      </span>
    </footer>
  );
}
