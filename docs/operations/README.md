# Operations & Runbooks

## Service Management
- **Startup**: `scripts/up.ps1` or `podman compose -f infrastructure/containers/podman-compose.yml up -d`
- **Teardown**: `scripts/down.ps1` or `podman compose -f infrastructure/containers/podman-compose.yml down`
- **Zero-Residue Cleanup**: `scripts/clean.ps1`
- **Disaster Recovery**: `scripts/backup-restore-test.ps1`
- **Cluster Lifecycle**: `scripts/k8s-up.ps1` and `scripts/k8s-down.ps1`
