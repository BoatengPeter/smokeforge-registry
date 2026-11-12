import Link from "next/link";
import type { ReactNode } from "react";
import { site } from "@/lib/site";

export default function AuthLayout({ children }: { children: ReactNode }) {
  return (
    <main className="grid min-h-dvh place-items-center bg-muted/30 px-6 py-12">
      <div className="w-full max-w-md">
        <Link className="mb-8 inline-block font-display text-xl tracking-tight" href="/">
          {site.name}
        </Link>
        {children}
      </div>
    </main>
  );
}
