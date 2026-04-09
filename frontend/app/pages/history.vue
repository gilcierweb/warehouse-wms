<template>
  <div class="history-page">
    <header class="page-header">
      <h1 class="page-title">{{ $t('history.title') }}</h1>
      <div class="header-actions">
        <button class="icon-btn" @click="api.downloadExcel()">↓ EXCEL</button>
      </div>
    </header>

    <!-- Filters -->
    <div class="filters">
      <input v-model="filters.slotId" type="text" :placeholder="$t('history.slot')" style="width:160px;text-transform:uppercase" @input="filters.slotId = filters.slotId.toUpperCase()" />
      <select v-model="filters.type">
        <option value="">{{ $t('history.typeAll') }}</option>
        <option value="entry">{{ $t('history.typeEntry') }}</option>
        <option value="exit">{{ $t('history.typeExit') }}</option>
      </select>
      <input v-model="filters.from" type="date" />
      <input v-model="filters.to"   type="date" />
      <button class="icon-btn" @click="loadMovements">{{ $t('common.filter') }}</button>
      <button class="icon-btn" @click="resetFilters">{{ $t('common.clear') }}</button>
    </div>

    <!-- Table -->
    <div class="table-wrap">
      <table class="movements-table">
        <thead>
          <tr>
            <th>{{ $t('history.slot') }}</th>
            <th>{{ $t('history.type') }}</th>
            <th>{{ $t('slot.sku') }}</th>
            <th>{{ $t('history.operator') }}</th>
            <th>{{ $t('history.time') }}</th>
            <th>Obs</th>
          </tr>
        </thead>
        <tbody>
          <tr v-if="loading">
            <td colspan="6" class="empty-row">{{ $t('common.loading') }}</td>
          </tr>
          <tr v-else-if="!movements.length">
            <td colspan="6" class="empty-row">{{ $t('history.noResults') }}</td>
          </tr>
          <tr v-for="m in movements" :key="m.id" class="movement-row">
            <td><span class="addr-pill">{{ m.slotId }}</span></td>
            <td>
              <span class="tag" :class="m.type === 'entry' ? 'tag-red' : 'tag-green'">
                {{ m.type === 'entry' ? '▶ ' + $t('history.typeEntry') : '◀ ' + $t('history.typeExit') }}
              </span>
            </td>
            <td><span class="sku-text">{{ m.sku || '—' }}</span></td>
            <td>{{ m.operator }}</td>
            <td class="date-cell">{{ formatDate(m.createdAt) }}</td>
            <td class="note-cell">{{ m.note || '' }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    <div class="pagination">
      <button class="icon-btn" :disabled="offset === 0" @click="prev">← {{ $t('common.previous') || 'Anterior' }}</button>
      <span class="page-info">{{ offset / PAGE_SIZE + 1 }} / {{ Math.ceil(total / PAGE_SIZE) || 1 }}</span>
      <button class="icon-btn" :disabled="offset + PAGE_SIZE >= total" @click="next">{{ $t('common.next') || 'Próximo' }} →</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Movement } from '~/types'

definePageMeta({ 
  layout: 'default',
  requiresAuth: true
})

const api = useWarehouseApi()
const { t } = useI18n()
const PAGE_SIZE = 50

const movements = ref<Movement[]>([])
const loading = ref(false)
const total = ref(0)
const offset = ref(0)

const filters = reactive({ slotId: '', type: '' as '' | 'entry' | 'exit', from: '', to: '' })

function formatDate(iso: string) {
  if (!iso) return '—'
  const date = new Date(iso)
  return isNaN(date.getTime()) ? '—' : new Intl.DateTimeFormat('pt-BR', { dateStyle: 'short', timeStyle: 'medium' }).format(date)
}

async function loadMovements() {
  loading.value = true
  try {
    const res = await api.fetchMovements({
      ...(filters.slotId ? { slotId: filters.slotId } : {}),
      ...(filters.type   ? { type: filters.type }     : {}),
      ...(filters.from   ? { from: filters.from }      : {}),
      ...(filters.to     ? { to: filters.to }          : {}),
      limit: PAGE_SIZE,
      offset: offset.value,
    })
    movements.value = res
  } catch {
    movements.value = mockMovements()
  } finally {
    loading.value = false
  }
}

function resetFilters() {
  filters.slotId = ''; filters.type = ''; filters.from = ''; filters.to = ''
  offset.value = 0
  loadMovements()
}

function prev() { if (offset.value > 0) { offset.value -= PAGE_SIZE; loadMovements() } }
function next() { offset.value += PAGE_SIZE; loadMovements() }

function mockMovements(): Movement[] {
  const streets = ['A','B','C','D','E','F']
  const lanes   = ['N1','N2','N3']
  const ops     = ['João', 'Maria', 'Carlos']
  return Array.from({ length: 20 }, (_, i) => ({
    id: String(i),
    slotId: `${streets[i%6]}-${(i%20)+1}-${lanes[i%3]}`,
    type: i % 3 === 0 ? 'exit' : 'entry',
    operator: ops[i % 3] || 'Desconhecido',
    sku: i % 4 === 0 ? `SKU-${1000+i}` : undefined,
    note: i % 7 === 0 ? 'Carga frágil' : undefined,
    createdAt: new Date(Date.now() - i * 3_600_000).toISOString(),
  }))
}

onMounted(loadMovements)
</script>

<style scoped>
.history-page { display: flex; flex-direction: column; height: 100vh; }

.page-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-1);
  flex-shrink: 0;
}
.page-title { font-size: 13px; font-weight: 600; letter-spacing: .16em; }
.header-actions { display: flex; gap: 8px; }

.filters {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-1);
  flex-wrap: wrap;
  flex-shrink: 0;
}
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
.icon-btn:disabled { opacity: .3; cursor: not-allowed; }

.table-wrap { flex: 1; overflow: auto; }
.movements-table { width: 100%; border-collapse: collapse; font-size: 12px; }
.movements-table th {
  position: sticky;
  top: 0;
  background: var(--bg-2);
  padding: 10px 16px;
  text-align: left;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: .1em;
  color: var(--text-3);
  border-bottom: 1px solid var(--border);
  white-space: nowrap;
}
.movements-table td {
  padding: 10px 16px;
  border-bottom: 1px solid var(--border);
  color: var(--text);
  vertical-align: middle;
}
.movement-row:hover td { background: var(--bg-1); }

.empty-row { text-align: center; color: var(--text-3); padding: 40px !important; }
.addr-pill {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: .08em;
  color: var(--text);
  font-family: var(--mono);
}
.sku-text  { font-size: 11px; color: var(--text-2); }
.date-cell { color: var(--text-2); white-space: nowrap; }
.note-cell { color: var(--text-3); font-size: 11px; max-width: 200px; }

.pagination {
  display: flex;
  align-items: center;
  gap: 12px;
  justify-content: center;
  padding: 12px;
  border-top: 1px solid var(--border);
  background: var(--bg-1);
  flex-shrink: 0;
}
.page-info { font-size: 11px; color: var(--text-3); }
</style>