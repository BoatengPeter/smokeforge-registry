import {
  Activity,
  CircleHelp,
  House,
  Library,
  Settings,
  UserRound,
} from "lucide-react";
import { useTranslation } from "react-i18next";
import { NavLink } from "react-router";

import { Button } from "@renderer/components/ui/button";
import { useAppStore } from "@renderer/lib/stores/app-store-provider";
import { cn } from "@renderer/lib/utils";

const primary = [
  { to: "/", labelKey: "nav.home", icon: House, end: true },
  { to: "/library", labelKey: "nav.library", icon: Library },
  { to: "/activity", labelKey: "nav.activity", icon: Activity },
];

const secondary = [
  { to: "/settings", labelKey: "nav.settings", icon: Settings },
  { to: "/account", labelKey: "nav.account", icon: UserRound },
  { to: "/about", labelKey: "nav.about", icon: CircleHelp },
];

export function Sidebar() {
  const { t } = useTranslation();
  const open = useAppStore((state) => state.sidebarOpen);
  const setOpen = useAppStore((state) => state.setSidebarOpen);

  return (
    <aside
      className={cn(
        "flex shrink-0 flex-col border-r border-sidebar-border bg-sidebar py-3 text-sidebar-foreground transition-[width]",
        open ? "w-[220px]" : "w-[60px]",
      )}
    >
      <nav aria-label={t("nav.sections.workspace")} className="grid gap-1 px-2">
        {primary.map((item) => (
          <SidebarLink key={item.to} open={open} {...item} />
        ))}
      </nav>
      <div className="my-3 border-t border-sidebar-border" />
      <nav aria-label={t("nav.sections.system")} className="grid gap-1 px-2">
        {secondary.map((item) => (
          <SidebarLink key={item.to} open={open} {...item} />
        ))}
      </nav>
      <div className="mt-auto px-2">
        <Button
          className={cn("w-full justify-start text-sidebar-foreground/70 hover:bg-sidebar-accent", !open && "px-0")}
          onClick={() => setOpen(!open)}
          variant="ghost"
        >
          <span className="grid size-5 place-items-center text-xs">{open ? "‹" : "›"}</span>
          {open ? <span className="ml-2 text-xs">{t("common.collapse")}</span> : null}
        </Button>
      </div>
    </aside>
  );
}

function SidebarLink({
  to,
  labelKey,
  icon: Icon,
  end,
  open,
}: {
  to: string;
  labelKey: string;
  icon: typeof House;
  end?: boolean;
  open: boolean;
}) {
  const { t } = useTranslation();

  return (
    <NavLink
      className={({ isActive }) =>
        cn(
          "flex h-9 items-center rounded-md px-2.5 text-sm text-sidebar-foreground/70 transition-colors hover:bg-sidebar-accent hover:text-sidebar-foreground",
          isActive && "bg-sidebar-accent text-sidebar-foreground",
          !open && "justify-center px-0",
        )
      }
      end={end}
      title={t(labelKey)}
      to={to}
    >
      <Icon className="size-[18px] shrink-0" />
      {open ? <span className="ml-2.5 truncate">{t(labelKey)}</span> : null}
    </NavLink>
  );
}
