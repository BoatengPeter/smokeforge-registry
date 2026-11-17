import type { ReactNode } from "react";
// smokeforge:anchor state-imports

export function Providers({ children }: { children: ReactNode }) {
  return (
    <>
      {/* smokeforge:anchor state-begin */}
      {children}
      {/* smokeforge:anchor state-end */}
    </>
  );
}
