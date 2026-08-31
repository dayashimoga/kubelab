import { test, expect } from '@playwright/test';

test.describe('Authentication Flows', () => {
  test('should render register page with validation rules', async ({ page }) => {
    await page.goto('/register');
    await expect(page.locator('h1')).toContainText('Create Your Account');
    await expect(page.locator('input[type="email"]')).toBeVisible();
    await expect(page.locator('input[type="password"]')).toBeVisible();
  });

  test('should render login page and navigate to register', async ({ page }) => {
    await page.goto('/login');
    await expect(page.locator('h1')).toContainText('Welcome Back');
    await page.click('text=Register');
    await expect(page).toHaveURL(/.*register/);
  });
});
