import { app } from "electron";

import { handleOAuthCallback, initializeOAuth, oauthScheme } from "../features/oauth/oauth-service";

let scheme = "smokeforge";

export function registerDeepLink(): void {
  scheme = oauthScheme();
  if (process.defaultApp) {
    app.setAsDefaultProtocolClient(scheme, process.execPath, [process.argv[1] ?? ""]);
  } else {
    app.setAsDefaultProtocolClient(scheme);
  }

  app.on("open-url", (event, url) => {
    event.preventDefault();
    if (url.startsWith(`${scheme}://`)) void handleOAuthCallback(url);
  });

  void app.whenReady().then(() => initializeOAuth());
}

export function handleDeepLink(argv: string[]): void {
  const url = argv.find((value) => value.startsWith(`${scheme}://`));
  if (url) void handleOAuthCallback(url);
}
