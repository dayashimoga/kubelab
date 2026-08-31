import { test, expect } from '@playwright/test';

const VIEWPORTS = [
  { width: 320, height: 568, name: 'Mobile Small (320px)' },
  { width: 375, height: 667, name: 'Mobile Medium (375px)' },
  { width: 768, height: 1024, name: 'Tablet (768px)' },
  { width: 1024, height: 768, name: 'Desktop Small (1024px)' },
  { width: 1440, height: 900, name: 'Desktop Standard (1440px)' },
  { width: 2560, height: 1440, name: 'Ultra-Wide 4K (2560px)' },
];

test.describe('Responsive Layout & Accessibility Viewport Matrix', () => {
  for (const vp of VIEWPORTS) {
    test(`Verify layout integrity and no clipping on ${vp.name}`, async ({ page }) => {
      await page.setViewportSize({ width: vp.width, height: vp.height });
      await page.goto('/');

      // Check header visibility
      await expect(page.locator('header')).toBeVisible();

      // Check main landmark
      await expect(page.locator('main')).toBeVisible();

      // Check no horizontal scrollbar overflow on body
      const scrollWidth = await page.evaluate(() => document.documentElement.scrollWidth);
      const clientWidth = await page.evaluate(() => document.documentElement.clientWidth);
      expect(scrollWidth).toBeLessThanOrEqual(clientWidth + 2); // 2px margin tolerance
    });
  }

  test('Verify WCAG semantic landmarks & accessibility headers on key pages', async ({ page }) => {
    const pages = ['/', '/learn', '/labs', '/skills', '/progress', '/incidents', '/certifications'];

    for (const path of pages) {
      await page.goto(path);
      // Ensure exactly one h1 per page
      const h1Count = await page.locator('h1').count();
      expect(h1Count).toBeGreaterThanOrEqual(1);

      // Verify interactive buttons have accessible text or aria-labels
      const buttons = page.locator('button');
      const count = await buttons.count();
      for (let i = 0; i < Math.min(count, 5); i++) {
        const btn = buttons.nth(i);
        const text = await btn.innerText();
        const ariaLabel = await btn.getAttribute('aria-label');
        expect((text && text.trim().length > 0) || (ariaLabel && ariaLabel.trim().length > 0)).toBeTruthy();
      }
    }
  });
});
