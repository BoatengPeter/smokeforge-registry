import { isSameOrigin, parseTokens } from "@/lib/auth/tokens";

describe("parseTokens", () => {
  it("accepts a valid token response", () => {
    expect(
      parseTokens({
        access_token: "access",
        refresh_token: "refresh",
        expires_in: 900,
      }),
    ).toEqual({
      access_token: "access",
      refresh_token: "refresh",
      token_type: undefined,
      expires_in: 900,
    });
  });

  it("rejects an invalid token response", () => {
    expect(parseTokens({ access_token: "access" })).toBeNull();
    expect(parseTokens(null)).toBeNull();
  });
});

describe("isSameOrigin", () => {
  it("allows the configured origin", () => {
    expect(
      isSameOrigin(
        "http://localhost:3000",
        "http://localhost:3000/",
        "http://localhost:3000/api/auth/me",
      ),
    ).toBe(true);
  });

  it("rejects a different origin", () => {
    expect(
      isSameOrigin(
        "https://evil.example",
        undefined,
        "http://localhost:3000/api/auth/me",
      ),
    ).toBe(false);
  });
});
