import type { DesktopApi } from "@shared/types/desktop-api";

declare global {
  interface Window {
    desktopApi: DesktopApi;
  }
}

export {};
