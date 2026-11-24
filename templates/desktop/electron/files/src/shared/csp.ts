// Single source of truth for the renderer Content-Security-Policy.
//
// The same policy is written into src/renderer/index.html at build/serve time
// (see electron.vite.config.ts) and sent as a response header by the main
// process (see src/main/security/index.ts). Browsers enforce both, so they must
// agree or the stricter one wins.
//
// Development relaxes exactly two things for the Vite dev server:
// - script-src 'unsafe-inline': @vitejs/plugin-react injects an inline
//   React Fast Refresh preamble. Without it every component module throws
//   "can't detect preamble" and HMR stops working.
// - connect-src localhost http/ws: the Vite HMR client websocket.
// Production never gets either.
export function rendererContentSecurityPolicy(isDevelopment: boolean): string {
  const scriptSource = isDevelopment ? "script-src 'self' 'unsafe-inline'" : "script-src 'self'";
  const connectSource = isDevelopment
    ? "connect-src 'self' http://localhost:* ws://localhost:*"
    : "connect-src 'self'";

  return [
    "default-src 'self'",
    scriptSource,
    "style-src 'self' 'unsafe-inline'",
    "img-src 'self' data:",
    connectSource,
    "font-src 'self'",
    "object-src 'none'",
    "base-uri 'self'",
    "frame-ancestors 'none'",
  ].join("; ");
}
