import { PanelLeft, Search } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@renderer/components/ui/button";
import { useAppStore } from "@renderer/lib/stores/app-store-provider";

export function TitleBar({ onPalette }: { onPalette: () => void }) {
  const { t } = useTranslation();
  const sidebarOpen = useAppStore((state) => state.sidebarOpen);
  const setSidebarOpen = useAppStore((state) => state.setSidebarOpen);
  const isMac = navigator.userAgent.includes("Mac");

  return (
    <header
      className={`app-region-drag flex h-[38px] shrink-0 items-center justify-between border-b border-sidebar-border bg-sidebar pl-3 pr-3 text-sidebar-foreground ${isMac ? "pl-[84px]" : ""}`}
    >
      <div className="app-region-no-drag flex items-center gap-2">
        <Button
          aria-label={t("commands.toggleSidebar")}
          className="text-sidebar-foreground hover:bg-sidebar-accent"
          onClick={() => setSidebarOpen(!sidebarOpen)}
          size="icon"
          variant="ghost"
        >
          <PanelLeft className="size-4" />
        </Button>
        <span className="hidden text-xs font-semibold tracking-[0.12em] text-sidebar-foreground/70 sm:block">
          Smokeforge
        </span>
      </div>
      <button
        className="app-region-no-drag flex h-7 w-60 items-center gap-2 rounded-md border border-sidebar-border bg-sidebar-accent/70 px-2.5 text-xs text-sidebar-foreground/60 transition-colors hover:bg-sidebar-accent"
        onClick={onPalette}
        type="button"
      >
        <Search className="size-3.5" />
        <span className="flex-1 text-left">{t("commands.placeholder")}</span>
        <kbd className="rounded border border-sidebar-border px-1.5 py-0.5 text-[10px]">⌘K</kbd>
      </button>
      <div className="w-20" />
    </header>
  );
}
