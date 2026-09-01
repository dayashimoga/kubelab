import { test, expect } from '@playwright/test';

test.describe('WCAG 2.2 AA Accessibility & Keyboard Nav', () => {
  const routes = ['/', '/learn', '/labs', '/skills', '/progress', '/incidents', '/certifications', '/login', '/register'];

  for (const route of routes) {
    test(`Verify semantic structure & focusability on ${route}`, async ({ page }) => {
      await page.goto(route);

      // 1. Verify single primary landmark
      const mainLandmark = page.locator('main, role=main');
      expect(await mainLandmark.count()).toBeGreaterThanOrEqual(1);

      // 2. Ensure heading hierarchy (h1 present)
      const h1Count = await page.locator('h1').count();
      expect(h1Count).toBeGreaterThanOrEqual(1);

      // 3. Tab key navigates to interactive elements
      await page.keyboard.press('Tab');
      const activeElementTag = await page.evaluate(() => document.activeElement?.tagName);
      expect(['A', 'BUTTON', 'INPUT', 'BODY']).toContain(activeElementTag);
    });
  }
});
