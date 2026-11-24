import { describe, expect, it } from "vitest";

import { clampZoom, stepZoom, zoom, zoomLabel } from "../../src/shared/zoom";

describe("interface size", () => {
  it("steps without drifting and stays in range", () => {
    expect(stepZoom(1.2, zoom.step)).toBe(1.3);
    expect(stepZoom(zoom.max, zoom.step)).toBe(zoom.max);
    expect(stepZoom(zoom.min, -zoom.step)).toBe(zoom.min);
    expect(clampZoom(9)).toBe(zoom.max);
  });

  it("labels as a whole percentage", () => {
    expect(zoomLabel(1.3)).toBe("130%");
    expect(zoomLabel(zoom.default)).toBe("100%");
  });
});
