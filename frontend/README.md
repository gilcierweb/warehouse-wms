# WMS — Frontend (Nuxt.js)

Interface de controle de endereçamento do armazém em tempo real.

## Stack

- **Nuxt 4** + **Vue 3** (Composition API + `<script setup>`)
- **TypeScript** — tipagem completa
- **WebSocket** — atualizações em tempo real via Actix-Web
- **IBM Plex Mono** — tipografia industrial

## Estrutura

```
frontend/
├── assets/css/main.css          # Design system global (dark theme, variáveis CSS)
├── components/
│   ├── SlotGrid.vue             # Renderiza uma rua com 3 corredores de posições
│   └── ActionPanel.vue          # Painel entrada/saída/desfazer + info do slot
├── composables/
│   ├── useWarehouseStore.ts     # Estado global dos slots (useState Nuxt)
│   ├── useWarehouseApi.ts       # Todas as chamadas REST ao backend Rust
│   ├── useWarehouseWS.ts        # Conexão WebSocket com reconexão automática
│   └── useAlerts.ts             # Sistema de toasts
├── layouts/
│   └── default.vue              # Shell: sidebar + status WS + toasts
├── pages/
│   ├── index.vue                # Mapa de ocupação principal
│   ├── history.vue              # Histórico com filtros e paginação
│   ├── dashboard.vue            # KPIs e gráfico por rua
│   └── settings.vue             # Configuração de alertas e conexão API
├── types/index.ts               # Interfaces TypeScript (Slot, Movement, etc.)
└── nuxt.config.ts               # Configuração Nuxt
```

## Setup

```bash
# Instalar dependências
npm install

# Rodar em desenvolvimento (aponta para http://localhost:8080 por padrão)
npm run dev

# Build de produção
npm run build
```

## Variáveis de ambiente

Crie `.env` na raiz do frontend:

```env
API_BASE=http://localhost:8080
WS_BASE=ws://localhost:8080
```

## Integração com backend (Actix-Web)

O frontend espera os seguintes endpoints do backend Rust:

| Método | Rota                           | Descrição                  |
|--------|-------------------------------|----------------------------|
| GET    | /api/slots                    | Lista todos os slots       |
| POST   | /api/slots/:id/entry          | Registrar entrada          |
| POST   | /api/slots/:id/exit           | Registrar saída            |
| POST   | /api/movements/undo           | Desfazer último movimento  |
| GET    | /api/movements                | Histórico (com filtros)    |
| GET    | /api/stats                    | Estatísticas gerais        |
| GET    | /api/export/excel             | Download Excel             |
| GET    | /api/export/pdf               | Download PDF               |
| WS     | /ws/live                      | Canal WebSocket tempo real |

### Formato WebSocket (JSON)

```json
{ "event": "slot_updated",  "payload": { "id": "A-5-N2", "status": "occupied", ... } }
{ "event": "stats_updated", "payload": { "total": 360, "occupied": 120, "pct": 33.3, ... } }
{ "event": "alert",         "payload": { "message": "Capacidade 80%", "pct": 80.0 } }
```

## Formato de endereço

`{RUA}-{POSIÇÃO}-{CORREDOR}` → ex: `D-10-N3`

- Rua: A–F (configurável em Settings)
- Posição: 1–20 (configurável)
- Corredor: N1, N2, N3

# Nuxt Minimal Starter

Look at the [Nuxt documentation](https://nuxt.com/docs/getting-started/introduction) to learn more.

## Setup

Make sure to install dependencies:

```bash
# npm
npm install

# pnpm
pnpm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# npm
npm run dev

# pnpm
pnpm dev

# yarn
yarn dev

# bun
bun run dev
```

## Production

Build the application for production:

```bash
# npm
npm run build

# pnpm
pnpm build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# npm
npm run preview

# pnpm
pnpm preview

# yarn
yarn preview

# bun
bun run preview
```

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.
