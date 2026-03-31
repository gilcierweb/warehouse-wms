# WMS Backend - Rust + Actix-Web + Diesel + PostgreSQL

REST API + Real-time WebSocket for the warehouse management system.

---

## Stack

| Layer         | Technology                              |
|---------------|-----------------------------------------|
| Web framework | Actix-Web 4                             |
| ORM           | Diesel 2 (PostgreSQL)                   |
| Pool          | r2d2                                    |
| Authentication| JWT (jsonwebtoken) + bcrypt             |
| WebSocket     | actix-ws + tokio broadcast channel      |
| Export        | rust_xlsxwriter (Excel .xlsx)           |
| Runtime       | Tokio async                             |

---
```shell
cargo add actix-web
cargo add serde
cargo add actix-cors
cargo add diesel
cargo add diesel --features "postgres"
cargo add chrono
cargo add uuid
cargo add argon2
cargo add r2d2
cargo add rand
cargo add tokio
cargo add futures-util
cargo add serde_json
cargo add actix-ws
cargo add actix-rt
cargo add thiserror
cargo add rust_xlsxwriter
cargo add rust-i18n
cargo add async-trait

cargo install diesel_cli --no-default-features --features postgres
export DATABASE_URL=postgres://username:password@localhost:5432/warehouse_wms_development
diesel setup

diesel migration generate create_users
diesel migration generate create_profiles
diesel migration generate create_alert_configs
diesel migration generate create_slots
diesel migration generate create_movements

diesel migration run
diesel migration redo
```

## Project Structure

```shell
wms-backend/
├── src/
│   ├── main.rs                  # Bootstrap: HTTP server, CORS, pool, hub
│   ├── auth/                    # Authentication logic
│   │   └── mod.rs
│   ├── config/                  # Environment configuration
│   │   └── mod.rs
│   ├── controllers/             # Route handlers
│   │   ├── mod.rs
│   │   ├── auth_controller.rs   # login, register, me
│   │   ├── export_controller.rs # download Excel
│   │   ├── movement_controller.rs # history, undo
│   │   └── slot_controller.rs   # entry, exit, list, stats
│   ├── db/                      # Database pool and connection
│   │   ├── mod.rs
│   │   └── conn.rs
│   ├── errors/                  # Error handling
│   │   └── mod.rs               # AppError with Actix ResponseError
│   ├── middleware/              # JWT auth middleware
│   │   ├── mod.rs
│   │   └── auth.rs              # AuthUser extractor (JWT FromRequest)
│   ├── models/                  # Data models
│   │   ├── mod.rs               # Slot, Movement, User, AlertConfig structs
│   │   ├── alert_config.rs
│   │   ├── movement.rs
│   │   ├── profile.rs
│   │   ├── slot.rs
│   │   └── user.rs
│   ├── repositories/            # Database access layer
│   │   ├── mod.rs
│   │   ├── alert_config_repository.rs
│   │   ├── movement_repository.rs
│   │   ├── slot_repository.rs
│   │   └── user_repository.rs
│   ├── routes/                  # Route definitions
│   │   ├── mod.rs
│   │   └── api.rs
│   └── ws/                      # WebSocket handlers
│       ├── mod.rs               # WsHub (broadcast) + ws_handler
│       └── handler.rs
├── migrations/
│   └── 00000000000000_initial/
│       ├── up.sql               # Creates all tables + slot seed
│       └── down.sql             # Drops everything
├── Cargo.toml
├── diesel.toml
└── .env.example
```

---

## Quick Setup (local development)

### 1. Prerequisites

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Diesel CLI with PostgreSQL support
cargo install diesel_cli --no-default-features --features postgres

# PostgreSQL running (Docker is easiest)
docker run -d \
  --name wms-postgres \
  -e POSTGRES_USER=wms_user \
  -e POSTGRES_PASSWORD=wms_pass \
  -e POSTGRES_DB=wms_db \
  -p 5432:5432 \
  postgres:18-alpine
```

### 2. Configure environment

```bash
cp .env.example .env
# Edit .env if needed — default values already point to the Docker above
```

### 3. Apply migrations + seed

```bash
# Create database and apply migrations (tables + 360 slots A-F / N1-N3 / 1-20)
diesel migration run

# (Optional) Regenerate schema.rs after database changes
diesel print-schema > src/schema.rs
```

### 4. Create initial admin user

```bash
curl -s -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@wms.local","password":"Password@123","role":"admin"}' \
  | jq
```

### 5. Run the server

```bash
cargo run

# Or in release mode (much faster)
cargo run --release
```

Server runs at `http://0.0.0.0:8080`.

---

## Docker Compose (everything together)

```bash
# Start PostgreSQL + Backend + Frontend in one command
docker compose up --build

# Rebuild only backend
docker compose up --build backend
```

---

## API Reference

### Authentication

All routes (except `/api/auth/login`, `/api/auth/register` and `/health`) require header:

```
Authorization: Bearer <token>
```

---

### Auth

| Method | Route                 | Body                                      | Description       |
|--------|-----------------------|-------------------------------------------|-------------------|
| POST   | `/api/auth/register`  | `{username, email, password, role?}`      | Register user     |
| POST   | `/api/auth/login`     | `{username, password}`                    | Login → JWT token |
| GET    | `/api/auth/me`        | —                                         | User data         |

**Login response:**
```json
{
  "token":    "eyJ...",
  "user_id":  "uuid",
  "username": "john",
  "role":     "operator"
}
```

---

### Slots

| Method | Route                     | Query / Body             | Description                     |
|--------|---------------------------|--------------------------|---------------------------------|
| GET    | `/api/slots`              | `?street=A&status=free`  | List all slots                  |
| GET    | `/api/slots/:id`          | —                        | Slot details                    |
| POST   | `/api/slots/:id/entry`    | `{sku?, note?}`          | Register entry (→ occupied)     |
| POST   | `/api/slots/:id/exit`     | `{note?}`                | Register exit (→ free)          |

**Example — entry:**
```bash
curl -X POST http://localhost:8080/api/slots/A-5-N2/entry \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"sku":"PROD-001","note":"Monday shipment"}'
```

**Slot response:**
```json
{
  "id":         "A-5-N2",
  "street":     "A",
  "position":   5,
  "lane":       "N2",
  "status":     "occupied",
  "sku":        "PROD-001",
  "updated_at": "2026-03-22T10:30:00Z"
}
```

---

### Stats

| Method | Route         | Description                      |
|--------|---------------|----------------------------------|
| GET    | `/api/stats`  | Overall occupancy + by street    |

```json
{
  "total":    360,
  "occupied": 120,
  "free":     240,
  "pct":      33.3,
  "streets": [
    { "name": "A", "occupied": 43, "free": 17, "total": 60, "pct": 71.7 }
  ]
}
```

---

### Movements

| Method | Route                   | Description                          |
|--------|-------------------------|--------------------------------------|
| GET    | `/api/movements`        | History with filters                 |
| POST   | `/api/movements/undo`   | Undo last movement for slot          |

**Available filters:**
```
?slot_id=A-5-N2&type=entry&from=2026-01-01T00:00:00Z&limit=50&offset=0
```

**Undo body:**
```json
{ "slot_id": "A-5-N2" }
```

---

### Export

| Method | Route                   | Description                                  |
|--------|------------------------|--------------------------------------------|
| GET    | `/api/export/excel`    | Download `.xlsx` with 3 tabs (map, history, summary) |

---

### WebSocket

```
ws://localhost:8080/ws/live
```

Client messages (JSON):

```json
// Slot updated
{ "event": "slot_updated",  "payload": { "id": "A-5-N2", "status": "occupied", ... } }

// Global stats updated
{ "event": "stats_updated", "payload": { "total": 360, "occupied": 120, "pct": 33.3, ... } }

// Capacity alert
{ "event": "alert",         "payload": { "message": "Warehouse reached 80.0%!", "pct": 80.0 } }
```

---

### Health check

```bash
curl http://localhost:8080/health
# → { "status": "ok", "service": "wms-backend" }
```

---

## User Roles

| Role       | Permissions                                   |
|------------|-----------------------------------------------|
| `admin`    | Everything — including managing users         |
| `operator` | Entry, exit, undo, queries                    |
| `viewer`   | Read-only (GET)                               |

---

## Arquitetura de Repositórios

O projeto utiliza uma arquitetura de repositórios com traits genéricas para reduzir código boilerplate.

### Estrutura

```
repositories/
├── base.rs              # BaseRepo com executor de queries
├── traits/mod.rs        # Trait IRepository<M, N> unificada
├── macros.rs            # Macro impl_crud! para gerar CRUD
└── container.rs         # AppContainer com injeção de dependências
```

### Trait IRepository

A trait `IRepository<M, N>` define o contrato CRUD:
- `M` - Modelo de retorno (ex: `User`)
- `N` - Modelo de entrada para create/update (ex: `NewUser`)

```rust
#[async_trait]
pub trait IRepository<M, N>: Send + Sync where M: 'static, N: 'static {
    async fn all(&self) -> QueryResult<Vec<M>>;
    async fn find(&self, id: &Uuid) -> QueryResult<M>;
    async fn create(&self, item: &N) -> QueryResult<M>;
    async fn update(&self, id: &Uuid, item: &N) -> QueryResult<M>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}
```

### Macro impl_crud!

Para cada repositório, basta definir a trait e usar a macro:

```rust
// repositories/users_repository.rs
#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn all(&self) -> QueryResult<Vec<User>>;
    async fn find(&self, id: &Uuid) -> QueryResult<User>;
    async fn create(&self, item: &NewUser) -> QueryResult<User>;
    async fn update(&self, id: &Uuid, item: &NewUser) -> QueryResult<User>;
    async fn destroy(&self, id: &Uuid) -> QueryResult<usize>;
}

// repositories/mod.rs
impl_crud!(IUserRepository, User, NewUser, users::table);
```

A macro gera automaticamente a implementação para `BaseRepo` e para `IRepository`.

### Controller Genérico

O `generic_controller.rs` fornece funções helpers para os controllers:

```rust
use crate::repositories::traits::IRepository;

// GET /recurso - Listar todos
pub async fn get_all<M, N, R>(repo: &Arc<R>) -> HttpResponse { ... }

// GET /recurso/:id - Buscar por ID
pub async fn get_by_id<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>) -> HttpResponse { ... }

// POST /recurso - Criar
pub async fn create<M, N, R>(repo: &Arc<R>, item: web::Json<N>) -> HttpResponse { ... }

// PUT /recurso/:id - Atualizar
pub async fn update<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>, item: web::Json<N>) -> HttpResponse { ... }

// DELETE /recurso/:id - Deletar
pub async fn delete<M, N, R>(repo: &Arc<R>, id: web::Path<Uuid>) -> HttpResponse { ... }
```

### Exemplo de Controller Simplificado

```rust
use actix_web::{get, web, HttpResponse, Responder};
use uuid::Uuid;
use crate::repositories::container::AppContainer;

#[get("/slots/{id}")]
pub async fn get_slot_by_id(
    container: web::Data<AppContainer>,
    id: web::Path<Uuid>,
) -> impl Responder {
    super::generic_controller::get_by_id(&container.slots, id).await
}

#[post("/slots")]
pub async fn create_slot(
    container: web::Data<AppContainer>,
    slot_request: web::Json<CreateSlotRequest>,
) -> Result<impl Responder, Error> {
    if let Err(e) = slot_request.validate() {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e, "code": "VALIDATION_ERROR"
        })));
    }
    let new_slot: NewSlot = slot_request.into_inner().into();
    Ok(super::generic_controller::create(&container.slots, web::Json(new_slot)).await)
}
```

### Container de Dependencies

O `AppContainer` expõe os repositórios via traits genéricas:

```rust
pub struct AppContainer {
    pub users: Arc<dyn IRepository<User, NewUser>>,
    pub slots: Arc<dyn IRepository<Slot, NewSlot>>,
    pub movements: Arc<dyn IRepository<Movement, NewMovement>>,
    // ...
}
```

---

## Environment Variables

| Variable           | Default             | Description                          |
|--------------------|---------------------|--------------------------------------|
| `DATABASE_URL`     | —                   | PostgreSQL connection string         |
| `HOST`             | `0.0.0.0`           | Server bind interface                |
| `PORT`             | `8080`              | TCP port                             |
| `JWT_SECRET`       | —                   | Secret key for JWT tokens            |
| `JWT_EXPIRY_HOURS` | `8`                 | Token validity in hours              |
| `ALERT_THRESHOLD`  | `80`                | Occupancy % that triggers alert      |
| `RUST_LOG`         | `info`              | Log level                            |
| `CORS_ORIGINS`     | `http://localhost:3000` | Allowed origins (comma separated)  |