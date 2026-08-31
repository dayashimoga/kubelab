import { test, expect } from '@playwright/test';

test.describe('Lab Workspace & Catalog', () => {
  test('should render lab catalog page with all tracks', async ({ page }) => {
    await page.goto('/labs');
    await expect(page.locator('h1')).toContainText('Hands-On Labs');
    await expect(page.locator('text=k8s-pod-basics')).toBeVisible();
  });

  test('should load lab workspace with Monaco and Terminal layout', async ({ page }) => {
    await page.goto('/labs/k8s-pod-basics');
    await expect(page.locator('text=manifest.yaml')).toBeVisible();
    await expect(page.locator('text=kubectl apply -f')).toBeVisible();
    await expect(page.locator('text=SANDBOX SHELL')).toBeVisible();
  });
});
