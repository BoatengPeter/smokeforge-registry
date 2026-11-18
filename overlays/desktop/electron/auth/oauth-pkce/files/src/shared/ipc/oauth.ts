import { z } from "zod";

import type { Result } from "../types/result";

export const oauthChannels = {
  login: "oauth:login",
  logout: "oauth:logout",
  status: "oauth:status",
  event: "oauth:event",
} as const;

export const oauthSchemas = {
  login: z.void(),
  logout: z.void(),
  status: z.void(),
};

export type OAuthStatus = "signed-out" | "pending" | "signed-in" | "error";

export type OAuthState = {
  status: OAuthStatus;
  message: string;
};

export type OAuthApi = {
  login: () => Promise<Result<OAuthState>>;
  logout: () => Promise<Result<OAuthState>>;
  status: () => Promise<Result<OAuthState>>;
  onEvent: (listener: (state: OAuthState) => void) => () => void;
};
