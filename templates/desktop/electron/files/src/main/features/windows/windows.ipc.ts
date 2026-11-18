import { windowChannels, windowSchemas } from "../../../shared/ipc/windows";
import { handle } from "../../ipc/handle";
import { openLibraryItemWindow } from "../../windows/detail-window";

export function registerWindowHandlers(): void {
  handle(windowChannels.openLibraryItem, windowSchemas.openLibraryItem, (input) => {
    openLibraryItemWindow(input.id);

    return { opened: true };
  });
}
