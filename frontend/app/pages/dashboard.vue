<template>
  <div class="dashboard-page">
    <header class="page-header">
      <h1 class="page-title">DASHBOARD</h1>
      <span class="last-updated">Atualizado: {{ now }}</span>
    </header>

    <div class="dash-body">
      <!-- KPI row -->
      <div class="kpi-row">
        <div class="kpi-card">
          <div class="kpi-label">OCUPAÇÃO GERAL</div>
          <div class="kpi-value" :class="globalPctClass">{{ globalStats.pct.toFixed(1) }}%</div>
          <div class="kpi-sub">{{ globalStats.occupied }} / {{ globalStats.total }} posições</div>
        </div>
        <div class="kpi-card">
          <div class="kpi-label">POSIÇÕES LIVRES</div>
          <div class="kpi-value kpi-green">{{ globalStats.free }}</div>
          <div class="kpi-sub">disponíveis agora</div>
        </div>
        <div class="kpi-card">
          <div class="kpi-label">RUA MAIS CHEIA</div>
          <div class="kpi-value kpi-amber">{{ busiestStreet.name }}</div>
          <div class="kpi-sub">{{ busiestStreet.pct.toFixed(1) }}% ocupada</div>
        </div>
        <div class="kpi-card">
          <div class="kpi-label">MOVIMENTOS HOJE</div>
          <div class="kpi-value kpi-blue">{{ movementsToday }}</div>
          <div class="kpi-sub">entradas + saídas</div>
        </div>
      </div>

      <!-- Bar chart -->
      <div class="chart-card">
        <div class="chart-title">OCUPAÇÃO POR RUA</div>
        <div class="bar-chart">
          <div v-for="s in streets" :key="s.name" class="bar-item">
            <div class="bar-wrap">
              <div
                class="bar-fill"
                :class="s.pct >= 80 ? 'pct-danger' : s.pct >= 50 ? 'pct-warn' : 'pct-ok'"
                :style="{ height: s.pct + '%' }"
                :title="`${s.pct.toFixed(1)}%`"
              >
                <span class="bar-label-top">{{ s.pct.toFixed(0) }}%</span>
              </div>
            </div>
            <div class="bar-name">{{ s.name }}</div>
          </div>
        </div>
      </div>

      <!-- Street detail table -->
      <div class="table-card">
        <div class="chart-title">DETALHE POR RUA</div>
        <table class="detail-table">
          <thead>
            <tr>
              <th>Rua</th>
              <th>Ocupadas</th>
              <th>Livres</th>
              <th>Total</th>
              <th>Ocupação</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="s in streets" :key="s.name">
              <td><strong>{{ s.name }}</strong></td>
              <td class="num-red">{{ s.occupied }}</td>
              <td class="num-green">{{ s.free }}</td>
              <td>{{ s.total }}</td>
              <td>
                <div class="inline-bar-wrap">
                  <div class="inline-bar">
                    <div
                      class="inline-bar-fill"
                      :class="s.pct >= 80 ? 'pct-danger' : s.pct >= 50 ? 'pct-warn' : 'pct-ok'"
                      :style="{ width: s.pct + '%' }"
                    />
                  </div>
                  <span :class="s.pct >= 80 ? 'pct-danger' : s.pct >= 50 ? 'pct-warn' : 'pct-ok'">
                    {{ s.pct.toFixed(1) }}%
                  </span>
                </div>
              </td>
              <td>
                <span class="tag" :class="s.pct >= 80 ? 'tag-red' : s.pct >= 50 ? 'tag-amber' : 'tag-green'">
                  {{ s.pct >= 80 ? 'CRÍTICO' : s.pct >= 50 ? 'ATENÇÃO' : 'NORMAL' }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ 
  layout: 'default',
  middleware: ['auth']
})

const store = useWarehouseStore()
const { streets, globalStats } = store

const now = computed(() =>
  new Intl.DateTimeFormat('pt-BR', { timeStyle: 'medium' }).format(new Date())
)

const globalPctClass = computed(() => {
  if (globalStats.value.pct >= 80) return 'pct-danger'
  if (globalStats.value.pct >= 50) return 'pct-warn'
  return 'pct-ok'
})

const busiestStreet = computed(() =>
  [...streets.value].sort((a, b) => b.pct - a.pct)[0] ?? { name: '—', pct: 0 }
)

const movementsToday = ref(Math.floor(Math.random() * 120) + 40)
</script>

<style scoped>
.dashboard-page { display: flex; flex-direction: column; height: 100vh; }

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-1);
  flex-shrink: 0;
}
.page-title    { font-size: 13px; font-weight: 600; letter-spacing: .16em; }
.last-updated  { font-size: 11px; color: var(--text-3); }

.dash-body { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 14px; }

/* KPIs */
.kpi-row { display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; }
.kpi-card {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 16px;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.kpi-label { font-size: 9px; font-weight: 600; letter-spacing: .12em; color: var(--text-3); }
.kpi-value { font-size: 28px; font-weight: 600; line-height: 1.1; }
.kpi-sub   { font-size: 10px; color: var(--text-3); }
.kpi-green { color: var(--green); }
.kpi-amber { color: var(--amber); }
.kpi-blue  { color: var(--blue);  }
.pct-ok     { color: var(--green); }
.pct-warn   { color: var(--amber); }
.pct-danger { color: var(--red); }

/* Bar chart */
.chart-card, .table-card {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 16px;
}
.chart-title {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: .12em;
  color: var(--text-3);
  margin-bottom: 14px;
}

.bar-chart {
  display: flex;
  align-items: flex-end;
  gap: 8px;
  height: 140px;
}
.bar-item { flex: 1; display: flex; flex-direction: column; align-items: center; gap: 6px; height: 100%; }
.bar-wrap  { flex: 1; width: 100%; display: flex; align-items: flex-end; }
.bar-fill  {
  width: 100%;
  border-radius: 3px 3px 0 0;
  position: relative;
  transition: height .5s;
  min-height: 3px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
}
.bar-fill.pct-ok     { background: var(--green-dim); }
.bar-fill.pct-warn   { background: #78350f; }
.bar-fill.pct-danger { background: var(--red-dim); }
.bar-label-top { font-size: 9px; font-weight: 600; padding-top: 3px; color: var(--text-2); }
.bar-name { font-size: 10px; font-weight: 600; color: var(--text-3); }

/* Table */
.detail-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.detail-table th {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: .1em;
  color: var(--text-3);
  text-align: left;
  padding: 8px 12px;
  border-bottom: 1px solid var(--border);
}
.detail-table td {
  padding: 10px 12px;
  border-bottom: 1px solid var(--border);
  color: var(--text);
}
.detail-table tr:last-child td { border-bottom: none; }
.num-red   { color: var(--red);   font-weight: 600; }
.num-green { color: var(--green); font-weight: 600; }

.inline-bar-wrap { display: flex; align-items: center; gap: 8px; }
.inline-bar { flex: 1; height: 3px; background: var(--bg-3); border-radius: 2px; overflow: hidden; }
.inline-bar-fill { height: 100%; border-radius: 2px; transition: width .4s; }
.inline-bar-fill.pct-ok     { background: var(--green); }
.inline-bar-fill.pct-warn   { background: var(--amber); }
.inline-bar-fill.pct-danger { background: var(--red); }
</style>