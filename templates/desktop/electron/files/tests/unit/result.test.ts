import { describe, expect, it } from "vitest";

import { fail, ok } from "../../src/shared/types/result";

describe("Result", () => {
  it("wraps success data", () => {
    expect(ok({ value: 1 })).toEqual({ ok: true, data: { value: 1 } });
  });

  it("wraps failures with a stable error", () => {
    expect(fail("invalid_request", "Invalid request")).toEqual({
      ok: false,
      error: { code: "invalid_request", message: "Invalid request" },
    });
  });
});
