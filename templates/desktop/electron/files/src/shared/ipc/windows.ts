import { z } from "zod";

import type { Result } from "../types/result";

export const windowChannels = {
  openLibraryItem: "windows:open-library-item",
} as const;

export const windowSchemas = {
  openLibraryItem: z.object({ id: z.string().uuid() }).strict(),
};

export type WindowApi = {
  openLibraryItem: (id: string) => Promise<Result<{ opened: boolean }>>;
};
