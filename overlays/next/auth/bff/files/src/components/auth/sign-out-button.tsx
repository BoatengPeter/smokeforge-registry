"use client";

import { useRouter } from "next/navigation";
import { useLogout } from "@/lib/query/hooks";

export function SignOutButton() {
  const router = useRouter();
  const logout = useLogout();

  return (
    <button
      type="button"
      disabled={logout.isPending}
      onClick={() =>
        logout.mutate({}, { onSuccess: () => router.replace("/login") })
      }
      className="rounded-md border px-3 py-1.5 text-sm font-medium disabled:opacity-50"
    >
      {logout.isPending ? "Signing out..." : "Sign out"}
    </button>
  );
}
