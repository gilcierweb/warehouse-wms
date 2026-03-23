# WMS Backend — Rust + Actix-Web + Diesel + PostgreSQL

API REST + WebSocket em tempo real para o sistema de gestão de armazém.

---

## Stack

| Camada        | Tecnologia                              |
|---------------|-----------------------------------------|
| Web framework | Actix-Web 4                             |
| ORM           | Diesel 2 (PostgreSQL)                   |
| Pool          | r2d2                                    |
| Autenticação  | JWT (jsonwebtoken) + bcrypt             |
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

cargo install diesel_cli --no-default-features --features postgres
export DATABASE_URL=postgres://postgres:gil123@localhost:5432/warehouse_wms_development
diesel setup

diesel migration generate create_users
diesel migration generate create_profiles
diesel migration generate create_alert_configs
diesel migration generate create_slots
diesel migration generate create_movements

diesel migration run
diesel migration redo
```

## Estrutura do Projeto

```shell
wms-backend/
├── src/
│   ├── main.rs                  # Bootstrap: HTTP server, CORS, pool, hub
│   ├── config.rs                # Leitura de variáveis de ambiente
│   ├── db.rs                    # Pool r2d2 Diesel/PostgreSQL
│   ├── routes.rs                # Todas as rotas em um lugar
│   ├── schema.rs                # Gerado pelo Diesel CLI
│   ├── errors/
│   │   └── mod.rs               # AppError com ResponseError do Actix
│   ├── models/
│   │   └── mod.rs               # Structs Slot, Movement, User, AlertConfig
│   ├── middleware/
│   │   ├── mod.rs
│   │   └── auth.rs              # Extrator AuthUser (JWT FromRequest)
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── slots.rs             # entry, exit, list, stats
│   │   ├── movements.rs         # histórico, desfazer
│   │   ├── auth.rs              # login, register, me
│   │   └── export.rs            # download Excel
│   └── ws/
│       └── mod.rs               # WsHub (broadcast) + ws_handler
├── migrations/
│   └── 00000000000000_initial/
│       ├── up.sql               # Cria todas as tabelas + seed de slots
│       └── down.sql             # Remove tudo
├── Cargo.toml
├── diesel.toml
├── Dockerfile
├── docker-compose.yml
└── .env.example
```

---

## Setup rápido (desenvolvimento local)

### 1. Pré-requisitos

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Diesel CLI com suporte PostgreSQL
cargo install diesel_cli --no-default-features --features postgres

# PostgreSQL rodando (Docker é o mais fácil)
docker run -d \
  --name wms-postgres \
  -e POSTGRES_USER=wms_user \
  -e POSTGRES_PASSWORD=wms_pass \
  -e POSTGRES_DB=wms_db \
  -p 5432:5432 \
  postgres:16-alpine
```

### 2. Configurar ambiente

```bash
cp .env.example .env
# Edite .env se necessário — os valores padrão já apontam para o Docker acima
```

### 3. Aplicar migrations + seed

```bash
# Cria o banco e aplica migrations (tabelas + 360 slots A-F / N1-N3 / 1-20)
diesel migration run

# (Opcional) Regenerar schema.rs após mudanças no banco
diesel print-schema > src/schema.rs
```

### 4. Criar usuário admin inicial

```bash
curl -s -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","email":"admin@wms.local","password":"Senha@123","role":"admin"}' \
  | jq
```

### 5. Rodar o servidor

```bash
cargo run

# Ou em modo release (muito mais rápido)
cargo run --release
```

O servidor sobe em `http://0.0.0.0:8080`.

---

## Docker Compose (tudo junto)

```bash
# Sobe PostgreSQL + Backend + Frontend em um comando
docker compose up --build

# Rebuild só do backend
docker compose up --build backend
```

---

## API Reference

### Autenticação

Todas as rotas (exceto `/api/auth/login`, `/api/auth/register` e `/health`) exigem header:

```
Authorization: Bearer <token>
```

---

### Auth

| Método | Rota                  | Body                                      | Descrição         |
|--------|-----------------------|-------------------------------------------|-------------------|
| POST   | `/api/auth/register`  | `{username, email, password, role?}`      | Cadastrar usuário |
| POST   | `/api/auth/login`     | `{username, password}`                    | Login → token JWT |
| GET    | `/api/auth/me`        | —                                         | Dados do usuário  |

**Login response:**
```json
{
  "token":    "eyJ...",
  "user_id":  "uuid",
  "username": "joao",
  "role":     "operator"
}
```

---

### Slots

| Método | Rota                      | Query / Body             | Descrição                  |
|--------|---------------------------|--------------------------|----------------------------|
| GET    | `/api/slots`              | `?street=A&status=free`  | Lista todos os slots       |
| GET    | `/api/slots/:id`          | —                        | Detalhe de um slot         |
| POST   | `/api/slots/:id/entry`    | `{sku?, note?}`          | Registrar entrada (→ occupied) |
| POST   | `/api/slots/:id/exit`     | `{note?}`                | Registrar saída (→ free)   |

**Exemplo — entrada:**
```bash
curl -X POST http://localhost:8080/api/slots/A-5-N2/entry \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"sku":"PROD-001","note":"Carga de segunda-feira"}'
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

| Método | Rota          | Descrição                      |
|--------|---------------|--------------------------------|
| GET    | `/api/stats`  | Ocupação geral + por rua       |

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

### Movimentos

| Método | Rota                    | Descrição                          |
|--------|-------------------------|------------------------------------|
| GET    | `/api/movements`        | Histórico com filtros              |
| POST   | `/api/movements/undo`   | Desfazer último movimento do slot  |

**Filtros disponíveis:**
```
?slot_id=A-5-N2&type=entry&from=2026-01-01T00:00:00Z&limit=50&offset=0
```

**Undo body:**
```json
{ "slot_id": "A-5-N2" }
```

---

### Export

| Método | Rota                   | Descrição                                  |
|--------|------------------------|--------------------------------------------|
| GET    | `/api/export/excel`    | Download `.xlsx` com 3 abas (mapa, histórico, resumo) |

---

### WebSocket

```
ws://localhost:8080/ws/live
```

Mensagens recebidas pelo cliente (JSON):

```json
// Slot atualizado
{ "event": "slot_updated",  "payload": { "id": "A-5-N2", "status": "occupied", ... } }

// Stats globais atualizadas
{ "event": "stats_updated", "payload": { "total": 360, "occupied": 120, "pct": 33.3, ... } }

// Alerta de capacidade
{ "event": "alert",         "payload": { "message": "Armazém atingiu 80.0%!", "pct": 80.0 } }
```

---

### Health check

```bash
curl http://localhost:8080/health
# → { "status": "ok", "service": "wms-backend" }
```

---

## Papéis de usuário (roles)

| Role       | Permissões                                   |
|------------|----------------------------------------------|
| `admin`    | Tudo — incluindo gerenciar usuários          |
| `operator` | Entrada, saída, desfazer, consultas          |
| `viewer`   | Somente leitura (GET)                        |

---

## Variáveis de ambiente

| Variável           | Padrão              | Descrição                          |
|--------------------|---------------------|------------------------------------|
| `DATABASE_URL`     | —                   | String de conexão PostgreSQL       |
| `HOST`             | `0.0.0.0`           | Interface de bind do servidor      |
| `PORT`             | `8080`              | Porta TCP                          |
| `JWT_SECRET`       | —                   | Chave secreta para tokens JWT      |
| `JWT_EXPIRY_HOURS` | `8`                 | Validade do token em horas         |
| `ALERT_THRESHOLD`  | `80`                | % de ocupação que dispara alerta   |
| `RUST_LOG`         | `info`              | Nível de log                       |
| `CORS_ORIGINS`     | `http://localhost:3000` | Origins permitidas (vírgula)   |