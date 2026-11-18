import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";

import type { UpdateState } from "@shared/ipc/updates";
import { invoke } from "@renderer/lib/desktop-api";

type UpdatesContextValue = {
  state: UpdateState | null;
  check: () => Promise<void>;
  download: () => Promise<void>;
  install: () => Promise<void>;
};

const UpdatesContext = createContext<UpdatesContextValue | null>(null);

export function UpdatesProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<UpdateState | null>(null);

  useEffect(() => {
    void invoke(window.desktopApi.updates.get()).then(setState).catch(() => undefined);
    return window.desktopApi.updates.onEvent(setState);
  }, []);

  const value = useMemo<UpdatesContextValue>(
    () => ({
      state,
      check: async () => setState(await invoke(window.desktopApi.updates.check())),
      download: async () => setState(await invoke(window.desktopApi.updates.download())),
      install: async () => setState(await invoke(window.desktopApi.updates.install())),
    }),
    [state],
  );

  return <UpdatesContext.Provider value={value}>{children}</UpdatesContext.Provider>;
}

export function useUpdates(): UpdatesContextValue {
  const value = useContext(UpdatesContext);
  if (!value) throw new Error("useUpdates must be used inside UpdatesProvider.");

  return value;
}
