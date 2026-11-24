/**
 * Interface size: the window's zoom factor, 1 is 100%.
 *
 * Settings is the single source of truth. Main applies it to every window
 * with `webContents.setZoomFactor`; the renderer changes it only through
 * `settings.update({ zoom })` (Settings, the welcome tour, Ctrl/Cmd + − 0).
 */
export const zoom = {
  default: 1,
  min: 0.7,
  max: 2,
  step: 0.1,
} as const;

export function clampZoom(value: number): number {
  return Math.min(zoom.max, Math.max(zoom.min, value));
}

/** `value + delta`, clamped and rounded to two decimals so steps don't drift. */
export function stepZoom(value: number, delta: number): number {
  return clampZoom(Math.round((value + delta) * 100) / 100);
}

/** `130%`. */
export function zoomLabel(value: number): string {
  return `${Math.round(value * 100)}%`;
}
