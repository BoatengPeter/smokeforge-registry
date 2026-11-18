import { _electron as electron, expect, test } from "@playwright/test";

const api = process.env.SYNC_E2E_API_URL;
const email = process.env.SYNC_E2E_EMAIL ?? "demo@example.com";
const password = process.env.SYNC_E2E_PASSWORD;

test.skip(!api || !password, "requires a live sync API");

test.setTimeout(60_000);

test("syncs a Library item from Electron to the live API", async () => {
  const app = await electron.launch({
    args: ["."],
    env: { ...process.env, DESKTOP_API_URL: api, DESKTOP_SYNC_ENABLED: "true" },
  });
  const page = await app.firstWindow();

  await page.waitForTimeout(1200);
  const getStarted = page.getByRole("button", { name: "Get started" });
  if (await getStarted.count()) await getStarted.click();

  await page.getByRole("navigation", { name: "System" }).getByRole("link", { name: "Account" }).click();
  await page.getByPlaceholder("Email").fill(email);
  await page.getByPlaceholder("Password").fill(password);
  await page.getByRole("button", { name: "Sign in" }).click();
  await expect(page.getByText("Signed in")).toBeVisible({ timeout: 10_000 });

  const title = `E2E sync ${Date.now()}`;
  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Library" }).click();
  await page.getByRole("button", { name: "New item" }).first().click();
  await page.getByLabel("Title").fill(title);
  await page.getByLabel("Description").fill("Created offline by the Electron E2E.");
  await page.getByRole("button", { name: "Save" }).click();
  await expect(page.getByText(title).first()).toBeVisible({ timeout: 10_000 });

  const login = await fetch(`${api}/api/v1/auth/login`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ email, password }),
  });
  expect(login.ok).toBe(true);
  const tokens = (await login.json()) as { access_token: string };

  await expect.poll(async () => {
    const response = await fetch(`${api}/api/v1/library/snapshot`, {
      headers: { Authorization: `Bearer ${tokens.access_token}` },
    });
    const body = (await response.json()) as { items: Array<{ title: string }> };
    return body.items.some((item) => item.title === title);
  }, { timeout: 20_000 }).toBe(true);

  await app.close();
});
