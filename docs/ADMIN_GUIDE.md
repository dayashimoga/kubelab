# KubeLab Admin & Instructor Guide

## Admin Guide

### Platform Operations
- **Start platform**: `./scripts/up.ps1` (Windows) or `./scripts/up.sh` (Linux/macOS)
- **Stop platform**: `./scripts/down.ps1` / `./scripts/down.sh`
- **Health check**: `./scripts/doctor.ps1` / `./scripts/doctor.sh`
- **Full cleanup**: `./scripts/clean.ps1` / `./scripts/clean.sh`

### User Management
- Users are stored in PostgreSQL `users` table
- Roles: `learner`, `instructor`, `admin`
- Role changes require direct database update or API admin endpoint
- Password resets use Argon2 re-hashing

### Monitoring
- **Grafana** at `http://localhost:3001` — dashboards for API latency, lab provisioning, and system health
- **Prometheus** at `http://localhost:9090` — raw metrics queries
- **API metrics** at `http://localhost:8080/metrics` — Prometheus exposition format

### Database Management
- Migrations in `services/api/migrations/`
- Backup: `./scripts/backup-restore-test.ps1` demonstrates pg_dump/restore cycle
- See [Backup & Restore](BACKUP_RESTORE_DR.md) for operational procedures

### Lab Cluster Management
- Spin up a disposable Kind cluster: `./scripts/k8s-up.ps1` / `./scripts/k8s-up.sh`
- Tear down with zero residue: `./scripts/k8s-down.ps1` / `./scripts/k8s-down.sh`
- Clusters are ephemeral — destroyed after each lab session or test run

## Instructor Guide

### Creating Lab Content
See [Lab Authoring Guide](LAB_AUTHORING.md) for detailed instructions.

### Reviewing Progress
- Query the progress service API for per-learner completion data
- Access quiz results and lab scores via the admin API
- Skill tree data shows competency across all domains

### Curriculum Management
- Curriculum content defined in `packages/curriculum/`
- Track/lesson/module structure in `services/learning/src/service.rs`
- Lab definitions in `labs/` directory (YAML)
