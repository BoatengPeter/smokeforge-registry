import { failure, success } from "@/lib/api/envelope";

describe("api envelope", () => {
  it("wraps successful data", () => {
    expect(success({ id: "1" })).toEqual({ data: { id: "1" } });
  });

  it("wraps errors with optional fields", () => {
    expect(failure("validation_failed", "Invalid input.", { name: ["Required"] }))
      .toEqual({
        error: {
          code: "validation_failed",
          message: "Invalid input.",
          fields: { name: ["Required"] },
        },
      });
  });
});
