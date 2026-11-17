import { createStore } from "zustand/vanilla";

export type AppState = {
  sidebarOpen: boolean;
  setSidebarOpen: (open: boolean) => void;
};

export function createAppStore() {
  return createStore<AppState>()((set) => ({
    sidebarOpen: false,
    setSidebarOpen: (sidebarOpen) => set({ sidebarOpen }),
  }));
}

export type AppStore = ReturnType<typeof createAppStore>;
