import type { AuthUser } from "../../../shared/ipc/auth";
import { authChannels, authSchemas } from "../../../shared/ipc/auth";
import { loadConfig } from "../../infra/config";
import { handle } from "../../ipc/handle";
import { AuthClient } from "./auth-client";

let client: AuthClient | null = null;

function authClient(): AuthClient {
  client ??= new AuthClient(loadConfig());

  return client;
}

export function registerAuthHandlers(): void {
  handle(authChannels.login, authSchemas.login, (input) =>
    authClient().login(input.email, input.password),
  );
  handle(authChannels.register, authSchemas.register, (input) =>
    authClient().register(input.email, input.password),
  );
  handle(authChannels.logout, authSchemas.empty, async (): Promise<{ loggedOut: true }> => {
    await authClient().logout();
    return { loggedOut: true };
  });
  handle(authChannels.me, authSchemas.empty, () => authClient().me() as Promise<AuthUser | null>);
}
