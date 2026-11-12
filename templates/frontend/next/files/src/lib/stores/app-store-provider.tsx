"use client";

import { createContext, use, useMemo, useState, type ReactNode } from "react";
import { useStore } from "zustand";
import { AppActions } from "@/lib/stores/app-actions";
import { createAppStore, type AppStore } from "@/lib/stores/app-store";
import type { AppState } from "@/types/store";

const AppStoreContext = createContext<AppStore | null>(null);
const AppActionsContext = createContext<AppActions | null>(null);

export function AppStoreProvider({
  children,
  initialState,
}: {
  children: ReactNode;
  initialState?: Partial<AppState>;
}) {
  const [store] = useState(() => createAppStore(initialState));
  const actions = useMemo(() => new AppActions(store), [store]);

  return (
    <AppStoreContext value={store}>
      <AppActionsContext value={actions}>{children}</AppActionsContext>
    </AppStoreContext>
  );
}

export function useAppStore<T>(selector: (state: AppState) => T): T {
  const store = use(AppStoreContext);

  if (store === null) {
    throw new Error("useAppStore must be used within AppStoreProvider");
  }

  return useStore(store, selector);
}

export function useAppActions(): AppActions {
  const actions = use(AppActionsContext);

  if (actions === null) {
    throw new Error("useAppActions must be used within AppStoreProvider");
  }

  return actions;
}
