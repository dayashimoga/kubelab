# KubeLab REST & WebSocket API Specification

## Base URL
- Production API: `https://api.kubelab.io/v1`
- Local Development: `http://localhost:8080/v1`
- WebSocket Gateway: `ws://localhost:8080/v1/ws`

## Authentication
Bearer JWT Token in header:
```http
Authorization: Bearer <jwt_token>
```

## Endpoints Overview

### Health & Telemetry
- `GET /healthz`: Liveness probe
- `GET /readyz`: Readiness probe (PostgreSQL, Redis, NATS checks)
- `GET /metrics`: Prometheus metrics

### Auth & User (`services/auth`)
- `POST /v1/auth/register`: Create new user
- `POST /v1/auth/login`: Authenticate and receive JWT
- `POST /v1/auth/logout`: Invalidate session
- `GET /v1/auth/me`: Current user profile & roles

### Learning (`services/learning`)
- `GET /v1/tracks`: List all 12 curriculum tracks
- `GET /v1/tracks/:slug`: Track details with ordered modules
- `GET /v1/lessons/:id`: Complete lesson with MDX and visualizations

### Assessment (`services/assessment`)
- `GET /v1/lessons/:id/quiz`: Fetch quiz questions for lesson
- `POST /v1/quizzes/submit`: Submit answers and get instant scored results

### Labs & Terminal (`services/labs`)
- `GET /v1/labs`: List declarative labs
- `POST /v1/labs/:id/start`: Launch disposable sandbox session
- `GET /v1/labs/sessions/:sessionId`: Session status & cluster credentials
- `POST /v1/labs/sessions/:sessionId/validate`: Execute state assertions
- `WS /v1/ws/terminal/:sessionId`: Interactive bi-directional xterm.js PTY stream

### Progress & Skill Tree (`services/progress`)
- `GET /v1/progress`: Overall user XP, streak, and level
- `GET /v1/skills/graph`: Competency DAG with mastery levels
