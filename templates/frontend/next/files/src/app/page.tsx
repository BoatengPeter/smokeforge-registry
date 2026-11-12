import Link from "next/link";
import {
  ArrowRight,
  Database,
  Server,
  ShieldCheck,
  TestTube,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { site } from "@/lib/site";

const capabilities = [
  {
    title: "Route Handler API",
    description:
      "Backend endpoints live in src/app/api/**/route.ts with Zod validation and a consistent response envelope.",
    Icon: Server,
  },
  {
    title: "Postgres ready",
    description:
      "Add the database overlay for Drizzle ORM, migrations, and a typed Postgres client.",
    Icon: Database,
  },
  {
    title: "Secure by default",
    description:
      "Strict TypeScript, security headers, server-only secrets, and Better Auth when the auth overlay is selected.",
    Icon: ShieldCheck,
  },
  {
    title: "Tested",
    description:
      "Jest covers unit behavior and Playwright covers the app shell. CI runs both.",
    Icon: TestTube,
  },
] as const;

export default function Home() {
  return (
    <main className="mx-auto flex min-h-dvh w-full max-w-6xl flex-col gap-24 px-6 py-12 sm:px-10 lg:py-16">
      <header className="flex flex-wrap items-center justify-between gap-4 border-b border-border pb-6">
        <span className="font-display text-xl tracking-tight">{site.name}</span>
        <nav className="flex items-center gap-2">
          {/* smokeforge:anchor auth-cta */}
          <Button asChild size="sm" variant="ghost">
            <Link href="/api/health">Health</Link>
          </Button>
        </nav>
      </header>

      <section className="grid gap-12 lg:grid-cols-[1.15fr_0.85fr] lg:items-end">
        <div className="max-w-3xl">
          <h1 className="font-display text-5xl leading-[1.02] tracking-tight text-balance sm:text-7xl">
            {site.name}
          </h1>
          <p className="mt-6 max-w-2xl text-lg leading-8 text-muted-foreground">
            {site.description}
          </p>
          <div className="mt-10 flex flex-wrap gap-3">
            <Button asChild size="lg">
              <Link href="#capabilities">
                Explore the stack
                <ArrowRight aria-hidden="true" />
              </Link>
            </Button>
            <Button asChild size="lg" variant="outline">
              <Link href="/api/health">Health endpoint</Link>
            </Button>
          </div>
        </div>

        <Card className="bg-muted/40">
          <CardHeader>
            <CardTitle className="font-display text-2xl">
              Ready to extend
            </CardTitle>
          </CardHeader>
          <CardContent>
            <dl className="flex flex-col gap-4 text-sm">
              <div className="flex items-center justify-between gap-6 border-b border-border pb-4">
                <dt className="text-muted-foreground">App URL</dt>
                <dd className="font-mono">NEXT_PUBLIC_APP_URL</dd>
              </div>
              <div className="flex items-center justify-between gap-6 border-b border-border pb-4">
                <dt className="text-muted-foreground">API</dt>
                <dd className="font-mono">/api/v1/**</dd>
              </div>
              <div className="flex items-center justify-between gap-6 border-b border-border pb-4">
                <dt className="text-muted-foreground">Health</dt>
                <dd className="font-mono">/api/health</dd>
              </div>
              <div className="flex items-center justify-between gap-6">
                <dt className="text-muted-foreground">Auth</dt>
                <dd className="font-mono">/api/auth/**</dd>
              </div>
            </dl>
          </CardContent>
        </Card>
      </section>

      <section className="flex flex-col gap-10" id="capabilities">
        <div className="max-w-2xl">
          <h2 className="font-display text-3xl tracking-tight">
            A backend, not just a page
          </h2>
          <p className="mt-3 text-base leading-7 text-muted-foreground">
            The base gives you a working Next.js app. Add auth, Postgres, Docker,
            and CI as opt-in overlays when the project needs them.
          </p>
        </div>

        <div className="grid gap-4 sm:grid-cols-2">
          {capabilities.map(({ title, description, Icon }) => (
            <Card key={title}>
              <CardHeader>
                <Icon
                  aria-hidden="true"
                  className="size-5 text-primary"
                  strokeWidth={1.75}
                />
                <CardTitle className="pt-2 text-lg">{title}</CardTitle>
              </CardHeader>
              <CardContent>
                <p className="text-sm leading-6 text-muted-foreground">
                  {description}
                </p>
              </CardContent>
            </Card>
          ))}
        </div>
      </section>

      <footer className="mt-auto flex flex-wrap items-center justify-between gap-3 border-t border-border pt-6 text-sm text-muted-foreground">
        <p>Next.js 16 · React 19 · Route Handler API</p>
        <p>Metadata, OpenGraph, sitemap, and robots are configured.</p>
      </footer>
    </main>
  );
}
