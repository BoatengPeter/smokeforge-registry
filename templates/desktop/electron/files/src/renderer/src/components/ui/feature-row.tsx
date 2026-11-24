import type { ReactNode } from "react";

/** "What this app does" row for tours and intros: icon tile, title, body. */
export function FeatureRow({ icon, title, children }: { icon: ReactNode; title: string; children: ReactNode }) {
  return (
    <div className="flex items-start gap-3">
      <div className="flex size-8 shrink-0 items-center justify-center rounded-lg border bg-muted text-primary [&_svg]:size-4">
        {icon}
      </div>
      <div className="grid min-w-0 gap-1">
        <p className="text-sm font-semibold">{title}</p>
        <p className="text-sm text-muted-foreground">{children}</p>
      </div>
    </div>
  );
}
