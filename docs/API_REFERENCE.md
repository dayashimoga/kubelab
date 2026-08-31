# KubeLab API Reference

## Base URL
```
http://localhost:8080
```

## Authentication
All authenticated endpoints require: `Authorization: Bearer <jwt_token>`

## Endpoints

### Auth
| Method | Path | Auth | Description |
|---|---|---|---|
| POST | `/api/auth/register` | No | Register new user |
| POST | `/api/auth/login` | No | Login and receive JWT |

### Learning
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/api/tracks` | Yes | List all curriculum tracks |
| GET | `/api/tracks/{slug}` | Yes | Get track details |
| GET | `/api/lessons/{id}` | Yes | Get lesson content |

### Labs
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/api/labs` | Yes | List available labs |
| POST | `/api/labs/{id}/start` | Yes | Start a lab session |
| POST | `/api/labs/{id}/grade` | Yes | Submit lab for grading |
| DELETE | `/api/labs/{id}/stop` | Yes | Stop and cleanup lab session |

### Progress
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/api/progress` | Yes | Get current user progress |
| GET | `/api/progress/skills` | Yes | Get skill tree state |

### Terminal (WebSocket)
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/ws/terminal/{session_id}` | Yes (JWT in query) | WebSocket terminal connection |

### System
| Method | Path | Auth | Description |
|---|---|---|---|
| GET | `/healthz` | No | Health check |
| GET | `/metrics` | No | Prometheus metrics |
| GET | `/swagger-ui` | No | OpenAPI documentation |

## Request/Response Format
All endpoints use JSON. Example:

```json
// POST /api/auth/register
{
  "email": "learner@example.com",
  "password": "secure_password_123",
  "name": "Cloud Learner"
}

// Response
{
  "token": "eyJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "uuid",
    "email": "learner@example.com",
    "name": "Cloud Learner",
    "role": "learner"
  }
}
```

## Error Responses
```json
{
  "error": "Unauthorized",
  "message": "Invalid or expired token",
  "status": 401
}
```
