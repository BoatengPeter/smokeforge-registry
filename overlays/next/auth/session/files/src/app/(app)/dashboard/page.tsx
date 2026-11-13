import Link from "next/link";
import { headers } from "next/headers";
import { redirect } from "next/navigation";
import { Reveal } from "@/components/reveal";
import { SignOutButton } from "@/components/auth/sign-out-button";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { site } from "@/lib/site";
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
    <main className="mx-auto flex min-h-dvh w-full max-w-5xl flex-col px-5 py-8 sm:px-8 lg:px-12">
      <header className="flex flex-wrap items-center justify-between gap-4 border-b border-charcoal/15 pb-6">
        <Link
          className="font-display text-2xl font-extrabold lowercase tracking-[-0.07em]"
          href="/"
        >
          {site.name}
        </Link>
        <div className="flex items-center gap-2">
          <Button asChild className="rounded-full" variant="ghost">
            <Link href="/">Home</Link>
          </Button>
          <SignOutButton />
        </div>
      </header>

      <Reveal className="my-auto grid gap-10 py-16 lg:grid-cols-[0.8fr_1.2fr] lg:items-start lg:gap-20">
        <div>
          <p className="text-sm text-muted-warm">Your account</p>
          <h1 className="mt-4 font-display text-[clamp(2.8rem,6vw,5rem)] font-bold leading-[0.94] tracking-[-0.06em]">
            Dashboard
          </h1>
          <p className="mt-5 max-w-sm text-base leading-7 text-muted-warm">
            Your session is read on the server. Keep this area focused on the
            work your people came to do.
          </p>
        </div>

        <Card className="rounded-[28px] border border-charcoal/15 bg-card shadow-none">
          <CardHeader>
            <CardTitle className="font-display text-2xl font-bold tracking-[-0.04em]">
              Signed in
            </CardTitle>
            <CardDescription>
              Session and user data are read on the server.
            </CardDescription>
          </CardHeader>
          <CardContent>
            <dl className="grid gap-4 text-sm sm:grid-cols-[8rem_1fr]">
              <dt className="text-muted-warm">Name</dt>
              <dd>{session.user.name}</dd>
              <dt className="text-muted-warm">Email</dt>
              <dd>{session.user.email}</dd>
              <dt className="text-muted-warm">Verified</dt>
              <dd>{session.user.emailVerified ? "Yes" : "No"}</dd>
            </dl>
          </CardContent>
        </Card>
      </Reveal>
    </main>
  );
}
