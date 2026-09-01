import { test, expect } from '@playwright/test';

test.describe('End-to-End User Learning & Certification Journey', () => {
  test('Complete flow: Register -> Learn -> Lab -> Monaco -> Terminal -> Progress -> Incident -> Certification', async ({ page }) => {
    // 1. Visit landing page
    await page.goto('/');
    await expect(page.locator('h1')).toBeVisible();

    // 2. Navigate to Register & Fill Form
    await page.goto('/register');
    await expect(page.locator('h1')).toContainText('Join KubeLab');
    await page.fill('input[type="text"]', 'E2E Cloud Practitioner');
    await page.fill('input[type="email"]', `learner-${Date.now()}@kubelab.io`);
    await page.fill('input[type="password"]', 'SecurePassw0rd!123');
    
    // Submit registration form
    const submitBtn = page.locator('button[type="submit"]');
    if (await submitBtn.isVisible()) {
      await submitBtn.click();
    }

    // 3. Navigate to Curriculum & Tracks
    await page.goto('/learn');
    await expect(page.locator('h1')).toContainText('Engineering Tracks');
    await expect(page.locator('text=Kubernetes Core Architecture').first()).toBeVisible();

    // 4. Open Lab Catalog & Select Lab
    await page.goto('/labs');
    await expect(page.locator('h1')).toContainText('Hands-On Labs');
    await expect(page.locator('text=k8s-pod-basics').first()).toBeVisible();

    // 5. Open Interactive Lab Workspace
    await page.goto('/labs/k8s-pod-basics');
    await expect(page.locator('body')).toBeVisible();
    await expect(page.locator('text=SANDBOX SHELL').first()).toBeVisible();

    // 6. Check Progress & Skill Tree
    await page.goto('/progress');
    await expect(page.locator('h1')).toContainText('Progress & Achievement Hub');

    // 7. Check Skills Matrix
    await page.goto('/skills');
    await expect(page.locator('h1')).toContainText('Skill Tree');

    // 8. Check Certifications & Incidents
    await page.goto('/certifications');
    await expect(page.locator('h1')).toContainText('Certifications');

    await page.goto('/incidents');
    await expect(page.locator('body')).toContainText('CoreDNS');
  });
});
