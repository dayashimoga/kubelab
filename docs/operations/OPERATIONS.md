# Operational Runbook & Observability

## Monitoring & Alerting
- Prometheus metrics exposed on `:8080/metrics`
- Grafana dashboards available for active sessions, sandbox provisioning latency, and error rates.

## Backup & Recovery
- PostgreSQL automated daily backups using `pg_dump`
- Redis snapshot persistence enabled (RDB + AOF)
- Stateless lab sandboxes auto-expire and are disposable by design
