import { describe, expect, it } from "vitest";

import { isTrustedRendererUrl } from "../../src/shared/security";

describe("isTrustedRendererUrl", () => {
  it("accepts the packaged app origin", () => {
    expect(isTrustedRendererUrl("app://bundle/index.html")).toBe(true);
  });

  it("accepts the configured development origin", () => {
    expect(isTrustedRendererUrl("http://localhost:5173/", "http://localhost:5173")).toBe(true);
  });

  it("rejects arbitrary origins", () => {
    expect(isTrustedRendererUrl("https://evil.example")).toBe(false);
  });
});
