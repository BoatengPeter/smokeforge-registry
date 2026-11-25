import { windowChrome } from "@shared/design-tokens";

/**
 * The bar over the content column. It drags the window; on Windows and
 * Linux the native title bar overlay draws the window controls at its right
 * edge (`windowChrome.titleBarOverlay`), so keep that corner empty.
 */
export function TitleBar() {
  return (
    <header
      className="app-region-drag flex shrink-0 items-center border-b border-sidebar-border bg-sidebar"
      style={{ height: windowChrome.titleBarOverlay.height }}
    />
  );
}
