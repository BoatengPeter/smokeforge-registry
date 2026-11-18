import { Copy, Download, FolderOpen } from "lucide-react";
import { useTranslation } from "react-i18next";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { useSystemVersion } from "@renderer/lib/query/hooks";
import { invoke } from "@renderer/lib/desktop-api";

export function AboutPage() {
  const { t } = useTranslation();
  const { data: info } = useSystemVersion();

  const copyDiagnostics = async () => {
    if (!info) return;
    await navigator.clipboard.writeText(JSON.stringify(info, null, 2));
    toast.success(t("common.copied"));
  };

  return (
    <div className="mx-auto grid w-full max-w-3xl gap-5 p-6">
      <Card>
        <CardHeader>
          <CardTitle>{t("about.title")}</CardTitle>
          <CardDescription>{t("about.subtitle")}</CardDescription>
        </CardHeader>
        <CardContent className="grid gap-3 text-sm">
          <Row label={t("about.application")} value={info?.version ?? "—"} />
          <Row label={t("about.electron")} value={info?.electron ?? "—"} />
          <Row label={t("about.chromium")} value={info?.chrome ?? "—"} />
          <Row label={t("about.node")} value={info?.node ?? "—"} />
          <Row label={t("about.platform")} value={`${info?.platform ?? "—"} / ${info?.arch ?? "—"}`} />
          <Separator />
          <div className="grid gap-1">
            <span className="text-muted-foreground">{t("about.paths")}</span>
            <code className="break-all rounded-md bg-muted px-2 py-1.5 text-xs">{info?.userDataPath ?? "—"}</code>
            <code className="break-all rounded-md bg-muted px-2 py-1.5 text-xs">{info?.logsPath ?? "—"}</code>
          </div>
          <div className="mt-2 flex gap-2">
            <Button onClick={() => void copyDiagnostics()} variant="outline">
              <Copy className="size-4" />
              {t("about.copyDiagnostics")}
            </Button>
            <Button
              onClick={() => {
                void invoke(window.desktopApi.system.exportDiagnostics()).then((result) => {
                  if (result) toast.success(t("about.diagnosticsExported"));
                }).catch(() => toast.error(t("common.error")));
              }}
              variant="outline"
            >
              <Download className="size-4" />
              {t("about.exportDiagnostics")}
            </Button>
            <Button onClick={() => void invoke(window.desktopApi.system.openLogs())} variant="ghost">
              <FolderOpen className="size-4" />
              {t("about.openLogs")}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}

function Row({ label, value }: { label: string; value: string }) {
  return (
    <div className="grid grid-cols-[130px_1fr] gap-3">
      <span className="text-muted-foreground">{label}</span>
      <span className="font-medium">{value}</span>
    </div>
  );
}
