import { describe, expect, it } from "vitest";

import { libraryImportSchema, librarySchemas } from "../../src/shared/ipc/library";

describe("library schemas", () => {
  it("trims and deduplicates tags", () => {
    const parsed = librarySchemas.create.parse({
      title: "  Release  ",
      description: "",
      tags: ["release", " release ", "desktop"],
    });

    expect(parsed.title).toBe("Release");
    expect(parsed.tags).toEqual(["release", "desktop"]);
  });

  it("rejects an empty title and invalid update payload", () => {
    expect(librarySchemas.create.safeParse({ title: " " }).success).toBe(false);
    expect(librarySchemas.update.safeParse({ id: crypto.randomUUID() }).success).toBe(false);
  });

  it("validates portable import files", () => {
    expect(
      libraryImportSchema.safeParse({ items: [{ title: "Imported", tags: ["one"] }] }).success,
    ).toBe(true);
    expect(libraryImportSchema.safeParse({ items: [{ title: "" }] }).success).toBe(false);
  });
});
