"use client";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState, type FormEvent } from "react";
import { useRegister } from "@/lib/query/hooks";

export function SignupForm() {
  const router = useRouter();
  const register = useRegister();
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");

  function onSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    register.mutate(
      { email, password },
      { onSuccess: () => router.replace("/dashboard") },
    );
  }

  return (
    <form onSubmit={onSubmit} className="space-y-4">
      <label className="block text-sm font-medium">
        Email
        <input
          type="email"
          autoComplete="email"
          required
          value={email}
          onChange={(event) => setEmail(event.target.value)}
          className="mt-1 w-full rounded-md border bg-background px-3 py-2"
        />
      </label>
      <label className="block text-sm font-medium">
        Password
        <input
          type="password"
          autoComplete="new-password"
          minLength={12}
          required
          value={password}
          onChange={(event) => setPassword(event.target.value)}
          className="mt-1 w-full rounded-md border bg-background px-3 py-2"
        />
      </label>
      {register.error ? (
        <p role="alert" className="text-sm text-red-600">
          {register.error.message}
        </p>
      ) : null}
      <button
        type="submit"
        disabled={register.isPending}
        className="w-full rounded-md bg-foreground px-4 py-2 font-medium text-background disabled:opacity-50"
      >
        {register.isPending ? "Creating account..." : "Create account"}
      </button>
      <p className="text-sm text-muted-foreground">
        Already registered?{" "}
        <Link href="/login" className="underline">
          Sign in
        </Link>
      </p>
    </form>
  );
}
