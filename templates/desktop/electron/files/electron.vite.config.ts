import { resolve } from "node:path";

import { defineConfig, externalizeDepsPlugin } from "electron-vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import type { Plugin } from "vite";

import { rendererContentSecurityPolicy } from "./src/shared/csp";

const cspMeta = /(<meta\s+http-equiv="Content-Security-Policy"\s+content=")[^"]*(")/;

// Writes the mode-specific CSP into index.html. `electron-vite dev` gets the
// relaxed policy React Fast Refresh needs; `electron-vite build` gets the
// production policy. Fails the build if the meta tag disappears so the
// renderer can never ship without a CSP.
function rendererCsp(): Plugin {
  let isDevelopment = false;

  return {
    name: "renderer-csp",
    configResolved(config) {
      isDevelopment = config.command === "serve";
    },
    transformIndexHtml(html) {
      if (!cspMeta.test(html)) {
        throw new Error("src/renderer/index.html must contain a Content-Security-Policy meta tag");
      }

      return html.replace(cspMeta, `$1${rendererContentSecurityPolicy(isDevelopment)}$2`);
    },
  };
}

export default defineConfig({
  main: {
    plugins: [externalizeDepsPlugin({ exclude: ["p-queue"] })],
    build: {
      rollupOptions: {
        input: {
          index: resolve("src/main/index.ts"),
          "workers/activity-worker": resolve("src/main/workers/activity-worker.ts"),
        },
        output: { dynamicImportInCjs: true },
      },
    },
  },
  preload: {
    plugins: [externalizeDepsPlugin({ exclude: ["zod"] })],
  },
  renderer: {
    root: resolve("src/renderer"),
    resolve: {
      alias: {
        "@": resolve("src/renderer/src"),
        "@renderer": resolve("src/renderer/src"),
        "@shared": resolve("src/shared"),
      },
    },
    plugins: [rendererCsp(), react(), tailwindcss()],
  },
});
