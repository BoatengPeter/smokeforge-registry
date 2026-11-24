import type { ReactNode } from "react";

import { cn } from "@/lib/utils";

/**
 * A box inside a dialog or tour page. `quiet` holds secondary details;
 * `callout` (accent outline) marks the one thing on a page worth remembering;
 * use at most one per page. On the page canvas use `Card` instead.
 */
export function Inset({ variant = "quiet", children }: { variant?: "quiet" | "callout"; children: ReactNode }) {
  return (
    <div
      className={cn(
        "grid gap-2 rounded-lg border p-3.5",
        variant === "callout" ? "border-primary bg-primary/10" : "bg-muted",
      )}
    >
      {children}
    </div>
  );
}
