export function isTrustedRendererUrl(url: string, devUrl?: string): boolean {
  if (devUrl) {
    return url.startsWith(devUrl);
  }

  return url === "app://bundle" || url.startsWith("app://bundle/");
}
