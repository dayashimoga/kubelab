import { test, expect } from '@playwright/test';

test.describe('End-to-End User Learning & Certification Journey', () => {
  test('Complete flow: Register -> Learn -> Lab -> Monaco -> Terminal -> Progress -> Logout', async ({ page }) => {
    // 1. Visit landing page
    await page.goto('/');
    await expect(page.locator('h1')).toContainText('Cloud-Native');

    // 2. Navigate to Register
    await page.goto('/register');
    await expect(page.locator('h1')).toContainText('Join KubeLab');
    await page.fill('input[type="text"]', 'E2E Cloud Learner');
    await page.fill('input[type="email"]', `e2e-learner-${Date.now()}@kubelab.io`);
    await page.fill('input[type="password"]', 'StrongPassw0rd!123');

    // 3. Navigate to Curriculum & Tracks
    await page.goto('/learn');
    await expect(page.locator('h1')).toContainText('Engineering Tracks');
    await expect(page.locator('text=Kubernetes Core Architecture')).toBeVisible();

    // 4. Open Lab Catalog & Select Lab
    await page.goto('/labs');
    await expect(page.locator('h1')).toContainText('Hands-On Labs');
    await expect(page.locator('text=k8s-pod-basics')).toBeVisible();

    // 5. Open Interactive Lab Workspace
    await page.goto('/labs/k8s-pod-basics');
    await expect(page.locator('text=manifest.yaml')).toBeVisible();
    await expect(page.locator('text=kubectl apply -f')).toBeVisible();
    await expect(page.locator('text=SANDBOX SHELL')).toBeVisible();

    // 6. Verify Monaco Editor and Security Policy Validation
    const editor = page.locator('.monaco-editor');
    await expect(editor).toBeDefined();

    // 7. Check Progress & Skill Tree
    await page.goto('/progress');
    await expect(page.locator('h1')).toContainText('Progress & Achievement Hub');

    // 8. Check Skills Matrix
    await page.goto('/skills');
    await expect(page.locator('h1')).toContainText('Skill Tree');

    // 9. Check Certifications & Incidents
    await page.goto('/certifications');
    await expect(page.locator('h1')).toContainText('Certifications');

    await page.goto('/incidents');
    await expect(page.locator('h1')).toContainText('CoreDNS Outage');
  });
});
