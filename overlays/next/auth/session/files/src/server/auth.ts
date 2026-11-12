import "server-only";
import { betterAuth } from "better-auth";
import { drizzleAdapter } from "better-auth/adapters/drizzle";
import { emailOTP } from "better-auth/plugins/email-otp";
import * as schema from "@/server/db/schema";
import { db } from "@/server/db";
import { sendAuthEmail } from "@/server/email";
import { serverEnv } from "@/server/env";
import { site } from "@/lib/site";

function trustedOrigins(): string[] {
  const configured = serverEnv.BETTER_AUTH_TRUSTED_ORIGINS;

  if (!configured) {
    return [];
  }

  return configured
    .split(",")
    .map((origin) => origin.trim())
    .filter(Boolean);
}

export const auth = betterAuth({
  appName: site.name,
  baseURL: serverEnv.BETTER_AUTH_URL,
  secret: serverEnv.BETTER_AUTH_SECRET,
  database: drizzleAdapter(db, {
    provider: "pg",
    schema,
  }),
  emailAndPassword: {
    enabled: true,
    requireEmailVerification: true,
    minPasswordLength: 10,
  },
  plugins: [
    emailOTP({
      storeOTP: "hashed",
      expiresIn: 600,
      allowedAttempts: 5,
      async sendVerificationOTP({ email, otp, type }) {
        await sendAuthEmail({ to: email, otp, type });
      },
    }),
  ],
  trustedOrigins: trustedOrigins(),
  rateLimit: {
    enabled: true,
    window: 60,
    max: 20,
    customRules: {
      "/api/auth/sign-in/email": { window: 60, max: 5 },
      "/api/auth/sign-up/email": { window: 60, max: 3 },
      "/api/auth/email-otp/request-password-reset": { window: 60, max: 3 },
    },
  },
});
