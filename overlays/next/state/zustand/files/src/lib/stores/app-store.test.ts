import { AppActions } from "@/lib/stores/app-actions";
import { createAppStore } from "@/lib/stores/app-store";
import { selectDensity, selectPendingEmail } from "@/lib/stores/app-selectors";

describe("app store", () => {
  it("updates auth flow state through actions", () => {
    const store = createAppStore();
    const actions = new AppActions(store);

    actions.setPendingEmail("user@example.com");

    expect(selectPendingEmail(store.getState())).toBe("user@example.com");

    actions.clearAuthFlow();

    expect(selectPendingEmail(store.getState())).toBeNull();
  });

  it("updates preferences through actions", () => {
    const store = createAppStore();
    const actions = new AppActions(store);

    actions.setDensity("compact");

    expect(selectDensity(store.getState())).toBe("compact");
  });

  it("creates a fresh store per call", () => {
    const first = createAppStore();
    const second = createAppStore();

    new AppActions(first).setPendingEmail("first@example.com");

    expect(selectPendingEmail(first.getState())).toBe("first@example.com");
    expect(selectPendingEmail(second.getState())).toBeNull();
  });
});
