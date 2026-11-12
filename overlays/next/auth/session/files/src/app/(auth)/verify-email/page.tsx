import type { Metadata } from "next";
import { VerifyEmailForm } from "@/components/auth/verify-email-form";

export const instant = false;

export const metadata: Metadata = {
  title: "Verify email",
};

export default async function VerifyEmailPage({
  searchParams,
}: {
  searchParams: Promise<{ email?: string }>;
}) {
  const { email } = await searchParams;

  return <VerifyEmailForm initialEmail={email ?? ""} />;
}
