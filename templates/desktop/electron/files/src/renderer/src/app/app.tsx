import { useEffect, useState } from "react";

import type { SystemInfo } from "@shared/ipc/system";
import { DesktopApiError, invoke } from "@renderer/lib/desktop-api";

export function App() {
  const [info, setInfo] = useState<SystemInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let active = true;

    invoke(window.desktopApi.system.getVersion())
      .then((value) => {
        if (active) setInfo(value);
      })
      .catch((reason: unknown) => {
        if (!active) return;
        setError(reason instanceof DesktopApiError ? reason.message : "Unable to load app info.");
      });

    return () => {
      active = false;
    };
  }, []);

  return (
    <main className="mx-auto flex min-h-screen max-w-4xl flex-col justify-center gap-8 px-8 py-12">
      <div>
        <p className="text-sm font-medium uppercase tracking-[0.2em] text-cyan-300">Smokeforge</p>
        <h1 className="mt-3 text-4xl font-semibold tracking-tight text-white">Electron desktop app</h1>
        <p className="mt-3 max-w-xl text-slate-300">
          The renderer is sandboxed. System access goes through a typed, validated preload IPC bridge.
        </p>
      </div>

      <section className="grid gap-4 sm:grid-cols-3" aria-label="Application information">
        <article className="rounded-xl border border-white/10 bg-white/5 p-4">
          <p className="text-xs uppercase tracking-wide text-slate-400">Version</p>
          <p className="mt-2 text-lg font-medium text-white" data-testid="app-version">
            {info?.version ?? "..."}
          </p>
        </article>
        <article className="rounded-xl border border-white/10 bg-white/5 p-4">
          <p className="text-xs uppercase tracking-wide text-slate-400">Platform</p>
          <p className="mt-2 text-lg font-medium text-white">{info?.platform ?? "..."}</p>
        </article>
        <article className="rounded-xl border border-white/10 bg-white/5 p-4">
          <p className="text-xs uppercase tracking-wide text-slate-400">Architecture</p>
          <p className="mt-2 text-lg font-medium text-white">{info?.arch ?? "..."}</p>
        </article>
      </section>

      {error ? (
        <p role="alert" className="rounded-lg border border-red-400/30 bg-red-400/10 p-4 text-sm text-red-200">
          {error}
        </p>
      ) : null}
    </main>
  );
}
