import { expect, test } from '@playwright/test';

test('smoke test', async ({ page }) => {
	await page.goto('/');
	expect(await page.textContent('h1')).toBe('Login');
});

// generated with `npx playwright codegen`
test('test login and logout', async ({ page }) => {
  await page.goto('/');

  // Click input >> nth=0
  await page.locator('input').first().click();

  // Fill input >> nth=0
  await page.locator('input').first().fill('laxa88');

  // Press Tab
  await page.locator('input').first().press('Tab');

  // Fill text=Login Username: Password: Login >> input[type="password"]
  await page.locator('text=Login Username: Password: Login >> input[type="password"]').fill('abc');

  // Click button:has-text("Login")
  await page.locator('button:has-text("Login")').click();

  // Click text=Logout
  await page.locator('text=Logout').click();

  // Click h1:has-text("Login")
  expect(await page.textContent('h1')).toBe('Login');
});
