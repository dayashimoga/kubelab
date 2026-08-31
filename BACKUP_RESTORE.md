# KubeLab — Disaster Recovery & Backup / Restore Guide

## 1. Backup Strategy & RPO/RTO Targets
- **Recovery Point Objective (RPO)**: < 1 hour (PostgreSQL continuous WAL / hourly snapshots).
- **Recovery Time Objective (RTO)**: < 15 minutes to fully restore database, Redis caches, and cluster orchestrators.

---

## 2. PostgreSQL Backup & Verification

### Automated Backup Execution
```bash
# Export PostgreSQL database dump
podman exec -t kubelab-postgres pg_dump -U kubelab -d kubelab -F c -b -v -f /tmp/kubelab_backup.dump
podman cp kubelab-postgres:/tmp/kubelab_backup.dump ./backups/kubelab_$(date +%Y%m%d_%H%M%S).dump
```

### Restore & Verification
```bash
# Restore PostgreSQL from backup dump
podman exec -i kubelab-postgres pg_restore -U kubelab -d kubelab --clean --if-exists /tmp/kubelab_backup.dump
```

---

## 3. Disaster Recovery Scenario: Full Outage
1. Run `./scripts/clean.sh` to purge all degraded resources.
2. Run `./scripts/up.sh` to spin up fresh backing infrastructure.
3. Apply database restore script.
4. Execute `./scripts/test-containerized.sh` to verify full platform integrity and data consistency.
