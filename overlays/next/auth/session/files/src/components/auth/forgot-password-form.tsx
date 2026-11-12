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
import {
  forgotPasswordSchema,
  type ForgotPasswordInput,
} from "@/validations/auth";

export function ForgotPasswordForm() {
  const router = useRouter();
  const [formError, setFormError] = useState<string | null>(null);

  const form = useForm<ForgotPasswordInput>({
    resolver: zodResolver(forgotPasswordSchema),
    defaultValues: {
      email: "",
    },
  });

  async function onSubmit(values: ForgotPasswordInput) {
    setFormError(null);

    const { error } = await authClient.emailOtp.requestPasswordReset({
      email: values.email,
    });

    if (error) {
      setFormError(error.message ?? "Unable to start the reset.");

      return;
    }

    router.push(`/reset-password?email=${encodeURIComponent(values.email)}`);
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle className="font-display text-2xl">Reset password</CardTitle>
        <CardDescription>
          We will email a 6-digit code if an account exists for that address.
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
            {formError ? (
              <p className="text-sm text-destructive" role="alert">
                {formError}
              </p>
            ) : null}
            <Button className="w-full" disabled={form.formState.isSubmitting} type="submit">
              {form.formState.isSubmitting ? "Sending..." : "Send reset code"}
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
