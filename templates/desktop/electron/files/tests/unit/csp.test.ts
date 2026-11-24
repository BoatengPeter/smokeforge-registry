import { describe, expect, it } from "vitest";

import { rendererContentSecurityPolicy } from "../../src/shared/csp";

function directive(csp: string, name: string): string | undefined {
  return csp
    .split(";")
    .map((part) => part.trim())
    .find((part) => part.startsWith(`${name} `));
}

describe("rendererContentSecurityPolicy", () => {
  it("allows the React Fast Refresh preamble and HMR socket in development", () => {
    const csp = rendererContentSecurityPolicy(true);

    expect(directive(csp, "script-src")).toBe("script-src 'self' 'unsafe-inline'");
    expect(directive(csp, "connect-src")).toContain("ws://localhost:*");
  });

  it("keeps production strict", () => {
    const csp = rendererContentSecurityPolicy(false);

    expect(directive(csp, "script-src")).toBe("script-src 'self'");
    expect(directive(csp, "connect-src")).toBe("connect-src 'self'");
    expect(csp).not.toContain("localhost");
    expect(directive(csp, "object-src")).toBe("object-src 'none'");
  });
});
