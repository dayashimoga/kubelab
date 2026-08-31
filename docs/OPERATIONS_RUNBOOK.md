# KubeLab Operations Runbook

## Startup
```bash
./scripts/up.ps1      # Windows
./scripts/up.sh       # Linux/macOS
```
Starts PostgreSQL, Redis, NATS, API, Web, and observability stack.

## Shutdown
```bash
./scripts/down.ps1    # Graceful stop
./scripts/clean.ps1   # Full cleanup (volumes, images, temp files)
```

## Health Checks
| Service | Check |
|---|---|
| API | `curl http://localhost:8080/healthz` |
| Web | `curl http://localhost:3000` |
| PostgreSQL | `pg_isready -h localhost -p 5432` |
| Redis | `redis-cli ping` |
| NATS | `curl http://localhost:8222/varz` |
| Grafana | `curl http://localhost:3001/api/health` |

## Deploy / Rollback
```bash
# Build new container images
podman-compose -f infrastructure/containers/podman-compose.yml build
# Rolling update
podman-compose -f infrastructure/containers/podman-compose.yml up -d
# Rollback: redeploy previous image tag
podman-compose -f infrastructure/containers/podman-compose.yml up -d --force-recreate
```

## Scaling
- API: Horizontal scaling behind a load balancer (stateless with Redis sessions)
- PostgreSQL: Read replicas for query scaling
- Labs: Scale Kind/k3d cluster node count or add remote cluster pool

## Incident Response
1. Check Grafana dashboards at `http://localhost:3001`
2. Review API logs: `podman logs kubelab-api`
3. Check NATS event flow: `curl http://localhost:8222/connz`
4. Review error rates in Prometheus: `rate(http_requests_total{status=~"5.."}[5m])`

## Backups
```bash
./scripts/backup-restore-test.ps1  # Automated backup/restore verification
```
- RPO: 0 (zero data loss with pg_dump)
- RTO: < 5 seconds

## Key Rotation
1. Generate new JWT secret (min 32 chars)
2. Update `JWT_SECRET` environment variable
3. Restart API service — existing sessions will require re-authentication

## Database Migration
```bash
# Migrations are in services/api/migrations/
# Applied automatically on API startup via sqlx migrate
```

## Emergency Procedures
- **API unresponsive**: `podman restart kubelab-api`
- **Database corruption**: Restore from latest backup
- **Lab cluster stuck**: `./scripts/k8s-down.ps1 && ./scripts/k8s-up.ps1`
- **Full system reset**: `./scripts/clean.ps1 && ./scripts/up.ps1`
