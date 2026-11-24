import { motion, useReducedMotion } from "motion/react";
import { useEffect, useMemo, useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { Outlet, useLocation, useNavigate } from "react-router";
import { toast } from "sonner";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Toaster } from "@/components/ui/sonner";
import { commandIds, type CommandId } from "@shared/commands";
import { stepZoom, zoom } from "@shared/zoom";
import { WelcomeTourProvider, useWelcomeTourState } from "@renderer/features/welcome/welcome-context";
import { WelcomeTour } from "@renderer/features/welcome/welcome-tour";
import { useAppSettings } from "@renderer/lib/app-settings";
import { useSystemVersion } from "@renderer/lib/query/hooks";
import { useAppStore } from "@renderer/lib/stores/app-store-provider";

import { CommandPalette } from "./command-palette";
import { Sidebar } from "./sidebar";
import { StatusBar } from "./status-bar";
import { TitleBar } from "./title-bar";
import { TopBar } from "./top-bar";

const routeForCommand: Partial<Record<CommandId, string>> = {
  [commandIds.home]: "/",
  [commandIds.library]: "/library",
  [commandIds.activity]: "/activity",
  [commandIds.settings]: "/settings",
  [commandIds.account]: "/account",
  [commandIds.about]: "/about",
};

/** Ctrl/Cmd + key → command. Keep in step with the accelerators in `appCommands`. */
const commandForKey: Partial<Record<string, CommandId>> = {
  k: commandIds.palette,
  b: commandIds.toggleSidebar,
  n: commandIds.newItem,
  ",": commandIds.settings,
  "=": commandIds.zoomIn,
  "+": commandIds.zoomIn,
  "-": commandIds.zoomOut,
  "0": commandIds.zoomReset,
  "1": commandIds.home,
  "2": commandIds.library,
  "3": commandIds.activity,
};

export function AppShell() {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const location = useLocation();
  const { settings, update } = useAppSettings();
  const { data: info } = useSystemVersion();
  const [paletteOpen, setPaletteOpen] = useState(false);
  const [whatsNewDismissed, setWhatsNewDismissed] = useState(false);
  const sidebarOpen = useAppStore((state) => state.sidebarOpen);
  const setSidebarOpen = useAppStore((state) => state.setSidebarOpen);
  const reducedMotion = useReducedMotion();
  const restored = useRef(false);
  const tour = useWelcomeTourState(info?.version);
  const savedRoute = useRef<string | null>(null);

  useEffect(() => {
    if (!settings || restored.current) return;
    restored.current = true;
    setSidebarOpen(settings.sidebarOpen);

    if ((window.location.hash === "" || window.location.hash === "#/") && settings.lastRoute) {
      navigate(settings.lastRoute, { replace: true });
    }
  }, [navigate, settings, setSidebarOpen]);

  useEffect(() => {
    if (!settings) return;
    const next = `${location.pathname}${location.search}`;
    if (savedRoute.current === next) return;
    savedRoute.current = next;
    const timer = window.setTimeout(() => {
      void update({ lastRoute: next });
    }, 400);

    return () => window.clearTimeout(timer);
  }, [location.pathname, location.search, settings, update]);

  const commands = useMemo(
    () => ({
      run: (id: CommandId) => {
        const route = routeForCommand[id];
        if (route) {
          navigate(route);
          return;
        }

        switch (id) {
          case commandIds.palette:
            setPaletteOpen(true);
            break;
          case commandIds.newItem:
            navigate("/library", { state: { newItem: true } });
            break;
          case commandIds.toggleSidebar:
            setSidebarOpen(!sidebarOpen);
            break;
          case commandIds.theme: {
            const next = settings?.theme === "dark" ? "light" : settings?.theme === "light" ? "system" : "dark";
            void update({ theme: next });
            break;
          }
          case commandIds.checkUpdates:
            toast.info(t("commands.updateUnavailable"));
            break;
          case commandIds.zoomIn:
            void update({ zoom: stepZoom(settings?.zoom ?? zoom.default, zoom.step) });
            break;
          case commandIds.zoomOut:
            void update({ zoom: stepZoom(settings?.zoom ?? zoom.default, -zoom.step) });
            break;
          case commandIds.zoomReset:
            void update({ zoom: zoom.default });
            break;
          case commandIds.welcome:
            tour.show();
            break;
        }
      },
    }),
    [navigate, setSidebarOpen, settings?.theme, settings?.zoom, sidebarOpen, t, tour, update],
  );

  useEffect(() => window.desktopApi.commands.on((id) => commands.run(id)), [commands]);

  useEffect(() => {
    const listener = (event: KeyboardEvent) => {
      if (!(event.metaKey || event.ctrlKey)) return;
      const key = event.key.toLowerCase();
      const id = commandForKey[key];
      if (!id) return;
      event.preventDefault();
      commands.run(id);
    };
    window.addEventListener("keydown", listener);

    return () => window.removeEventListener("keydown", listener);
  }, [commands]);

  const whatsNewOpen = Boolean(
    settings?.onboardingComplete &&
      settings.lastSeenVersion &&
      info &&
      settings.lastSeenVersion !== info.version &&
      !whatsNewDismissed,
  );

  const closeWhatsNew = () => {
    setWhatsNewDismissed(true);
    void update({ lastSeenVersion: info?.version ?? "" });
  };

  return (
    <WelcomeTourProvider tour={tour}>
      <div className="flex h-screen min-h-0 flex-col overflow-hidden bg-background">
        <a
          className="sr-only focus:not-sr-only focus:absolute focus:left-3 focus:top-3 focus:z-50 focus:rounded-md focus:bg-card focus:px-3 focus:py-2"
          href="#main-content"
        >
          {t("common.skipToContent")}
        </a>
        <TitleBar onPalette={() => setPaletteOpen(true)} />
        <div className="flex min-h-0 flex-1">
          <Sidebar />
          <div className="flex min-w-0 flex-1 flex-col">
            <TopBar />
            <main className="scrollbar-thin min-h-0 flex-1 overflow-auto" id="main-content">
              <motion.div
                animate={{ opacity: 1, y: 0 }}
                initial={reducedMotion ? false : { opacity: 0, y: 4 }}
                key={location.pathname}
                transition={{ duration: 0.16, ease: [0.16, 1, 0.3, 1] }}
              >
                <Outlet />
              </motion.div>
            </main>
            <StatusBar />
          </div>
        </div>
        <CommandPalette onOpenChange={setPaletteOpen} onRun={commands.run} open={paletteOpen} />
        <WelcomeTour appName={t("app.name")} dataPath={info?.userDataPath} version={info?.version} />
        <Dialog onOpenChange={(open) => { if (!open) closeWhatsNew(); }} open={whatsNewOpen}>
          <DialogContent>
            <DialogHeader>
              <DialogTitle>{t("whatsNew.title")}</DialogTitle>
              <DialogDescription>{t("whatsNew.body")}</DialogDescription>
            </DialogHeader>
            <DialogFooter>
              <Button onClick={closeWhatsNew}>{t("whatsNew.close")}</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <Toaster position="bottom-right" />
      </div>
    </WelcomeTourProvider>
  );
}
