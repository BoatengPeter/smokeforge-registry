import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";

import type { ActivityState } from "@shared/ipc/activity";
import { invoke } from "@renderer/lib/desktop-api";

type ActivityContextValue = {
  state: ActivityState | null;
  start: () => Promise<void>;
  cancel: () => Promise<void>;
  retry: () => Promise<void>;
};

const ActivityContext = createContext<ActivityContextValue | null>(null);

export function ActivityProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<ActivityState | null>(null);

  useEffect(() => {
    void invoke(window.desktopApi.activity.get()).then(setState).catch(() => undefined);
    const unsubscribe = window.desktopApi.activity.onEvent(setState);

    return unsubscribe;
  }, []);

  const value = useMemo<ActivityContextValue>(
    () => ({
      state,
      start: async () => {
        setState(await invoke(window.desktopApi.activity.start()));
      },
      cancel: async () => {
        setState(await invoke(window.desktopApi.activity.cancel()));
      },
      retry: async () => {
        setState(await invoke(window.desktopApi.activity.retry()));
      },
    }),
    [state],
  );

  return <ActivityContext.Provider value={value}>{children}</ActivityContext.Provider>;
}

export function useActivity(): ActivityContextValue {
  const value = useContext(ActivityContext);
  if (!value) throw new Error("useActivity must be used inside ActivityProvider.");

  return value;
}
