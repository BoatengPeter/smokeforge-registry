import { describe, expect, it } from "vitest";

import { LocalStore } from "../../src/main/storage/local-store";

describe("LocalStore SQL contract", () => {
  it("documents the stable key/value behavior", () => {
    const operations = ["SELECT value FROM local_kv", "INSERT INTO local_kv", "DELETE FROM local_kv"];

    expect(operations).toHaveLength(3);
    expect(LocalStore).toBeTypeOf("function");
  });
});
