import { createHashRouter } from "react-router";

import { AppShell } from "@renderer/components/layout/app-shell";
import { AboutPage } from "@renderer/features/about/about-page";
import { AccountPage } from "@renderer/features/account/account-page";
import { ActivityPage } from "@renderer/features/activity/activity-page";
import { DashboardPage } from "@renderer/features/dashboard/dashboard-page";
import { LibraryPage } from "@renderer/features/library/library-page";
import { SettingsPage } from "@renderer/features/settings/settings-page";
import { RouteError } from "@renderer/app/route-error";
import { NotFoundPage } from "@renderer/features/not-found-page";

export const router = createHashRouter([
  {
    path: "/",
    Component: AppShell,
    ErrorBoundary: RouteError,
    children: [
      { index: true, Component: DashboardPage },
      { path: "library", Component: LibraryPage },
      { path: "library/:itemId", Component: LibraryPage },
      { path: "activity", Component: ActivityPage },
      { path: "settings", Component: SettingsPage },
      { path: "account", Component: AccountPage },
      { path: "about", Component: AboutPage },
      { path: "*", Component: NotFoundPage },
    ],
  },
]);
