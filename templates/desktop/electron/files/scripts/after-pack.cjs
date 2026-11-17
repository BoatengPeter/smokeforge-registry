const { join } = require("node:path");
const { FuseV1Options, FuseVersion, flipFuses } = require("@electron/fuses");

module.exports = async function afterPack(context) {
  const executableName =
    context.electronPlatformName === "win32"
      ? `${context.packager.appInfo.productFilename}.exe`
      : context.packager.appInfo.productFilename;
  const executablePath =
    context.electronPlatformName === "darwin"
      ? join(
          context.appOutDir,
          `${context.packager.appInfo.productFilename}.app`,
          "Contents",
          "MacOS",
          executableName,
        )
      : join(context.appOutDir, executableName);

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
