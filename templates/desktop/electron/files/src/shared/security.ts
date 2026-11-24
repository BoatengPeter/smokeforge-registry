const loopbackHosts = new Set(["localhost", "127.0.0.1", "[::1]"]);

function parseUrl(value: string): URL | undefined {
  try {
    return new URL(value);
  } catch {
    return undefined;
  }
}

/**
 * Returns the dev server origin to load and trust, or undefined.
 *
 * electron-vite sets ELECTRON_RENDERER_URL during `make dev`. A packaged app
 * must never honor it: otherwise anyone who can launch the installed app with
 * an environment variable could load their own page into the trusted window
 * and reach the preload API. In development only http(s) loopback servers are
 * accepted.
 */
export function resolveDevRendererUrl(raw: string | undefined, isPackaged: boolean): string | undefined {
  if (isPackaged || !raw) {
    return undefined;
  }

  const parsed = parseUrl(raw);
  if (!parsed || (parsed.protocol !== "http:" && parsed.protocol !== "https:")) {
    return undefined;
  }

  return loopbackHosts.has(parsed.hostname) ? parsed.origin : undefined;
}

/**
 * Whether a navigation target or IPC sender URL belongs to the app renderer.
 * Compares exact origins, never string prefixes.
 */
export function isTrustedRendererUrl(url: string, devOrigin?: string): boolean {
  const parsed = parseUrl(url);
  if (!parsed) {
    return false;
  }

  if (devOrigin) {
    return parsed.origin === devOrigin;
  }

  return parsed.protocol === "app:" && parsed.host === "bundle" && parsed.username === "" && parsed.password === "";
}
