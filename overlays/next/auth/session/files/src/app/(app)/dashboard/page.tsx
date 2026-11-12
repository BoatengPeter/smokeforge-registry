import Link from "next/link";
import { headers } from "next/headers";
import { redirect } from "next/navigation";
import { SignOutButton } from "@/components/auth/sign-out-button";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { auth } from "@/server/auth";

export const instant = false;

export default async function DashboardPage() {
  const session = await auth.api.getSession({
    headers: await headers(),
  });

  if (!session) {
    redirect("/login");
  }

  return (
    <main className="mx-auto flex min-h-dvh w-full max-w-3xl flex-col gap-10 px-6 py-16">
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div>
          <h1 className="font-display text-4xl tracking-tight">Dashboard</h1>
          <p className="mt-2 text-muted-foreground">
            This page is protected by the Better Auth session.
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button asChild variant="outline">
            <Link href="/">Home</Link>
          </Button>
          <SignOutButton />
        </div>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Signed in</CardTitle>
          <CardDescription>
            Session and user data are read on the server.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <dl className="grid gap-4 text-sm sm:grid-cols-[8rem_1fr]">
            <dt className="text-muted-foreground">Name</dt>
            <dd>{session.user.name}</dd>
            <dt className="text-muted-foreground">Email</dt>
            <dd>{session.user.email}</dd>
            <dt className="text-muted-foreground">Verified</dt>
            <dd>{session.user.emailVerified ? "Yes" : "No"}</dd>
          </dl>
        </CardContent>
      </Card>
    </main>
  );
}
