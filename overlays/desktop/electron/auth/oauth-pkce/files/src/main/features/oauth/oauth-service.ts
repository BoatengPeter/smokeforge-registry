import { BrowserWindow, safeStorage, shell } from "electron";
import { mkdir, readFile, rename, unlink, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

import type { AuthorizationServer, Client } from "oauth4webapi";

import type { OAuthState } from "../../../shared/ipc/oauth";
import { oauthChannels } from "../../../shared/ipc/oauth";
import { userDataPath } from "../../infra/paths";
import { PublicError } from "../../infra/public-error";

type Pending = {
  as: AuthorizationServer;
  client: Client;
  codeVerifier: string;
  state: string;
  redirectUri: string;
};

let pending: Pending | null = null;
let state: OAuthState = { status: "signed-out", message: "" };

class OAuthTokenStore {
  private readonly file = userDataPath("oauth-token.bin");

  async read(): Promise<string | null> {
    if (!(await safeStorage.isAsyncEncryptionAvailable())) return null;
    try {
      const encrypted = Buffer.from((await readFile(this.file, "utf8")).trim(), "base64");
      return safeStorage.decryptString(encrypted);
    } catch {
      return null;
    }
  }

  async write(value: string): Promise<void> {
    if (!(await safeStorage.isAsyncEncryptionAvailable())) return;
    await mkdir(dirname(this.file), { recursive: true, mode: 0o700 });
    const temporary = `${this.file}.${process.pid}.tmp`;
    try {
      await writeFile(temporary, safeStorage.encryptString(value).toString("base64"), { mode: 0o600 });
      await rename(temporary, this.file);
    } catch (error) {
      await unlink(temporary).catch(() => undefined);
      throw error;
    }
  }

  async clear(): Promise<void> {
    await unlink(this.file).catch(() => undefined);
  }
}

const tokens = new OAuthTokenStore();

function publish(next: OAuthState): OAuthState {
  state = next;
  for (const window of BrowserWindow.getAllWindows()) {
    if (!window.isDestroyed()) window.webContents.send(oauthChannels.event, state);
  }
  return state;
}

function settings() {
  const issuer = process.env.OIDC_ISSUER;
  const clientId = process.env.OIDC_CLIENT_ID;
  const redirectUri = process.env.OIDC_REDIRECT_URI ?? "smokeforge://auth/callback";
  if (!issuer || !clientId) {
    throw new PublicError("oauth_not_configured", "Set OIDC_ISSUER and OIDC_CLIENT_ID to enable sign-in.");
  }
  return { issuer: new URL(issuer), clientId, redirectUri };
}

export function oauthScheme(): string {
  try {
    return settings().redirectUri.split(":")[0];
  } catch {
    return "smokeforge";
  }
}

export function getOAuthState(): OAuthState {
  return state;
}

export async function initializeOAuth(): Promise<void> {
  if (await tokens.read()) state = { status: "signed-in", message: "" };
}

export async function loginOAuth(): Promise<OAuthState> {
  const config = settings();
  const oauth = await import("oauth4webapi");
  const discovery = await oauth.discoveryRequest(config.issuer);
  const as = await oauth.processDiscoveryResponse(config.issuer, discovery);
  if (!as.authorization_endpoint) throw new PublicError("oauth_not_configured", "Issuer has no authorization endpoint.");
  const client: Client = { client_id: config.clientId };
  const codeVerifier = oauth.generateRandomCodeVerifier();
  const stateValue = oauth.generateRandomState();
  const challenge = await oauth.calculatePKCECodeChallenge(codeVerifier);
  const url = new URL(as.authorization_endpoint);
  url.searchParams.set("client_id", config.clientId);
  url.searchParams.set("redirect_uri", config.redirectUri);
  url.searchParams.set("response_type", "code");
  url.searchParams.set("scope", process.env.OIDC_SCOPE ?? "openid profile email");
  url.searchParams.set("code_challenge", challenge);
  url.searchParams.set("code_challenge_method", "S256");
  url.searchParams.set("state", stateValue);
  pending = { as, client, codeVerifier, state: stateValue, redirectUri: config.redirectUri };
  publish({ status: "pending", message: "Complete sign-in in your browser." });
  await shell.openExternal(url.href);

  return state;
}

export async function handleOAuthCallback(callbackUrl: string): Promise<void> {
  if (!pending) {
    publish({ status: "error", message: "No sign-in request is pending." });
    return;
  }

  try {
    const oauth = await import("oauth4webapi");
    const params = oauth.validateAuthResponse(pending.as, pending.client, new URL(callbackUrl), pending.state);
    const response = await oauth.authorizationCodeGrantRequest(
      pending.as,
      pending.client,
      oauth.None(),
      params,
      pending.redirectUri,
      pending.codeVerifier,
    );
    const result = await oauth.processAuthorizationCodeResponse(pending.as, pending.client, response);
    await tokens.write(JSON.stringify(result));
    pending = null;
    publish({ status: "signed-in", message: "" });
  } catch (error) {
    publish({ status: "error", message: error instanceof Error ? error.message : "Sign-in failed." });
  }
}

export async function logoutOAuth(): Promise<OAuthState> {
  await tokens.clear();
  pending = null;
  return publish({ status: "signed-out", message: "" });
}
