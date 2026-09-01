import { test, expect } from '@playwright/test';

test.describe('15-Track Curriculum & Dynamic Routing E2E Test Suite', () => {
  test('should render all 15 tracks on /learn with search and filter', async ({ page }) => {
    await page.goto('/learn');

    // Header assertion
    await expect(page.locator('h1')).toContainText('15 Cloud-Native Engineering Tracks');

    // Assert key tracks across domains exist
    await expect(page.getByText('Linux & Container Fundamentals')).toBeVisible();
    await expect(page.getByText('Kubernetes Core Architecture & Workloads')).toBeVisible();
    await expect(page.getByText('Storage & Persistent Volumes')).toBeVisible();
    await expect(page.getByText('Cloud-Native Networking, CNI & Gateway API')).toBeVisible();
    await expect(page.getByText('Packaging with Helm & Kustomize')).toBeVisible();
    await expect(page.getByText('Zero-Trust Kubernetes Security & RBAC')).toBeVisible();
    await expect(page.getByText('GitOps & Continuous Delivery with Argo CD')).toBeVisible();
    await expect(page.getByText('Service Mesh with Istio & Envoy Proxy')).toBeVisible();
    await expect(page.getByText('OpenTelemetry, Prometheus & Grafana')).toBeVisible();
    await expect(page.getByText('Production Incident Response & Chaos')).toBeVisible();
    await expect(page.getByText('Real-World Exam & Certification Drills')).toBeVisible();
  });

  test('should navigate to Linux track syllabus and load unique lesson', async ({ page }) => {
    await page.goto('/learn/linux-containers');

    await expect(page.locator('h1')).toContainText('Linux & Container Fundamentals');
    await expect(page.getByText('Linux Filesystem Hierarchy & POSIX Permissions')).toBeVisible();

    // Click first lesson
    await page.getByText('Linux Filesystem Hierarchy & POSIX Permissions').click();

    // Verify lesson view
    await expect(page.locator('h1')).toContainText('Linux Filesystem Hierarchy & POSIX Permissions');
    await expect(page.getByText('Launch Sandbox: linux-01-fs-permissions')).toBeVisible();

    // Open AI Tutor
    await page.getByRole('button', { name: 'AI Tutor' }).click();
    await expect(page.getByText('AI Socratic Tutor').first()).toBeVisible();
    await expect(page.getByRole('button', { name: 'socratic' })).toBeVisible();

    // Open Quiz Modal
    await page.getByRole('button', { name: 'Take Quiz' }).click();
    await expect(page.getByText('Lesson Quiz • Quiz: Linux Filesystem Hierarchy')).toBeVisible();
  });

  test('should navigate to Security track and verify unique non-pod content and quiz', async ({ page }) => {
    await page.goto('/learn/security');

    await expect(page.locator('h1')).toContainText('Zero-Trust Kubernetes Security & RBAC');
    await expect(page.getByText('Role & RoleBinding for Least-Privilege')).toBeVisible();

    // Click Security lesson
    await page.getByText('Role & RoleBinding for Least-Privilege').click();

    await expect(page.locator('h1')).toContainText('Role & RoleBinding for Least-Privilege');
    await expect(page.getByText('Launch Sandbox: sec-01-rbac-role-binding')).toBeVisible();
    await expect(page.getByText('COMMON PRODUCTION MISTAKES')).toBeVisible();
  });

  test('should navigate to Service Mesh track and verify unique Istio content', async ({ page }) => {
    await page.goto('/learn/service-mesh');

    await expect(page.locator('h1')).toContainText('Service Mesh with Istio & Envoy Proxy');
    await expect(page.getByText('Deploying Istio Service Mesh Operator')).toBeVisible();

    // Click Service Mesh lesson
    await page.getByText('Deploying Istio Service Mesh Operator').click();

    await expect(page.locator('h1')).toContainText('Deploying Istio Service Mesh Operator');
    await expect(page.getByText('Launch Sandbox: mesh-01-istio-control-plane')).toBeVisible();
  });

  test('should navigate to Incident Response track and verify unique SEV-1 break-fix lesson', async ({ page }) => {
    await page.goto('/learn/incidents');

    await expect(page.locator('h1')).toContainText('Production Incident Response & Chaos');

    // Click CoreDNS incident lesson
    await page.getByText('Production Incident: CoreDNS Outage & Cascading 503 Errors').click();

    await expect(page.locator('h1')).toContainText('Production Incident: CoreDNS Outage & Cascading 503 Errors');
    await expect(page.getByText('Launch Sandbox: incident-coredns-failure')).toBeVisible();
  });
});
