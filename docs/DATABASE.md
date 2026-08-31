# KubeLab Database

## Database Engine
PostgreSQL 16 (Alpine) — primary persistent store.

## Schema
Defined in `services/api/migrations/0001_init.sql`.

### Core Tables

| Table | Purpose | Key Columns |
|---|---|---|
| `users` | User accounts | `id UUID PK`, `email UNIQUE`, `password_hash`, `name`, `role`, `created_at` |
| `lessons` | Lesson content | `id UUID PK`, `track_slug`, `title`, `content`, `order_index` |
| `lab_sessions` | Active lab sessions | `id UUID PK`, `user_id FK`, `lab_id`, `namespace`, `status`, `started_at` |
| `progress` | User progress tracking | `user_id FK`, `lesson_id`, `completed_at`, `xp_earned` |
| `quiz_results` | Quiz attempt results | `id UUID PK`, `user_id FK`, `quiz_id`, `score`, `answers JSONB` |

## Connection
```
DATABASE_URL=postgres://kubelab:kubelab_secret_password@127.0.0.1:5432/kubelab
```

## Migrations
- Location: `services/api/migrations/`
- Applied automatically on API startup via `sqlx::migrate!()`
- Run manually: `sqlx migrate run --source services/api/migrations/`

## Backup & Restore
See [Backup, Restore & DR](BACKUP_RESTORE_DR.md).
