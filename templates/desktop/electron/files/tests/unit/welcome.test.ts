import { describe, expect, it } from "vitest";

import { nextPage, pageIndex, previousPage, welcomePages } from "../../src/renderer/src/features/welcome/welcome-pages";
import { clampOffset } from "../../src/renderer/src/components/ui/wizard-drag";

describe("welcome tour pages", () => {
  it("step in order", () => {
    expect(previousPage("intro")).toBeUndefined();
    expect(nextPage("intro")).toBe("shortcuts");
    expect(previousPage("shortcuts")).toBe("intro");
    expect(nextPage("ready")).toBeUndefined();
    expect(pageIndex("ready")).toBe(welcomePages.length - 1);
  });
});

describe("wizard window drag", () => {
  const viewport = { width: 1000, height: 700 };
  const window = { width: 832, height: 600 };

  it("keeps the window on screen", () => {
    expect(clampOffset({ x: 5000, y: -5000 }, viewport, window)).toEqual({ x: 68, y: -34 });
    expect(clampOffset({ x: 10, y: 10 }, viewport, window)).toEqual({ x: 10, y: 10 });
  });

  it("pins a window larger than the viewport to the centre", () => {
    expect(clampOffset({ x: 50, y: 50 }, { width: 400, height: 300 }, window)).toEqual({ x: 0, y: 0 });
  });
});
