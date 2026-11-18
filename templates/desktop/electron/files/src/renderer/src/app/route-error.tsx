import { AlertTriangle } from "lucide-react";
import { useTranslation } from "react-i18next";
import { isRouteErrorResponse, useRouteError } from "react-router";

import { Button } from "@/components/ui/button";

export function RouteError() {
  const { t } = useTranslation();
  const error = useRouteError();
  const message = isRouteErrorResponse(error)
    ? `${error.status} ${error.statusText}`
    : error instanceof Error
      ? error.message
      : t("common.error");

  return (
    <main className="grid min-h-screen place-items-center bg-background p-6">
      <div className="max-w-md rounded-xl border border-border bg-card p-6 text-center">
        <AlertTriangle className="mx-auto size-8 text-warning" />
        <h1 className="mt-4 text-lg font-semibold">{t("errors.routeTitle")}</h1>
        <p className="mt-2 text-sm text-muted-foreground">{t("errors.routeBody")}</p>
        <code className="mt-3 block overflow-auto rounded-md bg-muted p-2 text-xs">{message}</code>
        <Button className="mt-4" onClick={() => { window.location.hash = "#/"; }}>
          {t("errors.home")}
        </Button>
      </div>
    </main>
  );
}
