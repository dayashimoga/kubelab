# API Reference & Contracts

## Endpoints
- `POST /api/auth/register` - Create new user account
- `POST /api/auth/login` - Authenticate and receive JWT
- `GET /api/tracks` - List curriculum tracks
- `GET /api/labs` - List available lab scenarios
- `POST /api/labs/{id}/start` - Provision sandbox namespace
- `POST /api/labs/{id}/grade` - Evaluate live cluster state
- `GET /ws/terminal/{session_id}` - Authenticated WebSocket PTY terminal
- `GET /metrics` - Prometheus metrics exposition
- `GET /healthz` - Liveness & readiness probes
