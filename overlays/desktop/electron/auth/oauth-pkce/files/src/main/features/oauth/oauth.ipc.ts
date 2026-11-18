import { oauthChannels, oauthSchemas } from "../../../shared/ipc/oauth";
import { handle } from "../../ipc/handle";
import { getOAuthState, loginOAuth, logoutOAuth } from "./oauth-service";

export function registerOAuthHandlers(): void {
  handle(oauthChannels.login, oauthSchemas.login, () => loginOAuth());
  handle(oauthChannels.logout, oauthSchemas.logout, () => logoutOAuth());
  handle(oauthChannels.status, oauthSchemas.status, () => getOAuthState());
}
