import { test, expect } from '@playwright/test';

test.describe('Live Terminal Component', () => {
  test('should render terminal shell container in incident room', async ({ page }) => {
    await page.goto('/incidents');
    await page.click('text=Triage Terminal');
    await expect(page.locator('text=sandbox-shell')).toBeVisible();
  });
});
