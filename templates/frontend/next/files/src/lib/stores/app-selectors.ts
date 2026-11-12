import type { AppState } from "@/types/store";

export const selectPendingEmail = (state: AppState) => state.authFlow.pendingEmail;

export const selectDensity = (state: AppState) => state.preferences.density;
