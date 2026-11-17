import { _electron as electron, expect, test } from "@playwright/test";

test("launches the desktop app and reads system info through IPC", async () => {
  const app = await electron.launch({ args: ["."] });
  const page = await app.firstWindow();

  await expect(page).toHaveTitle(/Verify|Desktop|Smokeforge/i);
  await expect(page.getByTestId("app-version")).not.toHaveText("...");

  await app.close();
});
