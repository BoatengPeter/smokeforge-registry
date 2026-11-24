import { createContext, useCallback, useContext, useMemo, useState, type ReactNode } from "react";

import { useAppSettings } from "@renderer/lib/app-settings";

import type { WelcomePage } from "./welcome-pages";

/**
 * Whether the welcome tour is open, and on which page.
 *
 * It opens by itself on first launch (`settings.onboardingComplete` is false)
 * and from Settings › Help or the "Show welcome tour" command. Closing it any
 * way sets `onboardingComplete` and records the version, so What's new starts
 * from here. The shell owns the state (`useWelcomeTourState`) and provides it;
 * pages read it with `useWelcomeTour`.
 */
export type WelcomeTour = {
  /** The page on screen, or `null` when the tour is closed. */
  page: WelcomePage | null;
  show: () => void;
  go: (page: WelcomePage) => void;
  close: () => void;
};

const WelcomeTourContext = createContext<WelcomeTour | null>(null);

export function useWelcomeTourState(appVersion: string | undefined): WelcomeTour {
  const { settings, update } = useAppSettings();
  const [chosen, setChosen] = useState<WelcomePage | null>(null);
  const [closed, setClosed] = useState(false);
  const firstRun = Boolean(settings && !settings.onboardingComplete && !closed);
  const page = chosen ?? (firstRun ? "intro" : null);

  const show = useCallback(() => {
    setClosed(false);
    setChosen("intro");
  }, []);
  const close = useCallback(() => {
    setChosen(null);
    setClosed(true);
    void update({ onboardingComplete: true, lastSeenVersion: appVersion ?? "" });
  }, [appVersion, update]);

  return useMemo(() => ({ page, show, go: setChosen, close }), [page, show, close]);
}

export function WelcomeTourProvider({ tour, children }: { tour: WelcomeTour; children: ReactNode }) {
  return <WelcomeTourContext.Provider value={tour}>{children}</WelcomeTourContext.Provider>;
}

export function useWelcomeTour(): WelcomeTour {
  const tour = useContext(WelcomeTourContext);
  if (!tour) throw new Error("useWelcomeTour must be used inside WelcomeTourProvider.");

  return tour;
}
