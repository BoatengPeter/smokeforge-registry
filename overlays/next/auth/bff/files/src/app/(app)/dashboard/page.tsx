"use client";

import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { SignOutButton } from "@/components/auth/sign-out-button";
import { useAuthSession } from "@/lib/query/hooks";

export default function DashboardPage() {
  const router = useRouter();
  const session = useAuthSession();

  useEffect(() => {
    if (session.error) {
      router.replace("/login");
    }
  }, [router, session.error]);

  if (session.isLoading) {
    return <main className="mx-auto max-w-2xl px-6 py-16">Loading...</main>;
  }

  if (!session.data) {
    return null;
  }

  return (
    <main className="mx-auto max-w-2xl space-y-6 px-6 py-16">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-semibold">Dashboard</h1>
          <p className="text-sm text-muted-foreground">{session.data.email}</p>
        </div>
        <SignOutButton />
      </div>
      <p className="rounded-lg border p-4 text-sm">
        This page reads server state through TanStack Query. The API JWT is never
        available to browser JavaScript.
      </p>
    </main>
  );
}
