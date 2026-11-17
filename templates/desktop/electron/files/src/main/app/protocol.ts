import { readFile } from "node:fs/promises";
import { extname, join, normalize, sep } from "node:path";

import { protocol } from "electron";

import { rendererPath } from "../infra/paths";

export const appScheme = "app";

const contentTypes: Record<string, string> = {
  ".css": "text/css; charset=utf-8",
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".woff2": "font/woff2",
};

export function registerAppProtocol(): void {
  const root = rendererPath();

  protocol.handle(appScheme, async (request) => {
    const pathname = decodeURIComponent(new URL(request.url).pathname);
    const relative = pathname === "/" ? "index.html" : pathname.replace(/^\/+/, "");
    const file = normalize(join(root, relative));

    if (!file.startsWith(`${root}${sep}`)) {
      return new Response("Not found", { status: 404 });
    }

    try {
      const body = await readFile(file);
      return new Response(body, {
        headers: {
          "Content-Type": contentTypes[extname(file)] ?? "application/octet-stream",
        },
      });
    } catch {
      return new Response("Not found", { status: 404 });
    }
  });
}

export function registerAppScheme(): void {
  protocol.registerSchemesAsPrivileged([
    {
      scheme: appScheme,
      privileges: {
        standard: true,
        secure: true,
        supportFetchAPI: true,
        corsEnabled: true,
      },
    },
  ]);
}
