import type { AppStore } from "@/lib/stores/app-store";
import type { Density } from "@/types/store";

export class AppActions {
  readonly #store: AppStore;

  constructor(store: AppStore) {
    this.#store = store;
  }

  setPendingEmail(email: string | null): void {
    this.#store.setState(
      (state) => ({
        authFlow: {
          ...state.authFlow,
          pendingEmail: email,
        },
      }),
      false,
    );
  }

  clearAuthFlow(): void {
    this.#store.setState(
      { authFlow: { pendingEmail: null } },
      false,
    );
  }

  setDensity(density: Density): void {
    this.#store.setState(
      (state) => ({
        preferences: {
          ...state.preferences,
          density,
        },
      }),
      false,
    );
  }
}
