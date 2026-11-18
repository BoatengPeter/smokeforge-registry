import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";

import type { AppSettings, SettingsPatch } from "@shared/ipc/settings";
import { invoke } from "@renderer/lib/desktop-api";

type SettingsContextValue = {
  settings: AppSettings | null;
  update: (patch: SettingsPatch) => Promise<void>;
  reload: () => Promise<void>;
};

const SettingsContext = createContext<SettingsContextValue | null>(null);

function applyTheme(theme: AppSettings["theme"]): void {
  const media = window.matchMedia("(prefers-color-scheme: dark)");
  const dark = theme === "dark" || (theme === "system" && media.matches);
  document.documentElement.classList.toggle("dark", dark);
}

export function AppSettingsProvider({ children }: { children: ReactNode }) {
  const [settings, setSettings] = useState<AppSettings | null>(null);

  const reload = useCallback(async () => {
    try {
      const next = await invoke(window.desktopApi.settings.get());
      setSettings(next);
      applyTheme(next.theme);
      document.documentElement.lang = next.language;
    } catch {
      // The shell still renders with defaults if settings cannot be read.
    }
  }, []);

  const update = useCallback(async (patch: SettingsPatch) => {
    const next = await invoke(window.desktopApi.settings.update(patch));
    setSettings(next);
    applyTheme(next.theme);
  }, []);

  useEffect(() => {
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void reload();
  }, [reload]);

  useEffect(() => {
    if (settings?.theme !== "system") return;

    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const listener = () => applyTheme("system");
    media.addEventListener("change", listener);

    return () => media.removeEventListener("change", listener);
  }, [settings?.theme]);

  const value = useMemo(() => ({ settings, update, reload }), [settings, update, reload]);

  return <SettingsContext.Provider value={value}>{children}</SettingsContext.Provider>;
}

export function useAppSettings(): SettingsContextValue {
  const value = useContext(SettingsContext);
  if (!value) throw new Error("useAppSettings must be used inside AppSettingsProvider.");

  return value;
}
