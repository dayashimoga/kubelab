# KubeLab Backup, Restore & Disaster Recovery

## Backup Strategy
- **PostgreSQL**: Automated `pg_dump` snapshots
- **Redis**: RDB snapshots (sessions are ephemeral — loss is acceptable)
- **NATS**: Stateless event bus — no backup needed
- **Lab data**: Ephemeral namespaces — no backup needed

## Recovery Targets
| Metric | Target | Proven |
|---|---|---|
| RPO (Recovery Point Objective) | 0 (zero data loss) | ✅ Via `pg_dump` |
| RTO (Recovery Time Objective) | < 5 seconds | ✅ Via automated restore |

## Backup Procedure
```bash
# Automated backup + restore verification
./scripts/backup-restore-test.ps1

# Manual PostgreSQL backup
pg_dump -h localhost -U kubelab -d kubelab > backup.sql

# Restore
psql -h localhost -U kubelab -d kubelab < backup.sql
```

## Disaster Recovery
1. **Database failure**: Restore from latest `pg_dump` backup
2. **Redis failure**: Restart — sessions regenerate on next login
3. **NATS failure**: Restart — events are fire-and-forget
4. **Full cluster loss**: `./scripts/clean.ps1 && ./scripts/up.ps1`
5. **Lab cluster corruption**: `./scripts/k8s-down.ps1 && ./scripts/k8s-up.ps1`
