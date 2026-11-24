import { describe, expect, it } from "vitest";

import { isTrustedRendererUrl, resolveDevRendererUrl } from "../../src/shared/security";

describe("isTrustedRendererUrl", () => {
  it("accepts the packaged app origin", () => {
    expect(isTrustedRendererUrl("app://bundle/index.html")).toBe(true);
    expect(isTrustedRendererUrl("app://bundle/index.html#/library/1")).toBe(true);
  });

  it("rejects look-alike packaged origins", () => {
    expect(isTrustedRendererUrl("app://bundle.evil/index.html")).toBe(false);
    expect(isTrustedRendererUrl("app://bundle@evil/index.html")).toBe(false);
    expect(isTrustedRendererUrl("app://other/index.html")).toBe(false);
  });

  it("accepts the configured development origin", () => {
    expect(isTrustedRendererUrl("http://localhost:5173/", "http://localhost:5173")).toBe(true);
    expect(isTrustedRendererUrl("http://localhost:5173/#/settings", "http://localhost:5173")).toBe(true);
  });

  it("rejects prefix look-alikes of the development origin", () => {
    const dev = "http://localhost:5173";

    expect(isTrustedRendererUrl("http://localhost:51730/", dev)).toBe(false);
    expect(isTrustedRendererUrl("http://localhost:5173.evil.example/", dev)).toBe(false);
    expect(isTrustedRendererUrl("http://localhost:5173@evil.example/", dev)).toBe(false);
  });

  it("does not trust the packaged origin while a development origin is set", () => {
    expect(isTrustedRendererUrl("app://bundle/index.html", "http://localhost:5173")).toBe(false);
  });

  it("rejects arbitrary and malformed origins", () => {
    expect(isTrustedRendererUrl("https://evil.example")).toBe(false);
    expect(isTrustedRendererUrl("not a url")).toBe(false);
    expect(isTrustedRendererUrl("")).toBe(false);
  });
});

describe("resolveDevRendererUrl", () => {
  it("ignores the variable in packaged builds", () => {
    expect(resolveDevRendererUrl("http://localhost:5173", true)).toBeUndefined();
    expect(resolveDevRendererUrl("https://evil.example", true)).toBeUndefined();
  });

  it("accepts loopback dev servers in development", () => {
    expect(resolveDevRendererUrl("http://localhost:5173", false)).toBe("http://localhost:5173");
    expect(resolveDevRendererUrl("http://localhost:5173/", false)).toBe("http://localhost:5173");
    expect(resolveDevRendererUrl("http://127.0.0.1:5174", false)).toBe("http://127.0.0.1:5174");
    expect(resolveDevRendererUrl("http://[::1]:5173", false)).toBe("http://[::1]:5173");
  });

  it("rejects non-loopback or non-http dev servers even in development", () => {
    expect(resolveDevRendererUrl("https://evil.example", false)).toBeUndefined();
    expect(resolveDevRendererUrl("http://localhost.evil.example:5173", false)).toBeUndefined();
    expect(resolveDevRendererUrl("file:///etc/passwd", false)).toBeUndefined();
    expect(resolveDevRendererUrl("not a url", false)).toBeUndefined();
    expect(resolveDevRendererUrl(undefined, false)).toBeUndefined();
    expect(resolveDevRendererUrl("", false)).toBeUndefined();
  });
});
