"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState } from "react";
import { useForm } from "react-hook-form";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { authClient } from "@/lib/auth-client";
import { verifyEmailSchema, type VerifyEmailInput } from "@/validations/auth";

export function VerifyEmailForm({ initialEmail }: { initialEmail: string }) {
  const router = useRouter();
  const [formError, setFormError] = useState<string | null>(null);
  const [resendMessage, setResendMessage] = useState<string | null>(null);

  const form = useForm<VerifyEmailInput>({
    resolver: zodResolver(verifyEmailSchema),
    defaultValues: {
      email: initialEmail,
      otp: "",
    },
  });

  async function onSubmit(values: VerifyEmailInput) {
    setFormError(null);

    const { error } = await authClient.emailOtp.verifyEmail({
      email: values.email,
      otp: values.otp,
    });

    if (error) {
      setFormError(error.message ?? "Unable to verify the code.");

      return;
    }

    router.push("/login?verified=1");
  }

  async function resendCode() {
    setResendMessage(null);
    const email = form.getValues("email");

    if (!email) {
      setResendMessage("Enter your email address first.");

      return;
    }

    const { error } = await authClient.emailOtp.sendVerificationOtp({
      email,
      type: "email-verification",
    });

    setResendMessage(
      error ? (error.message ?? "Unable to resend the code.") : "A new code was sent.",
    );
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="font-display text-2xl">Verify email</CardTitle>
        <CardDescription>
          Enter the 6-digit code from your email. Codes expire after 10 minutes.
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Form {...form}>
          <form className="flex flex-col gap-5" onSubmit={form.handleSubmit(onSubmit)}>
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Email</FormLabel>
                  <FormControl>
                    <Input autoComplete="email" type="email" {...field} />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="otp"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Code</FormLabel>
                  <FormControl>
                    <Input
                      autoComplete="one-time-code"
                      inputMode="numeric"
                      maxLength={6}
                      placeholder="000000"
                      {...field}
                    />
                  </FormControl>
                  <FormMessage />
                </FormItem>
              )}
            />
            {formError ? (
              <p className="text-sm text-destructive" role="alert">
                {formError}
              </p>
            ) : null}
            {resendMessage ? (
              <p className="text-sm text-muted-foreground">{resendMessage}</p>
            ) : null}
            <Button className="w-full" disabled={form.formState.isSubmitting} type="submit">
              {form.formState.isSubmitting ? "Verifying..." : "Verify email"}
            </Button>
            <Button onClick={resendCode} type="button" variant="ghost">
              Resend code
            </Button>
          </form>
        </Form>
      </CardContent>
      <CardFooter className="justify-center text-sm text-muted-foreground">
        <Link className="underline-offset-4 hover:underline" href="/login">
          Back to sign in
        </Link>
      </CardFooter>
    </Card>
  );
}
