import { mkdir, readFile, rename, unlink, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

import { safeStorage } from "electron";

export class TokenStore {
  private refreshToken: string | null = null;

  constructor(private readonly filePath: string) {}

  async load(): Promise<string | null> {
    if (this.refreshToken || !(await safeStorage.isAsyncEncryptionAvailable())) {
      return this.refreshToken;
    }

    try {
      const encoded = await readFile(this.filePath, "utf8");
      const encrypted = Buffer.from(encoded.trim(), "base64");
      this.refreshToken = safeStorage.decryptString(encrypted);
      return this.refreshToken;
    } catch {
      return null;
    }
  }

  async set(refreshToken: string): Promise<void> {
    this.refreshToken = refreshToken;

    if (!(await safeStorage.isAsyncEncryptionAvailable())) {
      return;
    }

    await mkdir(dirname(this.filePath), { recursive: true, mode: 0o700 });

    const temporary = `${this.filePath}.${process.pid}.tmp`;

    try {
      const encrypted = safeStorage.encryptString(refreshToken);
      await writeFile(temporary, encrypted.toString("base64"), {
        encoding: "utf8",
        mode: 0o600,
      });
      await rename(temporary, this.filePath);
    } catch {
      // Keep the refresh token in memory when the OS keychain is unavailable
      // (for example headless Linux CI). It will not survive a restart.
      await unlink(temporary).catch(() => undefined);
    }
  }

  async clear(): Promise<void> {
    this.refreshToken = null;
    await unlink(this.filePath).catch(() => undefined);
  }
}
