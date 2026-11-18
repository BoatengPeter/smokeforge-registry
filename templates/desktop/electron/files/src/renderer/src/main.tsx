import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { App } from "./app/app";
import { Providers } from "./app/providers";
import { ActivityProvider } from "./features/activity/activity-context";
import { UpdatesProvider } from "./features/updates/updates-context";
import { AppSettingsProvider } from "./lib/app-settings";
import "./lib/i18n";
import "./styles/globals.css";

const root = document.getElementById("root");

if (!root) {
  throw new Error("Renderer root element is missing.");
}

createRoot(root).render(
  <StrictMode>
    <Providers>
      <AppSettingsProvider>
        <ActivityProvider>
          <UpdatesProvider>
            <App />
          </UpdatesProvider>
        </ActivityProvider>
      </AppSettingsProvider>
    </Providers>
  </StrictMode>,
);
