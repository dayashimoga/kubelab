import { test, expect } from '@playwright/test';

test.describe('Lab Workspace & Catalog', () => {
  test('should render lab catalog page with all tracks', async ({ page }) => {
    await page.goto('/labs');
    await expect(page.locator('h1')).toContainText('Hands-On');
    await expect(page.locator('text=k8s-pod-basics')).toBeVisible();
  });

  test('should load lab workspace with Monaco and Terminal layout', async ({ page }) => {
    await page.goto('/labs/k8s-pod-basics');
    await expect(page.locator('text=Terminal')).toBeVisible();
    await expect(page.locator('text=YAML Editor')).toBeVisible();
    await expect(page.locator('text=Validate & Grade Submission')).toBeVisible();
  });
});
