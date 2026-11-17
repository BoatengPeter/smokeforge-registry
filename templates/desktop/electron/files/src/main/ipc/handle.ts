import { BrowserWindow, ipcMain, type IpcMainInvokeEvent } from "electron";
import type { ZodType } from "zod";

import { isTrustedRendererUrl } from "../../shared/security";
import { fail, ok, type Result } from "../../shared/types/result";
import { log } from "../infra/logger";

const devRendererUrl = process.env.ELECTRON_RENDERER_URL;

function trustedSender(event: IpcMainInvokeEvent): boolean {
  const window = BrowserWindow.fromWebContents(event.sender);
  const url = event.senderFrame?.url;

  if (!window || !url) {
    return false;
  }

  return isTrustedRendererUrl(url, devRendererUrl);
}

export function handle<TInput, TOutput>(
  channel: string,
  schema: ZodType<TInput>,
  handler: (input: TInput, event: IpcMainInvokeEvent) => Promise<TOutput> | TOutput,
): void {
  ipcMain.handle(channel, async (event, raw: unknown): Promise<Result<TOutput>> => {
    if (!trustedSender(event)) {
      log("warn", "ipc rejected untrusted sender", { channel, url: event.senderFrame?.url });

      return fail("forbidden_sender", "The request did not come from the app window.");
    }

    const parsed = schema.safeParse(raw);
    if (!parsed.success) {
      const fields = Object.fromEntries(
        Object.entries(parsed.error.flatten().fieldErrors).filter(
          (entry): entry is [string, string[]] => entry[1] !== undefined,
        ),
      );

      return fail("validation_failed", "The IPC request is invalid.", fields);
    }

    try {
      return ok(await handler(parsed.data, event));
    } catch (error) {
      log("error", "ipc handler failed", {
        channel,
        error: error instanceof Error ? error.message : "unknown error",
      });

      return fail("internal_error", "The desktop operation failed.");
    }
  });
}
