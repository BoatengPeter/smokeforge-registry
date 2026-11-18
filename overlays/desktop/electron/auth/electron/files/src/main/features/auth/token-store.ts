import { mkdir, readFile, rename, unlink, writeFile } from "node:fs/promises";
import { dirname } from "node:path";

import { safeStorage } from "electron";

export class TokenStore {
  private refreshToken: string | null = null;

  constructor(private readonly filePath: string) {}

  async load(): Promise<string | null> {
    if (this.refreshToken || !safeStorage.isEncryptionAvailable()) {
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

    if (!safeStorage.isEncryptionAvailable()) {
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
    } catch (error) {
      await unlink(temporary).catch(() => undefined);
      throw error;
    }
  }

  async clear(): Promise<void> {
    this.refreshToken = null;
    await unlink(this.filePath).catch(() => undefined);
  }
}
