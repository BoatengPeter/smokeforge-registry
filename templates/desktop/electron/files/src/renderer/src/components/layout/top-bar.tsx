import { Plus } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useLocation, useNavigate } from "react-router";

import { Button } from "@/components/ui/button";

const titles: Record<string, { title: string; subtitle: string }> = {
  "/": { title: "dashboard.title", subtitle: "dashboard.subtitle" },
  "/library": { title: "library.title", subtitle: "library.subtitle" },
  "/activity": { title: "activity.title", subtitle: "activity.subtitle" },
  "/settings": { title: "settings.title", subtitle: "settings.subtitle" },
  "/account": { title: "account.title", subtitle: "account.subtitle" },
  "/about": { title: "about.title", subtitle: "about.subtitle" },
};

export function TopBar() {
  const { t } = useTranslation();
  const location = useLocation();
  const navigate = useNavigate();
  const current = titles[location.pathname] ?? titles["/"];

  return (
    <div className="flex shrink-0 items-center justify-between gap-4 border-b border-border bg-background/95 px-6 py-4">
      <div>
        <h1 className="text-lg font-semibold tracking-tight">{t(current.title)}</h1>
        <p className="mt-0.5 text-xs text-muted-foreground">{t(current.subtitle)}</p>
      </div>
      {location.pathname === "/library" ? (
        <Button onClick={() => navigate("/library", { state: { newItem: true } })}>
          <Plus className="size-4" />
          {t("library.newItem")}
        </Button>
      ) : null}
    </div>
  );
}
