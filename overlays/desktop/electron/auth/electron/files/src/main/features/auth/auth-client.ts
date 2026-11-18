import { net } from "electron";

import type { AuthUser } from "../../../shared/ipc/auth";
import type { AppConfig } from "../../infra/config";
import { userDataPath } from "../../infra/paths";
import { PublicError } from "../../infra/public-error";
import { TokenStore } from "./token-store";

type TokenResponse = {
  access_token: string;
  refresh_token?: string;
  expires_in: number;
};

type ApiBody = {
  error?: { message?: string };
  detail?: string;
  title?: string;
};

export class AuthClient {
  private accessToken: string | null = null;
  private readonly baseUrl: string;
  private readonly tokens: TokenStore;

  constructor(config: AppConfig) {
    this.baseUrl = config.apiBaseUrl.replace(/\/$/, "");
    this.tokens = new TokenStore(userDataPath("auth-token.bin"));
  }

  async login(email: string, password: string): Promise<AuthUser> {
    const response = await this.request("/api/v1/auth/login", {
      method: "POST",
      body: JSON.stringify({ email, password }),
      headers: {
        "Content-Type": "application/json",
        "X-Refresh-Token-Transport": "body",
      },
    });

    if (response.status === 202) {
      throw new PublicError("mfa_required", "This account requires a second factor.");
    }

    if (!response.ok) {
      throw await this.error(response, "invalid_credentials", "Invalid email or password.");
    }

    await this.storeTokens(await this.body<TokenResponse>(response));
    const user = await this.me();
    if (!user) throw new PublicError("auth_unavailable", "The API did not return the current user.");

    return user;
  }

  async register(email: string, password: string): Promise<AuthUser> {
    const response = await this.request("/api/v1/auth/register", {
      method: "POST",
      body: JSON.stringify({ email, password }),
      headers: { "Content-Type": "application/json" },
    });

    if (!response.ok) {
      throw await this.error(response, "registration_failed", "Could not create the account.");
    }

    const body = await this.body<TokenResponse | AuthUser>(response);
    if (body && "access_token" in body) {
      await this.storeTokens(body as TokenResponse);
      const user = await this.me();
      if (user) return user;
    }

    return this.login(email, password);
  }

  async me(): Promise<AuthUser | null> {
    if (!this.accessToken && !(await this.refresh())) {
      return null;
    }

    let response = await this.request("/api/v1/auth/me", {
      headers: { Authorization: `Bearer ${this.accessToken ?? ""}` },
    });

    if (response.status === 401 && (await this.refresh())) {
      response = await this.request("/api/v1/auth/me", {
        headers: { Authorization: `Bearer ${this.accessToken ?? ""}` },
      });
    }

    if (response.status === 401) {
      await this.tokens.clear();
      this.accessToken = null;
      return null;
    }

    if (!response.ok) {
      throw await this.error(response, "auth_unavailable", "Could not load the account.");
    }

    return this.body<AuthUser>(response);
  }

  async logout(): Promise<void> {
    const refreshToken = await this.tokens.load();
    if (refreshToken) {
      await this.request("/api/v1/auth/logout", {
        method: "POST",
        body: JSON.stringify({ refresh_token: refreshToken }),
        headers: { "Content-Type": "application/json" },
      }).catch(() => undefined);
    }

    await this.tokens.clear();
    this.accessToken = null;
  }

  private async refresh(): Promise<boolean> {
    const refreshToken = await this.tokens.load();
    if (!refreshToken) return false;

    const response = await this.request("/api/v1/auth/refresh", {
      method: "POST",
      body: JSON.stringify({ refresh_token: refreshToken }),
      headers: { "Content-Type": "application/json" },
    }).catch(() => null);

    if (!response?.ok) {
      await this.tokens.clear();
      this.accessToken = null;
      return false;
    }

    await this.storeTokens(await this.body<TokenResponse>(response));
    return true;
  }

  private async storeTokens(tokens: TokenResponse): Promise<void> {
    this.accessToken = tokens.access_token;
    if (tokens.refresh_token) {
      await this.tokens.set(tokens.refresh_token);
    }
  }

  private async request(path: string, init: RequestInit): Promise<Response> {
    return net.fetch(`${this.baseUrl}${path}`, {
      ...init,
      headers: {
        Accept: "application/json",
        "X-Requested-With": "XMLHttpRequest",
        ...init.headers,
      },
    });
  }

  private async body<T>(response: Response): Promise<T> {
    return (await response.json()) as T;
  }

  private async error(response: Response, code: string, fallback: string): Promise<PublicError> {
    const body = (await response.json().catch(() => null)) as ApiBody | null;

    return new PublicError(
      code,
      body?.error?.message ?? body?.detail ?? body?.title ?? fallback,
    );
  }
}
