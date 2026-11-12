import { createStore, type StoreApi } from "zustand";
import { devtools, persist, subscribeWithSelector } from "zustand/middleware";
import type { AppState } from "@/types/store";

export type AppStore = StoreApi<AppState>;

export function createInitialAppState(overrides?: Partial<AppState>): AppState {
  return {
    preferences: {
      density: "comfortable",
      ...overrides?.preferences,
    },
    authFlow: {
      pendingEmail: null,
      ...overrides?.authFlow,
    },
  };
}

export function createAppStore(overrides?: Partial<AppState>): AppStore {
  return createStore<AppState>()(
    devtools(
      persist(
        subscribeWithSelector(() => createInitialAppState(overrides)),
        {
          name: "smokeforge-preferences",
          partialize: (state) => ({ preferences: state.preferences }),
        },
      ),
      {
        name: "smokeforge-store",
        enabled: process.env.NODE_ENV !== "production",
      },
    ),
  );
}
