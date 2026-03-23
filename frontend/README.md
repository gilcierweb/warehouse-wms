# WMS — Frontend (Nuxt.js)

Real-time warehouse addressing control interface.

## Stack

- **Nuxt 4** + **Vue 3** (Composition API + `<script setup>`)
- **TypeScript** — full type safety
- **WebSocket** — real-time updates via Actix-Web
- **IBM Plex Mono** — industrial typography

## Structure

```
frontend/
├── app/
│   ├── assets/css/main.css          # Global design system (dark theme, CSS variables)
│   ├── components/
│   │   ├── SlotGrid.vue             # Renders a street with 3 position lanes
│   │   └── ActionPanel.vue          # Entry/exit/undo panel + slot info
│   ├── composables/
│   │   ├── useWarehouseStore.ts     # Global slots state (Nuxt useState)
│   │   ├── useWarehouseApi.ts       # All REST calls to Rust backend
│   │   ├── useWarehouseWS.ts        # WebSocket connection with auto-reconnect
│   │   └── useAlerts.ts             # Toast system
│   ├── layouts/
│   │   └── default.vue              # Shell: sidebar + WS status + toasts
│   ├── pages/
│   │   ├── index.vue                # Main occupancy map
│   │   ├── history.vue              # History with filters and pagination
│   │   ├── dashboard.vue            # KPIs and street chart
│   │   └── settings.vue             # Alert and API connection settings
│   └── types/index.ts               # TypeScript interfaces (Slot, Movement, etc.)
├── public/                          # Static assets
├── nuxt.config.ts                   # Nuxt configuration
└── package.json
```

## Setup

```bash
# Install dependencies
npm install

# Run in development (points to http://localhost:8080 by default)
npm run dev

# Production build
npm run build
```

## Environment Variables

Create `.env` in the frontend root:

```env
API_BASE=http://localhost:8080
WS_BASE=ws://localhost:8080
```

## Backend Integration (Actix-Web)

The frontend expects the following endpoints from the Rust backend:

| Method | Route                           | Description                  |
|--------|-------------------------------|----------------------------|
| GET    | /api/slots                    | List all slots             |
| POST   | /api/slots/:id/entry          | Register entry             |
| POST   | /api/slots/:id/exit           | Register exit              |
| POST   | /api/movements/undo           | Undo last movement         |
| GET    | /api/movements                | History (with filters)     |
| GET    | /api/stats                    | General statistics         |
| GET    | /api/export/excel             | Download Excel             |
| GET    | /api/export/pdf               | Download PDF               |
| WS     | /ws/live                      | Real-time WebSocket channel|

### WebSocket Format (JSON)

```json
{ "event": "slot_updated",  "payload": { "id": "A-5-N2", "status": "occupied", ... } }
{ "event": "stats_updated", "payload": { "total": 360, "occupied": 120, "pct": 33.3, ... } }
{ "event": "alert",         "payload": { "message": "Capacity 80%", "pct": 80.0 } }
```

## Address Format

`{STREET}-{POSITION}-{LANE}` → e.g.: `D-10-N3`

- Street: A–F (configurable in Settings)
- Position: 1–20 (configurable)
- Lane: N1, N2, N3

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
