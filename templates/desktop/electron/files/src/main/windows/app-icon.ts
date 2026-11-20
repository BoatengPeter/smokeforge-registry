import { app, nativeImage, type NativeImage } from "electron";
import { join } from "node:path";

export function loadAppIcon(): NativeImage | undefined {
  const iconPath = app.isPackaged
    ? join(process.resourcesPath, "icon.png")
    : join(app.getAppPath(), "resources", "icon.png");
  const icon = nativeImage.createFromPath(iconPath);
  return icon.isEmpty() ? undefined : icon;
}
