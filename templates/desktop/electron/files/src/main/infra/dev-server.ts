import { app } from "electron";

import { resolveDevRendererUrl } from "../../shared/security";

/**
 * The electron-vite dev server origin during `make dev`; undefined in packaged
 * builds and for anything that is not a loopback http(s) server.
 */
export function devRendererUrl(): string | undefined {
  return resolveDevRendererUrl(process.env.ELECTRON_RENDERER_URL, app.isPackaged);
}
