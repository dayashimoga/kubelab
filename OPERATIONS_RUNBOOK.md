# KubeLab — Operations & SRE Runbook

## 1. System Health Checks & Telemetry Verification

### 1.1. Health Endpoints
- **Liveness probe**: `GET /healthz` -> `200 OK` `{"status":"healthy","service":"kubelab-api"}`
- **Readiness probe**: `GET /readyz` -> `200 OK` `{"status":"ready","postgres":"ok","redis":"ok","nats":"ok","kubernetes":"ok"}`
- **Prometheus Metrics**: `GET /metrics` -> Exposes HTTP request rates, active sandbox counts, latency histograms.

---

## 2. Common Alerts & Incident Triage Procedures

### Alert: `HighSandboxProvisionLatency`
- **Symptom**: Sandbox creation takes > 5000ms.
- **Diagnosis**: Check Kubernetes cluster node capacity, API server responsiveness, or Podman resource limits.
- **Resolution**:
  ```powershell
  # Check node status
  kubectl get nodes
  # Check sandbox namespaces
  kubectl get namespaces | Select-String "sandbox-"
  ```

### Alert: `PostgreSqlConnectionExhaustion`
- **Symptom**: 500 errors on registration/login with DB pool timeout.
- **Diagnosis**: Check active PostgreSQL connections via `pg_stat_activity`.
- **Resolution**: Restart database or scale connection pool limit in `DATABASE_URL`.

### Alert: `RedisMemoryPressure`
- **Symptom**: Session token verification latency spikes or evictions.
- **Diagnosis**: Check Redis info memory and key expiration policies.
- **Resolution**: Ensure session store keys have appropriate TTLs (86400s default).

---

## 3. Routine Maintenance & Zero-Residue Cleanup
Run daily/weekly cleanup tasks:
```powershell
.\scripts\clean.ps1
```
Asserts:
- Active test containers = 0
- Orphan temporary volumes = 0
- Stale lab namespaces = 0
