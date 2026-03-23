<template>
  <div class="map-page">

    <!-- Top bar -->
    <header class="topbar">
      <div class="topbar-left">
        <h1 class="page-title">MAPA DE OCUPAÇÃO</h1>
        <div class="global-stats">
          <span class="stat-chip">
            <span class="chip-dot chip-dot--red" />
            {{ globalStats.occupied }} ocupados
          </span>
          <span class="stat-chip">
            <span class="chip-dot chip-dot--green" />
            {{ globalStats.free }} livres
          </span>
          <span class="stat-chip stat-chip--pct" :class="globalPctClass">
            {{ globalStats.pct.toFixed(1) }}% total
          </span>
        </div>
      </div>

      <div class="topbar-right">
        <!-- Search / filter -->
        <input v-model="search" type="text" placeholder="Buscar endereço ou SKU..." style="width:220px" />
        <button class="icon-btn" title="Exportar Excel" @click="api.downloadExcel()">↓ EXCEL</button>
        <button class="icon-btn icon-btn--blue" title="Exportar PDF" @click="api.downloadPdf()">↓ PDF</button>
      </div>
    </header>

    <!-- Capacity alert banner -->
    <Transition name="banner">
      <div v-if="showCapacityAlert" class="alert-banner">
        <span class="alert-banner-icon">⚠</span>
        <span>Atenção! O armazém atingiu <strong>{{ globalStats.pct.toFixed(1) }}%</strong> de ocupação — risco de congestionamento operacional.</span>
        <button class="banner-close" @click="showCapacityAlert = false">✕</button>
      </div>
    </Transition>

    <!-- Main grid -->
    <div class="content-grid">

      <!-- Map area -->
      <div class="map-area">
        <div class="streets-grid">
          <SlotGrid
            v-for="street in filteredStreets"
            :key="street.name"
            :street="street"
            :selected-slot="selectedSlot?.id"
            @select="onSelectSlot"
          />
        </div>

        <!-- Global progress bar -->
        <div class="global-bar-wrap">
          <div class="global-bar">
            <div
              class="global-bar-fill"
              :class="globalPctClass"
              :style="{ width: globalStats.pct + '%' }"
            />
          </div>
          <span class="global-bar-label">
            Ocupação Geral: {{ globalStats.pct.toFixed(1) }}% ({{ globalStats.occupied }}/{{ globalStats.total }})
          </span>
        </div>
      </div>

      <!-- Side panel -->
      <aside class="side-panel">
        <ActionPanel :selected-slot="selectedSlot ?? undefined" @done="selectedSlot = null" />

        <!-- Quick stats per street -->
        <div class="street-summary">
          <div class="summary-title">RESUMO POR RUA</div>
          <div v-for="s in streets" :key="s.name" class="summary-row">
            <span class="summary-name">{{ s.name }}</span>
            <div class="summary-bar-wrap">
              <div class="summary-bar">
                <div
                  class="summary-bar-fill"
                  :class="s.pct >= 80 ? 'pct-danger' : s.pct >= 50 ? 'pct-warn' : 'pct-ok'"
                  :style="{ width: s.pct + '%' }"
                />
              </div>
            </div>
            <span class="summary-pct" :class="s.pct >= 80 ? 'pct-danger' : s.pct >= 50 ? 'pct-warn' : 'pct-ok'">
              {{ s.pct.toFixed(0) }}%
            </span>
          </div>
        </div>

        <!-- Legend -->
        <div class="legend">
          <div class="legend-item">
            <span class="legend-swatch swatch-free" /> Livre
          </div>
          <div class="legend-item">
            <span class="legend-swatch swatch-occupied" /> Ocupado
          </div>
          <div class="legend-item">
            <span class="legend-swatch swatch-selected" /> Selecionado
          </div>
        </div>
      </aside>
    </div>

  </div>
</template>

<script setup lang="ts">
import type { Slot } from '~/types'

definePageMeta({ layout: 'default' })

const api = useWarehouseApi()
const store = useWarehouseStore()
const { push } = useAlerts()

const { streets, globalStats, bulkLoad } = store
const search = ref('')
const selectedSlot = ref<Slot | null>(null)
const showCapacityAlert = ref(false)

const globalPctClass = computed(() => {
  if (globalStats.value.pct >= 80) return 'pct-danger'
  if (globalStats.value.pct >= 50) return 'pct-warn'
  return 'pct-ok'
})

const filteredStreets = computed(() => {
  if (!search.value) return streets.value
  const q = search.value.toUpperCase()
  return streets.value.filter(s =>
    s.name.includes(q) ||
    s.slots.some(slot => slot.id.includes(q) || slot.sku?.includes(q))
  )
})

watch(() => globalStats.value.pct, (pct) => {
  if (pct >= 80) showCapacityAlert.value = true
})

function onSelectSlot(slot: Slot) {
  selectedSlot.value = slot
}

// Load initial slots from API
onMounted(async () => {
  try {
    const slots = await api.fetchSlots()
    bulkLoad(slots)
    
    // Inicializar WebSocket
    const ws = useWarehouseWS()
    ws.connect()
  } catch {
    push({ type: 'warning', message: 'API offline — usando dados locais de demonstração' })
  }
})

// Limpar WebSocket ao sair da página
onUnmounted(() => {
  const ws = useWarehouseWS()
  ws.disconnect()
})
</script>

<style scoped>
.map-page { display: flex; flex-direction: column; height: 100vh; }

/* Top bar */
.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-1);
  gap: 16px;
  flex-wrap: wrap;
  flex-shrink: 0;
}
.topbar-left { display: flex; align-items: center; gap: 20px; }
.page-title  { font-size: 13px; font-weight: 600; letter-spacing: .16em; color: var(--text); }
.global-stats { display: flex; gap: 10px; align-items: center; }
.stat-chip {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--text-2);
  font-weight: 500;
}
.stat-chip--pct { font-weight: 600; }
.chip-dot { width: 6px; height: 6px; border-radius: 50%; }
.chip-dot--red   { background: var(--red); }
.chip-dot--green { background: var(--green); }

.topbar-right { display: flex; align-items: center; gap: 8px; }
.icon-btn {
  height: 34px;
  padding: 0 12px;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: .06em;
  background: var(--bg-2);
  border: 1px solid var(--border);
  color: var(--text-2);
}
.icon-btn:hover { color: var(--text); border-color: var(--border-2); }
.icon-btn--blue { color: var(--blue); border-color: #0c4a6e; background: var(--blue-bg); }
.icon-btn--blue:hover { background: #032033; }

/* Alert banner */
.alert-banner {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 20px;
  background: var(--red-bg);
  border-bottom: 1px solid var(--red-dim);
  font-size: 12px;
  color: var(--red);
  animation: slide-in-top .25s;
  flex-shrink: 0;
}
.alert-banner-icon { font-size: 14px; }
.banner-close { background: none; border: none; color: var(--red); font-size: 14px; margin-left: auto; }
.banner-enter-active, .banner-leave-active { transition: all .25s; }
.banner-enter-from, .banner-leave-to { opacity: 0; transform: translateY(-8px); }

/* Content grid */
.content-grid {
  display: grid;
  grid-template-columns: 1fr 280px;
  gap: 0;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

/* Map area */
.map-area {
  overflow-y: auto;
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.streets-grid { display: flex; flex-direction: column; gap: 10px; }

/* Global bar */
.global-bar-wrap {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 0;
  border-top: 1px solid var(--border);
  margin-top: 4px;
}
.global-bar { flex: 1; height: 4px; background: var(--bg-3); border-radius: 2px; overflow: hidden; }
.global-bar-fill { height: 100%; border-radius: 2px; transition: width .5s; }
.global-bar-fill.pct-ok     { background: var(--green); }
.global-bar-fill.pct-warn   { background: var(--amber); }
.global-bar-fill.pct-danger { background: var(--red); }
.global-bar-label { font-size: 11px; font-weight: 500; color: var(--text-2); white-space: nowrap; }

/* Side panel */
.side-panel {
  border-left: 1px solid var(--border);
  overflow-y: auto;
  padding: 14px;
  display: flex;
  flex-direction: column;
  gap: 14px;
  background: var(--bg-1);
}

/* Street summary */
.street-summary {
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 12px;
}
.summary-title {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: .12em;
  color: var(--text-3);
  margin-bottom: 10px;
}
.summary-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 6px;
}
.summary-name  { font-size: 11px; font-weight: 600; color: var(--text-2); width: 16px; }
.summary-bar-wrap { flex: 1; }
.summary-bar { height: 3px; background: var(--bg-3); border-radius: 2px; overflow: hidden; }
.summary-bar-fill { height: 100%; border-radius: 2px; transition: width .4s; }
.summary-bar-fill.pct-ok     { background: var(--green); }
.summary-bar-fill.pct-warn   { background: var(--amber); }
.summary-bar-fill.pct-danger { background: var(--red); }
.summary-pct { font-size: 10px; font-weight: 600; width: 32px; text-align: right; }
.pct-ok     { color: var(--green); }
.pct-warn   { color: var(--amber); }
.pct-danger { color: var(--red); }

/* Legend */
.legend {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}
.legend-item { display: flex; align-items: center; gap: 5px; font-size: 10px; color: var(--text-3); }
.legend-swatch { display: inline-block; width: 16px; height: 12px; border-radius: 2px; border: 1px solid transparent; }
.swatch-free     { background: var(--green-bg); border-color: var(--green-dim); }
.swatch-occupied { background: var(--red-bg);   border-color: var(--red-dim); }
.swatch-selected { background: var(--bg-3); border-color: #fff; }
</style>