<template>
  <div class="action-panel">
    <div class="panel-header">
      <span class="panel-title">{{ $t('actionPanel.title') }}</span>
    </div>

    <div class="panel-body">
      <!-- Address input -->
      <div class="field-group">
        <label class="field-label">{{ $t('actionPanel.address') }}</label>
        <input
          ref="locRef"
          v-model="location"
          type="text"
          :placeholder="$t('actionPanel.addressPlaceholder')"
          maxlength="8"
          style="width:100%; text-transform:uppercase; letter-spacing:.08em;"
          @input="location = location.toUpperCase()"
          @keydown.enter="handleEntry"
        />
        <div class="field-hint" :class="validationClass">{{ validationMsg }}</div>
      </div>

      <!-- SKU / note (optional) -->
      <div class="field-group">
        <label class="field-label">{{ $t('actionPanel.skuNote') }} <span class="optional">{{ $t('actionPanel.optional') }}</span></label>
        <input v-model="sku" type="text" :placeholder="$t('actionPanel.skuPlaceholder')" style="width:100%" />
      </div>

      <!-- Buttons -->
      <div class="action-btns">
        <button class="btn btn-entry" :disabled="loading" @click="handleEntry">
          <span>▶ {{ $t('actionPanel.entry') }}</span>
        </button>
        <button class="btn btn-exit" :disabled="loading" @click="handleExit">
          <span>◀ {{ $t('actionPanel.exit') }}</span>
        </button>
      </div>

      <button class="btn-undo" :disabled="!lastAction || loading" @click="handleUndo">
        ↩ {{ $t('actionPanel.undo') }}
      </button>

      <!-- Selected slot info -->
      <div v-if="selectedSlot" class="slot-info">
        <div class="slot-info-header">
          <span class="slot-id">{{ selectedSlot.id }}</span>
          <span class="tag" :class="selectedSlot.status === 'free' ? 'tag-green' : 'tag-red'">
            {{ selectedSlot.status === 'free' ? $t('slot.free') : $t('slot.occupied') }}
          </span>
        </div>
        <div v-if="selectedSlot.sku" class="slot-meta">{{ $t('slot.sku') }}: {{ selectedSlot.sku }}</div>
        <div v-if="selectedSlot.updatedAt" class="slot-meta">
          {{ $t('slot.updatedAt') }}: {{ formatDate(selectedSlot.updatedAt) }}
        </div>
        <div v-if="selectedSlot.updatedBy" class="slot-meta">{{ $t('slot.updatedBy') }}: {{ selectedSlot.updatedBy }}</div>
        <button class="use-addr-btn" @click="location = selectedSlot!.id">
          {{ $t('actionPanel.useThisAddress') }}
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
const { t } = useI18n()

const location = ref('')
const sku = ref('')
const loading = ref(false)
const lastAction = ref<{ slotId: string; type: 'entry' | 'exit' } | null>(null)
const locRef = ref<HTMLInputElement | null>(null)

const ADDR_RE = /^[A-F]-(\d{1,2})-(N[123])$/

const validationMsg = computed(() => {
  if (!location.value) return ''
  if (!ADDR_RE.test(location.value)) return t('actionPanel.addressFormat')
  const slot = store.getSlot(location.value)
  if (!slot) return t('actionPanel.addressNotFound')
  return slot.status === 'free' ? t('actionPanel.slotFree') : t('actionPanel.slotOccupied')
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
  console.log('ACTION PANEL: Iniciando handleEntry para', location.value)
  
  try {
    const updated = await api.entry(location.value, sku.value || undefined)
    
    // Verificar status antes de atualizar
    const currentSlot = store.getSlot(location.value)
    const shouldNotify = !currentSlot || currentSlot.status !== updated.status
    
    store.setSlot(updated)
    lastAction.value = { slotId: location.value, type: 'entry' }
    
    push({ type: 'success', message: t('actionPanel.entryRegistered', { address: location.value }) + (updated.sku ? ` - SKU: ${updated.sku}` : '') })
    
    sku.value = ''
    emit('done')
  } catch (e: any) {
    console.log('ACTION PANEL: Erro no entry', e)
    if (e?.code === 'SLOT_OCCUPIED') {
      console.log('ACTION PANEL: Tratando SLOT_OCCUPIED')
      const existingSlot = store.getSlot(location.value)
      const shouldNotify = !existingSlot || existingSlot.status !== 'occupied'
      
      if (existingSlot && shouldNotify) {
        store.setSlot({ ...existingSlot, status: 'occupied', updatedAt: new Date().toISOString() })
        push({ type: 'warning', message: t('actionPanel.slotAlreadyOccupied', { address: location.value }) })
      }
      
      // Tentar buscar estado real do servidor
      try {
        const slots = await api.fetchSlots()
        const currentSlot = slots.find((s: Slot) => s.id === location.value)
        if (currentSlot) {
          store.setSlot(currentSlot)
        }
      } catch {
        // Manter estado local se falhar busca
      }
    } else {
      const errorMessage = e?.message || e?.error || t('errors.generic')
      push({ type: 'danger', message: errorMessage })
    }
  } finally {
    loading.value = false
  }
}

async function handleExit() {
  if (!ADDR_RE.test(location.value)) return
  loading.value = true
  try {
    const updated = await api.exit(location.value, sku.value || undefined)
    
    // Verificar se o slot já está com o mesmo status (evitar duplicação)
    const currentSlot = store.getSlot(location.value)
    const shouldNotify = !currentSlot || currentSlot.status !== updated.status
    
    store.setSlot(updated)
    lastAction.value = { slotId: location.value, type: 'exit' }
    
    push({ type: 'info', message: t('actionPanel.exitRegistered', { address: location.value }) })
    
    sku.value = ''
    emit('done')
  } catch (e: any) {
    // Com fetch nativo, o erro agora é o corpo da resposta diretamente
    if (e?.code === 'SLOT_FREE') {
      // Forçar atualização local imediata
      const existingSlot = store.getSlot(location.value)
      const shouldNotify = !existingSlot || existingSlot.status !== 'free'
      
      if (existingSlot && shouldNotify) {
        store.setSlot({ ...existingSlot, status: 'free', updatedAt: new Date().toISOString() })
        push({ type: 'warning', message: t('actionPanel.slotAlreadyFree', { address: location.value }) })
      }
      
      // Tentar buscar estado real do servidor
      try {
        const slots = await api.fetchSlots()
        const currentSlot = slots.find((s: Slot) => s.id === location.value)
        if (currentSlot) {
          store.setSlot(currentSlot)
        }
      } catch {
        // Manter estado local se falhar busca
      }
    } else {
      const errorMessage = e?.message || e?.error || t('errors.generic')
      push({ type: 'danger', message: errorMessage })
    }
  } finally {
    loading.value = false
  }
}

async function handleUndo() {
  if (!lastAction.value) return
  loading.value = true
  try {
    await api.undoLastMovement(lastAction.value.slotId)
    push({ type: 'info', message: t('history.undoSuccess') + ` ${lastAction.value.slotId}` })
    lastAction.value = null
    emit('done')
  } catch (e: any) {
    push({ type: 'danger', message: t('errors.generic') })
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