import Image from "next/image";
import Link from "next/link";
import type { ReactNode } from "react";
import { Reveal } from "@/components/reveal";
import { site } from "@/lib/site";

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <main className="min-h-dvh bg-background text-charcoal">
      <div className="grid min-h-dvh lg:grid-cols-[1.05fr_0.95fr]">
        <div className="flex min-h-dvh flex-col px-5 py-6 sm:px-8 lg:px-12">
          <header className="flex flex-wrap items-center justify-between gap-4">
            <Link
              className="font-display text-2xl font-extrabold lowercase tracking-[-0.07em]"
              href="/"
            >
              {site.name}
            </Link>
            <Link
              className="border-b border-charcoal/30 pb-0.5 text-sm font-semibold transition-colors hover:border-tide hover:text-tide"
              href="/"
            >
              Back to site
            </Link>
          </header>

          <div className="mx-auto flex w-full max-w-lg flex-1 flex-col justify-center py-12">
            <p className="text-[0.68rem] font-bold uppercase tracking-[0.18em] text-coral">
              Private access
            </p>
            <Reveal className="mt-6">{children}</Reveal>
            <div className="mt-10 flex flex-wrap justify-between gap-2 border-t border-charcoal/15 pt-4 text-xs text-muted-warm">
              <span>Boateng Peter</span>
              <span>Full-stack engineer</span>
            </div>
          </div>

          <footer className="flex flex-wrap justify-between gap-2 text-xs text-muted-warm">
            <span>{site.name}</span>
            <span>Secure account access</span>
          </footer>
        </div>

        <aside className="hidden p-4 lg:block">
          <div className="relative h-full min-h-[calc(100dvh-2rem)] overflow-hidden rounded-[50px] bg-ink">
            <Image
              alt="A community workshop gathered around a table under warm string lights."
              className="object-cover"
              fill
              sizes="(max-width: 1024px) 0vw, 48vw"
              src="/images/featured.jpeg"
            />
            <div aria-hidden="true" className="hero-card-wash absolute inset-0" />
            <div className="relative flex h-full flex-col justify-between p-10 text-cream xl:p-14">
              <p className="text-xs font-bold uppercase tracking-[0.18em] text-cream/65">
                {site.name} / Private access
              </p>
              <div className="max-w-md">
                <p className="font-display text-4xl font-bold uppercase leading-[0.94] tracking-[-0.05em] xl:text-5xl">
                  Make the work.
                  <br />
                  Keep it yours.
                </p>
                <p className="mt-5 max-w-xs text-sm leading-6 text-cream/65">
                  A quiet place to do focused work, with the people and the
                  details that matter.
                </p>
              </div>
            </div>
          </div>
        </aside>
      </div>
    </main>
  );
}
