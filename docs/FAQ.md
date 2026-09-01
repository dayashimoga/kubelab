# KubeLab FAQ

## General

**Q: What is KubeLab?**
A: An open-source platform for learning cloud-native engineering through hands-on labs with real Kubernetes clusters.

**Q: Do I need to install Kubernetes on my machine?**
A: No. KubeLab provisions disposable Kind/k3d clusters automatically. You only need Podman or Docker.

**Q: Is KubeLab free?**
A: Yes. KubeLab is Apache 2.0 licensed open-source software.

## Learning

**Q: How many labs are available?**
A: 154 labs across 15 tracks, from Linux basics to multi-cluster disaster recovery.

**Q: Can I use the mobile app for labs?**
A: Labs with terminals require the desktop web app. The mobile app supports lessons, quizzes, and progress tracking with a "Continue on Desktop" handoff for labs.

**Q: How is grading done?**
A: The grading engine queries actual Kubernetes API state using JSONPath assertions. It validates that you've made real changes to the cluster, not just typed commands.

## Technical

**Q: What tech stack is used?**
A: Rust/Axum backend, Next.js/TypeScript frontend, Flutter mobile, PostgreSQL/Redis/NATS data layer.

**Q: Can I contribute?**
A: Yes! See the [Contributing Guide](CONTRIBUTING.md).

**Q: How do I report a bug?**
A: Open a GitHub issue with reproduction steps, expected behavior, and actual behavior.

## Troubleshooting

**Q: The API won't start. What do I check?**
A: Run `./scripts/doctor.ps1` to verify prerequisites. Check that ports 5432, 6379, 4222, and 8080 are available.

**Q: My lab session timed out. Can I recover it?**
A: Lab sessions are ephemeral. Start a new session and your progress on that specific lab resets. Overall XP and lesson completion are preserved.

See [Troubleshooting Guide](TROUBLESHOOTING.md) for more.
