const { existsSync } = require("node:fs");
const { join } = require("node:path");
const { FuseV1Options, FuseVersion, flipFuses } = require("@electron/fuses");

module.exports = async function afterPack(context) {
  const productFilename = context.packager.appInfo.productFilename;
  const sanitizedName = context.packager.appInfo.sanitizedName;
  const executableName =
    context.electronPlatformName === "win32" ? `${sanitizedName}.exe` : sanitizedName;
  const executablePath =
    context.electronPlatformName === "darwin"
      ? join(context.appOutDir, `${productFilename}.app`, "Contents", "MacOS", executableName)
      : join(context.appOutDir, executableName);

  if (!existsSync(executablePath)) {
    throw new Error(`Packaged executable not found: ${executablePath}`);
  }

  await flipFuses(executablePath, {
    version: FuseVersion.V1,
    [FuseV1Options.RunAsNode]: false,
    [FuseV1Options.EnableCookieEncryption]: true,
    [FuseV1Options.EnableNodeOptionsEnvironmentVariable]: false,
    [FuseV1Options.EnableNodeCliInspectArguments]: false,
    [FuseV1Options.EnableEmbeddedAsarIntegrityValidation]: true,
    [FuseV1Options.OnlyLoadAppFromAsar]: true,
  });
};
