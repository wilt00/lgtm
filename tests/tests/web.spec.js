const { test, expect } = require("@playwright/test");

const url = process.env.TEST_URL || "http://localhost:5000";

test("test lgtm", async ({ page }) => {
  await page.goto(url);

  const lgtmText = page.locator(".lgtm-text");

  // test lgtm text
  await expect(lgtmText).toHaveText(/^l\w+\s\w+\st\w+\sm\w+$/, {
    timeout: 5000,
  });
});

test("test copy text", async ({ page, context }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto(url);

  const lgtmText = page.locator(".lgtm-text");

  // test copy text
  await page.click('button[data-automation-id="copyText"]');
  await page.waitForFunction(
    () =>
      document.querySelector("div.alert")?.innerHTML === "Copied to clipboard"
  );
  const clipboardText = await page.evaluate(() =>
    navigator.clipboard.readText()
  );
  await expect(lgtmText).toHaveText(clipboardText);
});

test("test copy image", async ({ page, context }) => {
  await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  await page.goto(url);

  const lgtmText = page.locator(".lgtm-text");

  // wait for text to animate
  await expect(lgtmText).toHaveText(/^l\w+\s\w+\st\w+\sm\w+$/, {
    timeout: 5000,
  });

  // test copy image
  await page.click('button[data-automation-id="copyImage"]');
  await page.waitForFunction(
    () =>
      document.querySelector("div.alert")?.innerHTML === "Copied to clipboard"
  );
  const imageType = await page.evaluate(async () => {
    const clips = await navigator.clipboard.read();
    return clips[0].types[0];
  });
  expect(imageType).toBe("image/png");
});

test("test reroll", async ({ page }) => {
  await page.goto(url);

  const lgtmText = page.locator(".lgtm-text");

  // test lgtm text
  await expect(lgtmText).toHaveText(/^l\w+\s\w+\st\w+\sm\w+$/, {
    timeout: 5000,
  });
  const oldText = await lgtmText.innerText();

  // reroll
  await page.click('button[data-automation-id="reroll"]');
  await page.waitForTimeout(100);

  // test lgtm text
  await expect(lgtmText).toHaveText(/^l\w+\s\w+\st\w+\sm\w+$/, {
    timeout: 5000,
  });
  const newText = await lgtmText.innerText();

  expect(oldText !== newText).toBeTruthy();
});
