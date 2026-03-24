# Warehouse WMS - Warehouse Management System

Complete Warehouse Management System (WMS) with Rust backend (Actix-Web) and Nuxt.js frontend. Slot management, entry/exit operations, Excel reports, and WebSocket for real-time updates.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Warehouse WMS                             │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐         ┌─────────────┐                    │
│  │  Frontend   │         │   Backend   │                    │
│  │  Nuxt.js 4  │◄───────►│  Actix-Web  │                    │
│  │   Vue 3     │  HTTP   │    Rust     │                    │
│  │  WS Client  │◄───────►│  WS Server  │                    │
│  └─────────────┘         └──────┬──────┘                    │
│        :3000                    │                           │
│                                 │                           │
│                                 ▼                           │
│                          ┌─────────────┐                    │
│                          │ PostgreSQL  │                    │
│                          │   :5432     │                    │
│                          └─────────────┘                    │
└─────────────────────────────────────────────────────────────┘
```

---

## Technology Stack

### Backend (`/backend`)
- **Language**: Rust (Edition 2024)
- **Web Framework**: Actix-Web 4.13
- **ORM**: Diesel 2 with PostgreSQL
- **Authentication**: JWT (jsonwebtoken) + Argon2
- **WebSocket**: actix-ws + tokio broadcast
- **Export**: rust_xlsxwriter (Excel .xlsx)
- **Connection Pool**: r2d2

### Frontend (`/frontend`)
- **Framework**: Nuxt.js 4.4.2
- **UI**: Vue 3 + Vue Router 5
- **Styling**: Tailwind CSS
- **Runtime**: Node.js 20+

### Infrastructure
- **Database**: PostgreSQL 18 (Alpine)
- **Containerization**: Docker + Docker Compose
- **Base Images**: Alpine Linux (lightweight and secure)

---

## Project Structure

```
warehouse-wms/
├── backend/                    # Rust API + Actix-Web
│   ├── src/
│   │   ├── main.rs            # Server bootstrap
│   │   ├── auth/              # Authentication logic
│   │   ├── config/            # Environment configuration
│   │   ├── controllers/       # Route handlers (slots, movements, auth, export)
│   │   ├── db/                # Database pool and connection
│   │   ├── errors/            # Error handling
│   │   ├── middleware/        # JWT auth middleware
│   │   ├── models/            # Data models (User, Slot, Movement, etc.)
│   │   ├── repositories/      # Database access layer
│   │   ├── routes/            # Route definitions
│   │   └── ws/                # WebSocket handlers
│   ├── migrations/            # Diesel migrations
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/                  # Nuxt.js 4 SPA
│   ├── app/                   # Nuxt app directory
│   │   ├── app.vue            # Root component
│   │   ├── assets/            # CSS and assets
│   │   ├── components/        # Vue components
│   │   ├── composables/       # Vue composables
│   │   ├── layouts/           # Page layouts
│   │   ├── pages/             # Route pages
│   │   └── types/             # TypeScript types
│   ├── public/                # Static assets
│   ├── nuxt.config.ts
│   ├── package.json
│   └── Dockerfile
├── docs/                      # Documentation and mockups
├── docker-compose.yml         # Full orchestration
├── Dockerfile.backend         # Multi-stage Rust build
├── Dockerfile.frontend        # Multi-stage Nuxt build
└── README.md                  # This file
```

---

## Features

### Core WMS
- **Slot Management**: 360 slots organized in streets (A-F), positions (1-20), and levels (N1-N3)
- **Goods Entry**: Slot occupation with SKU and notes
- **Goods Exit**: Slot release with historical record
- **Movement History**: Complete audit trail with filters
- **Operation Undo**: Revert last movement per slot

### Reports and Export
- **Real-time Dashboard**: Occupancy statistics by street
- **Excel Export**: 3 tabs (slot map, history, summary)
- **Capacity Alerts**: Notification when configurable threshold is reached

### Authentication and Authorization
- **JWT Token**: Stateless authentication
- **Roles**: admin, operator, viewer
- **Secure WebSocket**: Authenticated connection for real-time updates

---

## Getting Started

### Prerequisites
- Docker 24+ and Docker Compose
- (Optional) Rust 1.85+ for local development
- (Optional) Node.js 20+ for local development

### Run with Docker Compose (Recommended)

```bash
# Clone the repository
git clone <repo-url>
cd warehouse-wms

# Start the entire stack
docker compose up --build

# Or in detached mode
docker compose up --build -d
```

Available services:
- **Frontend**: http://localhost:3000
- **Backend API**: http://localhost:8080
- **WebSocket**: ws://localhost:8080/ws/live
- **PostgreSQL**: localhost:5432

### Local Development

#### Backend
```bash
cd backend

# Install Diesel CLI
cargo install diesel_cli --no-default-features --features postgres

# Configure database
export DATABASE_URL=postgres://username:password@localhost:5432/warehouse_wms_development

# Run migrations
diesel migration run

# Start server
cargo run --release
```

#### Frontend
```bash
cd frontend

# Install dependencies
npm install

# Start in development mode
npm run dev

pnpm add tailwindcss @tailwindcss/vite
```

---

## API Reference

### Authentication
All routes (except login/register) require header:
```
Authorization: Bearer <token>
```

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/auth/register` | Register user |
| POST | `/api/auth/login` | Login and get token |
| GET | `/api/auth/me` | Logged user data |
| GET | `/api/slots` | List all slots |
| GET | `/api/slots/:id` | Slot details |
| POST | `/api/slots/:id/entry` | Register entry |
| POST | `/api/slots/:id/exit` | Register exit |
| GET | `/api/stats` | Occupancy statistics |
| GET | `/api/movements` | Movement history |
| POST | `/api/movements/undo` | Undo last movement |
| GET | `/api/export/excel` | Download Excel report |
| GET | `/health` | Health check |
| WS | `/ws/live` | WebSocket for real-time updates |

### WebSocket Events

```json
// Slot updated
{ "event": "slot_updated", "payload": { "id": "A-5-N2", "status": "occupied", ... } }

// Stats updated
{ "event": "stats_updated", "payload": { "total": 360, "occupied": 120, ... } }

// Capacity alert
{ "event": "alert", "payload": { "message": "Warehouse reached 80%", "pct": 80.0 } }
```

---

## Environment Variables

### Backend
| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | — | PostgreSQL connection string |
| `HOST` | `0.0.0.0` | Bind interface |
| `PORT` | `8080` | HTTP port |
| `JWT_SECRET` | — | JWT secret key |
| `JWT_EXPIRY_HOURS` | `8` | Token validity in hours |
| `ALERT_THRESHOLD` | `80` | % for capacity alert |
| `RUST_LOG` | `info` | Log level |
| `CORS_ORIGINS` | `http://localhost:3000` | Allowed CORS origins |

### Frontend
| Variable | Default | Description |
|----------|---------|-------------|
| `API_BASE` | `http://localhost:8080` | API base URL |
| `WS_BASE` | `ws://localhost:8080` | WebSocket base URL |
| `NUXT_PORT` | `3000` | Nuxt server port |

---

## Useful Commands

```bash
# View logs
docker compose logs -f backend
docker compose logs -f frontend

# Rebuild only backend
docker compose up --build backend

# Run migrations
docker compose exec backend diesel migration run

# Shell into container
docker compose exec backend sh
docker compose exec frontend sh

# Stop everything
docker compose down

# Clean volumes (⚠️ loses data)
docker compose down -v
```

---

## Test Scripts

```bash
# Create admin user
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@wms.local","password":"Password@123","role":"admin"}'

# Login
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"Password@123"}'

# List slots
curl http://localhost:8080/api/slots \
  -H "Authorization: Bearer <token>"
```

---

## License

MIT License - See LICENSE file for details.

---

## Authors

- **GilcierWeb** - *Development* - [gilcierweb.com.br](https://gilcierweb.com.br)
