import "server-only";
import { logger } from "@/server/logger";
import { serverEnv } from "@/server/env";
import type { AuthOTPType } from "@/types/auth";

function subjectFor(type: AuthOTPType): string {
  switch (type) {
    case "email-verification":
      return "Verify your email";
    case "forget-password":
      return "Reset your password";
    case "sign-in":
      return "Your sign-in code";
    case "change-email":
      return "Confirm your email change";
  }
}

export async function sendAuthEmail(input: {
  to: string;
  otp: string;
  type: AuthOTPType;
}): Promise<void> {
  if (serverEnv.RESEND_API_KEY && serverEnv.EMAIL_FROM) {
    const response = await fetch("https://api.resend.com/emails", {
      method: "POST",
      headers: {
        Authorization: `Bearer ${serverEnv.RESEND_API_KEY}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        from: serverEnv.EMAIL_FROM,
        to: input.to,
        subject: subjectFor(input.type),
        text: `Your code is ${input.otp}. It expires in 10 minutes. If you did not request this, ignore this email.`,
      }),
    });

    if (!response.ok) {
      throw new Error("Unable to send authentication email.");
    }

    return;
  }

  const hostname = new URL(serverEnv.BETTER_AUTH_URL).hostname;
  const isLocalHost =
    hostname === "localhost" ||
    hostname === "127.0.0.1" ||
    hostname === "::1";

  if (isLocalHost) {
    logger.info("development auth OTP for %s (%s): %s", input.to, input.type, input.otp);

    return;
  }

  throw new Error(
    "Email delivery is not configured. Set RESEND_API_KEY and EMAIL_FROM.",
  );
}
