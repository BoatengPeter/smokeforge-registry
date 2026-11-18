import { _electron as electron, expect, test } from "@playwright/test";

test.describe.configure({ timeout: 60_000 });

async function dismissOnboarding(page: import("@playwright/test").Page) {
  await page.waitForTimeout(800);
  const button = page.getByRole("button", { name: "Get started" });
  if (await button.count()) await button.click();
}

test("opens the shell, navigates to Library, and edits a record", async () => {
  const app = await electron.launch({ args: ["."] });
  const page = await app.firstWindow();

  await expect(page).toHaveTitle(/Verify|Desktop|Smokeforge|Shell/i);
  await dismissOnboarding(page);

  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Library" }).click();
  await expect(page.getByPlaceholder(/Search title or description/i)).toBeVisible();
  await expect(page.getByText("Field notes").first()).toBeVisible();

  await page.getByText("Field notes").first().click();
  await expect(page.getByLabel("Title")).toHaveValue("Field notes");

  await page.keyboard.press("ControlOrMeta+K");
  await expect(page.getByPlaceholder(/Type a command/i)).toBeVisible();
  await page.keyboard.press("Escape");

  await app.close();
});

test("runs and cancels a utility-process activity job", async () => {
  const app = await electron.launch({ args: ["."] });
  const page = await app.firstWindow();

  await dismissOnboarding(page);
  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Activity" }).click();
  await page.getByRole("button", { name: "Run import job" }).click();
  await expect(page.getByText(/Starting worker|Reading source|Validating rows|Writing records/)).toBeVisible();
  await page.getByRole("button", { name: "Cancel job" }).click();
  await expect(page.getByText("Cancelled", { exact: true }).first()).toBeVisible();

  await app.close();
});

test("persists dashboard card order", async () => {
  const app = await electron.launch({ args: ["."] });
  const page = await app.firstWindow();

  await dismissOnboarding(page);
  await page.evaluate(async () => {
    await window.desktopApi.settings.update({ dashboardOrder: ["active", "total", "archived"] });
  });
  await page.reload();
  await page.waitForTimeout(500);
  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Home" }).click();
  const active = page.getByTestId("stat-active");
  await active.focus();
  await page.keyboard.press("Space");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("Space");
  await page.waitForTimeout(600);
  await page.reload();
  await page.waitForTimeout(600);
  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Home" }).click();

  const cards = page.locator('[data-testid^="stat-"]');
  await expect(cards.first()).not.toHaveAttribute("data-testid", "stat-active");
  await app.close();
});

test("opens a detached Library inspector window", async () => {
  const app = await electron.launch({ args: ["."] });
  const page = await app.firstWindow();

  await dismissOnboarding(page);
  await page.getByRole("navigation", { name: "Workspace" }).getByRole("link", { name: "Library" }).click();
  await page.getByText("Field notes").first().click();
  const [detail] = await Promise.all([
    app.waitForEvent("window"),
    page.getByRole("button", { name: "Open in new window" }).click(),
  ]);
  await expect(detail.getByLabel("Title")).toHaveValue("Field notes");

  await app.close();
});
