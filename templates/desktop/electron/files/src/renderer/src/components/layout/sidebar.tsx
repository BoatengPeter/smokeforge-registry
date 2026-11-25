import { Activity, CircleHelp, House, Library, PanelLeft, Plus, Search, Settings, UserRound } from "lucide-react";
import { useTranslation } from "react-i18next";
import { NavLink, useNavigate } from "react-router";

import { Button } from "@renderer/components/ui/button";
import logo from "@renderer/assets/logo.png";
import { useAppStore } from "@renderer/lib/stores/app-store-provider";
import { cn } from "@renderer/lib/utils";

const isMac = navigator.userAgent.includes("Mac");

const workspace = [
  { to: "/", labelKey: "nav.home", icon: House, end: true },
  { to: "/library", labelKey: "nav.library", icon: Library },
  { to: "/activity", labelKey: "nav.activity", icon: Activity },
  { to: "/account", labelKey: "nav.account", icon: UserRound },
];

const footer = [
  { to: "/settings", labelKey: "nav.settings", icon: Settings },
  { to: "/about", labelKey: "nav.about", icon: CircleHelp },
];

/**
 * Full-height sidebar: a header (logo, name, search, collapse) level with the
 * title bar, the workspace pages under a label with a "+", and Settings /
 * About in the footer. Collapsed, it is a 52 px icon rail with tooltips.
 * The header drags the window; its buttons opt out with `app-region-no-drag`.
 */
export function Sidebar({ onSearch }: { onSearch: () => void }) {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const open = useAppStore((state) => state.sidebarOpen);
  const setOpen = useAppStore((state) => state.setSidebarOpen);
  const toggle = (
    <IconButton label={open ? t("shell.collapse") : t("shell.expand")} onClick={() => setOpen(!open)}>
      <PanelLeft />
    </IconButton>
  );

  return (
    <aside
      className={cn(
        "flex shrink-0 flex-col border-r border-sidebar-border bg-sidebar text-sidebar-foreground transition-[width]",
        open ? "w-[268px]" : "w-[52px]",
      )}
    >
      <div
        className={cn(
          "app-region-drag flex h-11 shrink-0 items-center gap-1",
          open ? "pr-2.5 pl-3.5" : "justify-center",
          open && isMac && "pl-[84px]",
        )}
      >
        {open ? (
          <>
            <img alt="" className="size-5 rounded-[5px]" draggable={false} src={logo} />
            <span className="ml-1.5 min-w-0 flex-1 truncate text-[13.5px] font-semibold">{t("app.name")}</span>
            <IconButton label={t("shell.search")} onClick={onSearch}>
              <Search />
            </IconButton>
            {toggle}
          </>
        ) : isMac ? null : (
          toggle
        )}
      </div>

      {open ? (
        <div className="flex h-8 items-center pr-2.5 pl-5 text-[13px] text-sidebar-foreground/60">
          <span className="flex-1">{t("nav.sections.workspace")}</span>
          <IconButton label={t("commands.newItem")} onClick={() => navigate("/library", { state: { newItem: true } })}>
            <Plus />
          </IconButton>
        </div>
      ) : isMac ? (
        <div className="flex justify-center py-1">{toggle}</div>
      ) : null}

      <nav aria-label={t("nav.sections.workspace")} className="grid gap-0.5 px-2">
        {workspace.map((item) => (
          <SidebarLink key={item.to} open={open} {...item} />
        ))}
      </nav>

      <nav
        aria-label={t("nav.sections.system")}
        className="mt-auto grid gap-0.5 border-t border-sidebar-border px-2 pt-1 pb-2"
      >
        {footer.map((item) => (
          <SidebarLink key={item.to} open={open} {...item} />
        ))}
      </nav>
    </aside>
  );
}

function IconButton({ label, onClick, children }: { label: string; onClick: () => void; children: React.ReactNode }) {
  return (
    <Button
      aria-label={label}
      className="app-region-no-drag size-[26px] rounded-md text-sidebar-foreground/60 hover:bg-sidebar-accent hover:text-sidebar-foreground [&_svg]:size-[15px]"
      onClick={onClick}
      size="icon"
      title={label}
      variant="ghost"
    >
      {children}
    </Button>
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
          "flex h-8 items-center gap-2.5 rounded-lg px-2.5 text-sm text-sidebar-foreground/70 transition-colors hover:bg-sidebar-accent hover:text-sidebar-foreground",
          isActive && "bg-sidebar-accent text-sidebar-foreground",
          !open && "justify-center px-0",
        )
      }
      end={end}
      title={t(labelKey)}
      to={to}
    >
      <Icon className="size-4 shrink-0" />
      {open ? <span className="truncate">{t(labelKey)}</span> : null}
    </NavLink>
  );
}
