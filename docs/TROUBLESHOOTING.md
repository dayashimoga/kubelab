# KubeLab Troubleshooting

## Quick Diagnostic Commands

```bash
./scripts/doctor.ps1   # Check all prerequisites
./scripts/doctor.sh    # Linux/macOS
```

## Symptom → Cause → Fix

### Podman / Containers

| Symptom | Likely Cause | Fix |
|---|---|---|
| `podman-compose` not found | Podman Compose not installed | `pip install podman-compose` or use Docker Compose |
| Container fails to start | Port conflict | `netstat -tlnp` / `Get-NetTCPConnection` and stop conflicting service |
| Image pull timeout | Network/registry issues | Check internet, retry, or use mirror |
| Permission denied on volume | SELinux or rootless Podman | Add `:Z` suffix to volume mounts |

### Build Failures

| Symptom | Likely Cause | Fix |
|---|---|---|
| `cargo build` fails | Missing system dependencies | Install `pkg-config`, `libssl-dev` |
| `pnpm install` fails | Node version mismatch | Use Node 22+ |
| `flutter build` fails | Missing Android SDK | Install via `sdkmanager` or use CI |
| `edition2024` error | Old Rust toolchain | `rustup update stable` |

### Web / API

| Symptom | Likely Cause | Fix |
|---|---|---|
| 404 on API routes | API not running | Check `http://localhost:8080/healthz` |
| CORS error in browser | Mismatched origin | Check API CORS configuration |
| WebSocket disconnects | Idle timeout (30 min) | Reconnect; this is by design |
| JWT expired | Token TTL exceeded | Re-authenticate via login |

### Database / Redis / NATS

| Symptom | Likely Cause | Fix |
|---|---|---|
| Connection refused on 5432 | PostgreSQL not running | Start via compose or check health |
| Redis timeout | Redis not running or OOM | Check Redis container logs |
| NATS connection failed | NATS container not started | Check `http://localhost:8222/varz` |

### Kubernetes / Labs

| Symptom | Likely Cause | Fix |
|---|---|---|
| Lab provisioning fails | No K8s cluster | Run `./scripts/k8s-up.ps1` |
| Namespace stuck terminating | Finalizer deadlock | `kubectl delete ns <ns> --force --grace-period=0` |
| Pod stuck in Pending | Resource limits exceeded | Check LimitRange and node capacity |
| Grading returns `Unavailable` | Cluster not reachable | Verify kubeconfig and cluster status |

### CI Failures

| Symptom | Likely Cause | Fix |
|---|---|---|
| `cargo fmt` diff | Unformatted Rust code | Run `cargo fmt --all` locally |
| `clippy` warnings with `-D warnings` | Lint violations | Fix or add targeted `#[allow]` |
| Flutter `deprecated_member_use` | Deprecated Flutter APIs | Use recommended replacements |
| Missing APK artifact | Android build failed | Check `flutter build apk` logs |
| Node 20 deprecation warning | Old Node version in workflow | Use `node-version: 22` |

### Mobile / Flutter

| Symptom | Likely Cause | Fix |
|---|---|---|
| `flutter analyze` errors | Deprecated APIs or typos | Fix per analyzer output |
| Android build Gradle error | Incompatible Gradle/AGP version | Update `settings.gradle` |
| iOS build fails | Missing Xcode/CocoaPods | Install Xcode + `pod install` |
