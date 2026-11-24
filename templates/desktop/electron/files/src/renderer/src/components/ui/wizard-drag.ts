/** Drag maths for `WizardWindow`, kept pure so it can be unit-tested. */

/** Space kept between the window and the viewport edges, in CSS pixels. */
const SCREEN_MARGIN = 16;

export type Offset = { x: number; y: number };
export type Size = { width: number; height: number };

/** Keeps a centred window of `window` size fully inside `viewport`. */
export function clampOffset(offset: Offset, viewport: Size, window: Size): Offset {
  const limitX = Math.max(0, (viewport.width - window.width) / 2 - SCREEN_MARGIN);
  const limitY = Math.max(0, (viewport.height - window.height) / 2 - SCREEN_MARGIN);

  return {
    x: Math.min(limitX, Math.max(-limitX, offset.x)),
    y: Math.min(limitY, Math.max(-limitY, offset.y)),
  };
}
