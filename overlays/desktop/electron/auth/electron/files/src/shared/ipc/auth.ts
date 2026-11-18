import { z } from "zod";

import type { Result } from "../types/result";

export { authChannels } from "./auth-channels";

export const authSchemas = {
  login: z.object({
    email: z.string().trim().email().max(320),
    password: z.string().min(1).max(256),
  }),
  register: z.object({
    email: z.string().trim().email().max(320),
    password: z.string().min(12).max(256),
  }),
  empty: z.void(),
};

export type LoginInput = z.infer<typeof authSchemas.login>;
export type RegisterInput = z.infer<typeof authSchemas.register>;

export type AuthUser = {
  id: string;
  email: string;
  role: string;
  email_verified?: boolean;
  created_at: string;
};

export type LogoutResult = {
  loggedOut: true;
};

export type AuthApi = {
  login: (input: LoginInput) => Promise<Result<AuthUser>>;
  register: (input: RegisterInput) => Promise<Result<AuthUser>>;
  logout: () => Promise<Result<LogoutResult>>;
  me: () => Promise<Result<AuthUser | null>>;
};
