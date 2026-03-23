<template>
  <div class="action-panel">
    <div class="panel-header">
      <span class="panel-title">CONTROLE DE POSIÇÃO</span>
    </div>

    <div class="panel-body">
      <!-- Address input -->
      <div class="field-group">
        <label class="field-label">Endereço</label>
        <input
          ref="locRef"
          v-model="location"
          type="text"
          placeholder="A-1-N1"
          maxlength="8"
          style="width:100%; text-transform:uppercase; letter-spacing:.08em;"
          @input="location = location.toUpperCase()"
          @keydown.enter="handleEntry"
        />
        <div class="field-hint" :class="validationClass">{{ validationMsg }}</div>
      </div>

      <!-- SKU / note (optional) -->
      <div class="field-group">
        <label class="field-label">SKU / Observação <span class="optional">(opcional)</span></label>
        <input v-model="sku" type="text" placeholder="EX-4521-B" style="width:100%" />
      </div>

      <!-- Buttons -->
      <div class="action-btns">
        <button class="btn btn-entry" :disabled="loading" @click="handleEntry">
          <span>▶ ENTRADA</span>
        </button>
        <button class="btn btn-exit" :disabled="loading" @click="handleExit">
          <span>◀ SAÍDA</span>
        </button>
      </div>

      <button class="btn-undo" :disabled="!lastAction || loading" @click="handleUndo">
        ↩ Desfazer última operação
      </button>

      <!-- Selected slot info -->
      <div v-if="selectedSlot" class="slot-info">
        <div class="slot-info-header">
          <span class="slot-id">{{ selectedSlot.id }}</span>
          <span class="tag" :class="selectedSlot.status === 'free' ? 'tag-green' : 'tag-red'">
            {{ selectedSlot.status === 'free' ? 'LIVRE' : 'OCUPADO' }}
          </span>
        </div>
        <div v-if="selectedSlot.sku" class="slot-meta">SKU: {{ selectedSlot.sku }}</div>
        <div v-if="selectedSlot.updatedAt" class="slot-meta">
          Atualizado: {{ formatDate(selectedSlot.updatedAt) }}
        </div>
        <div v-if="selectedSlot.updatedBy" class="slot-meta">Por: {{ selectedSlot.updatedBy }}</div>
        <button class="use-addr-btn" @click="location = selectedSlot!.id">
          Usar este endereço
        </button>
      </div>
    </div>

    <!-- Loading overlay -->
    <div v-if="loading" class="loading-bar" />
  </div>
</template>

<script setup lang="ts">
import type { Slot } from '~/types'

const props = defineProps<{ selectedSlot?: Slot }>()
const emit = defineEmits<{ done: [] }>()

const api = useWarehouseApi()
const store = useWarehouseStore()
const { push } = useAlerts()

const location = ref('')
const sku = ref('')
const loading = ref(false)
const lastAction = ref<{ slotId: string; type: 'entry' | 'exit' } | null>(null)
const locRef = ref<HTMLInputElement | null>(null)

const ADDR_RE = /^[A-F]-(\d{1,2})-(N[123])$/

const validationMsg = computed(() => {
  if (!location.value) return ''
  if (!ADDR_RE.test(location.value)) return 'Formato inválido — use ex: A-5-N2'
  const slot = store.getSlot(location.value)
  if (!slot) return 'Posição não encontrada'
  return slot.status === 'free' ? '✓ Livre — pronta para entrada' : '✓ Ocupada — pronta para saída'
})

const validationClass = computed(() => {
  if (!location.value) return ''
  if (!ADDR_RE.test(location.value)) return 'hint-error'
  return 'hint-ok'
})

function formatDate(iso: string) {
  return new Intl.DateTimeFormat('pt-BR', { dateStyle: 'short', timeStyle: 'short' }).format(new Date(iso))
}

async function handleEntry() {
  if (!ADDR_RE.test(location.value)) return
  loading.value = true
  try {
    const updated = await api.entry(location.value, sku.value || undefined)
    store.setSlot(updated)
    lastAction.value = { slotId: location.value, type: 'entry' }
    push({ type: 'success', message: `Entrada registrada: ${location.value}` })
    sku.value = ''
    emit('done')
  } catch (e: any) {
    push({ type: 'danger', message: e?.data?.message ?? 'Erro ao registrar entrada' })
  } finally {
    loading.value = false
  }
}

async function handleExit() {
  if (!ADDR_RE.test(location.value)) return
  loading.value = true
  try {
    const updated = await api.exit(location.value, sku.value || undefined)
    store.setSlot(updated)
    lastAction.value = { slotId: location.value, type: 'exit' }
    push({ type: 'success', message: `Saída registrada: ${location.value}` })
    sku.value = ''
    emit('done')
  } catch (e: any) {
    push({ type: 'danger', message: e?.data?.message ?? 'Erro ao registrar saída' })
  } finally {
    loading.value = false
  }
}

async function handleUndo() {
  if (!lastAction.value) return
  loading.value = true
  try {
    await api.undoLastMovement(lastAction.value.slotId)
    push({ type: 'info', message: `Operação desfeita: ${lastAction.value.slotId}` })
    lastAction.value = null
    emit('done')
  } catch (e: any) {
    push({ type: 'danger', message: 'Não foi possível desfazer' })
  } finally {
    loading.value = false
  }
}

// Auto-fill from selected slot click
watch(() => props.selectedSlot, (s) => {
  if (s) location.value = s.id
})
</script>

<style scoped>
.action-panel {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  position: relative;
}
.panel-header {
  padding: 10px 14px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-2);
}
.panel-title {
  font-size: 10px;
  font-weight: 600;
  letter-spacing: .12em;
  color: var(--text-3);
}
.panel-body { padding: 14px; display: flex; flex-direction: column; gap: 12px; }

.field-group { display: flex; flex-direction: column; gap: 5px; }
.field-label { font-size: 10px; font-weight: 600; letter-spacing: .08em; color: var(--text-3); text-transform: uppercase; }
.optional { color: var(--text-3); font-weight: 400; text-transform: none; }
.field-hint { font-size: 10px; min-height: 14px; }
.hint-ok    { color: var(--green); }
.hint-error { color: var(--red); }

.action-btns { display: grid; grid-template-columns: 1fr 1fr; gap: 8px; }
.btn {
  height: 38px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: .06em;
  border: 1px solid transparent;
  transition: all .12s;
}
.btn:disabled { opacity: .4; cursor: not-allowed; }
.btn-entry {
  background: var(--red-bg);
  color: var(--red);
  border-color: var(--red-dim);
}
.btn-entry:not(:disabled):hover { background: #2d0909; border-color: var(--red); }
.btn-exit {
  background: var(--green-bg);
  color: var(--green);
  border-color: var(--green-dim);
}
.btn-exit:not(:disabled):hover { background: #064e1e; border-color: var(--green); }

.btn-undo {
  font-size: 10px;
  font-weight: 500;
  color: var(--text-3);
  background: none;
  border: 1px solid var(--border);
  height: 28px;
  letter-spacing: .04em;
  transition: all .12s;
}
.btn-undo:not(:disabled):hover { color: var(--text); border-color: var(--border-2); }
.btn-undo:disabled { opacity: .3; cursor: not-allowed; }

.slot-info {
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 10px 12px;
  display: flex;
  flex-direction: column;
  gap: 5px;
  animation: fade-in .15s;
}
.slot-info-header { display: flex; align-items: center; justify-content: space-between; }
.slot-id { font-size: 13px; font-weight: 600; letter-spacing: .08em; color: var(--text); }
.slot-meta { font-size: 10px; color: var(--text-3); }
.use-addr-btn {
  margin-top: 4px;
  font-size: 10px;
  color: var(--blue);
  background: none;
  border: none;
  padding: 0;
  text-align: left;
  cursor: pointer;
  letter-spacing: .04em;
  text-decoration: underline;
}

.loading-bar {
  position: absolute;
  bottom: 0; left: 0; right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, var(--green), transparent);
  animation: loading-slide 1s linear infinite;
}
@keyframes loading-slide {
  from { transform: translateX(-100%); }
  to   { transform: translateX(100%); }
}
</style>