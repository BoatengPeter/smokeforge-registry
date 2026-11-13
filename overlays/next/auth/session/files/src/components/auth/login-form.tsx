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
import { loginSchema, type LoginInput } from "@/validations/auth";

export function LoginForm() {
  const router = useRouter();
  const [formError, setFormError] = useState<string | null>(null);

  const form = useForm<LoginInput>({
    resolver: zodResolver(loginSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  async function onSubmit(values: LoginInput) {
    setFormError(null);

    const { error } = await authClient.signIn.email({
      email: values.email,
      password: values.password,
      callbackURL: "/dashboard",
    });

    if (error) {
      setFormError(error.message ?? "Unable to sign in.");

      return;
    }

    router.push("/dashboard");
    router.refresh();
  }

  return (
    <Card className="auth-card rounded-none border-0 bg-transparent py-0 shadow-none">
      <CardHeader>
        <CardTitle className="font-display text-4xl font-bold uppercase tracking-[-0.04em] sm:text-5xl">Sign in</CardTitle>
        <CardDescription>Use your email and password.</CardDescription>
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
              name="password"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Password</FormLabel>
                  <FormControl>
                    <Input
                      autoComplete="current-password"
                      type="password"
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
            <Button className="button-fill h-12 w-full rounded-full" disabled={form.formState.isSubmitting} type="submit">
              {form.formState.isSubmitting ? "Signing in..." : "Sign in"}
            </Button>
          </form>
        </Form>
      </CardContent>
      <CardFooter className="flex-col gap-2 text-sm text-muted-foreground">
        <div className="flex flex-wrap items-center justify-center gap-x-4 gap-y-1">
          <Link className="underline decoration-coral decoration-2 underline-offset-4 transition-colors hover:text-tide" href="/forgot-password">
            Forgot password?
          </Link>
          <Link className="underline decoration-coral decoration-2 underline-offset-4 transition-colors hover:text-tide" href="/verify-email">
            Verify email
          </Link>
        </div>
        <p>
          No account?{" "}
          <Link className="font-semibold text-foreground underline decoration-coral decoration-2 underline-offset-4 transition-colors hover:text-tide" href="/signup">
            Create one
          </Link>
        </p>
      </CardFooter>
    </Card>
  );
}
