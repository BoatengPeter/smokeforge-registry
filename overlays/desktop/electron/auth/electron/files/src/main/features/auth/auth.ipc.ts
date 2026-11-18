import type { AuthUser } from "../../../shared/ipc/auth";
import { authChannels, authSchemas } from "../../../shared/ipc/auth";
import { loadConfig } from "../../infra/config";
import { handle } from "../../ipc/handle";
import { AuthClient } from "./auth-client";
import { configureLibrarySync, stopLibrarySync } from "../sync/library-sync";

let client: AuthClient | null = null;

export function authClient(): AuthClient {
  client ??= new AuthClient(loadConfig());

  return client;
}

export async function initializeAuthSession(): Promise<void> {
  const user = await authClient().me();

  if (user) {
    configureLibrarySync(user.id, (path, init) => authClient().apiFetch(path, init));
  }
}

export function registerAuthHandlers(): void {
  handle(authChannels.login, authSchemas.login, async (input) => {
    const user = await authClient().login(input.email, input.password);
    configureLibrarySync(user.id, (path, init) => authClient().apiFetch(path, init));

    return user;
  });
  handle(authChannels.register, authSchemas.register, async (input) => {
    const user = await authClient().register(input.email, input.password);
    configureLibrarySync(user.id, (path, init) => authClient().apiFetch(path, init));

    return user;
  });
  handle(authChannels.logout, authSchemas.empty, async (): Promise<{ loggedOut: true }> => {
    stopLibrarySync();
    await authClient().logout();
    return { loggedOut: true };
  });
  handle(authChannels.me, authSchemas.empty, async () => {
    const user = await authClient().me() as AuthUser | null;
    if (user) configureLibrarySync(user.id, (path, init) => authClient().apiFetch(path, init));
    else stopLibrarySync();

    return user;
  });
}
