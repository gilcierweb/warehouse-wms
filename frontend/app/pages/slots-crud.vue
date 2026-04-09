<template>
  <div class="map-page">
    <!-- Top bar -->
    <header class="topbar">
      <div class="topbar-left">
        <h1 class="page-title">{{ $t('slotsCrud.title') }}</h1>
        <div class="global-stats">
          <span class="stat-chip">
            <span class="chip-dot chip-dot--green" />
            {{ slots.filter(s => s.status === 'free').length }} {{ $t('slotsCrud.free') }}
          </span>
          <span class="stat-chip">
            <span class="chip-dot chip-dot--red" />
            {{ slots.filter(s => s.status === 'occupied').length }} {{ $t('slotsCrud.occupied') }}
          </span>
          <span class="stat-chip stat-chip--pct">
            {{ slots.length }} {{ $t('slotsCrud.total') }}
          </span>
        </div>
      </div>

      <div class="topbar-right">
        <!-- Search / filter -->
        <input v-model="searchQuery" type="text" :placeholder="$t('slotsCrud.searchPlaceholder')" style="width:220px" />
        <select v-model="selectedStreet" class="form-select" style="width:120px">
          <option value="">{{ $t('slotsCrud.allStreets') }}</option>
          <option v-for="street in streets" :key="street" :value="street">
            {{ street }}
          </option>
        </select>
        <select v-model="selectedStatus" class="form-select" style="width:120px">
          <option value="">{{ $t('slotsCrud.allStatuses') }}</option>
          <option value="free">{{ $t('slotsCrud.free') }}</option>
          <option value="occupied">{{ $t('slotsCrud.occupied') }}</option>
        </select>
        <button class="icon-btn" :title="$t('slotsCrud.resetFilters')" @click="resetFilters">↻</button>
        <button class="icon-btn icon-btn--blue" :title="$t('slotsCrud.createSlot')" @click="showCreateModal = true">+</button>
      </div>
    </header>

    <!-- Loading -->
    <div v-if="loading" class="loading-container">
      <div class="spinner"></div>
      <p>{{ $t('slotsCrud.loading') }}</p>
    </div>

    <!-- Main content -->
    <div v-else class="content-grid">
      <!-- Table area -->
      <div class="map-area">
        <div class="table-card">
          <div class="table-wrapper">
            <table class="detail-table">
              <thead>
                <tr>
                  <th>{{ $t('slotsCrud.address') }}</th>
                  <th>{{ $t('slotsCrud.street') }}</th>
                  <th>{{ $t('slotsCrud.position') }}</th>
                  <th>{{ $t('slotsCrud.lane') }}</th>
                  <th>{{ $t('slotsCrud.status') }}</th>
                  <th>{{ $t('slotsCrud.sku') }}</th>
                  <th>{{ $t('slotsCrud.updatedAt') }}</th>
                  <th class="actions-column">{{ $t('slotsCrud.actions') }}</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="slot in filteredSlots" :key="slot.id">
                  <td>
                    <code class="address-code">{{ slot.address }}</code>
                  </td>
                  <td>{{ slot.street }}</td>
                  <td>{{ slot.position }}</td>
                  <td>{{ slot.lane }}</td>
                  <td>
                    <span 
                      class="tag" 
                      :class="slot.status === 'occupied' ? 'tag-red' : 'tag-green'"
                    >
                      {{ $t(`slotsCrud.${slot.status}`) }}
                    </span>
                  </td>
                  <td>
                    <span v-if="slot.sku" class="sku-badge">{{ slot.sku }}</span>
                    <span v-else class="text-muted">—</span>
                  </td>
                  <td class="date-cell">{{ formatDate(slot.updated_at) }}</td>
                  <td class="actions-cell">
                    <button 
                      class="icon-btn" 
                      @click="editSlot(slot)"
                      :title="$t('slotsCrud.edit')"
                    >
                      ✏️
                    </button>
                    <button 
                      class="icon-btn icon-btn--red" 
                      @click="confirmDelete(slot)"
                      :title="$t('slotsCrud.delete')"
                    >
                      🗑️
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>

          <!-- Empty State -->
          <div v-if="filteredSlots.length === 0" class="empty-state">
            <div class="empty-icon">📦</div>
            <h3>{{ $t('slotsCrud.noSlotsFound') }}</h3>
            <p>{{ $t('slotsCrud.noSlotsDescription') }}</p>
            <button class="icon-btn icon-btn--blue" @click="showCreateModal = true">
              {{ $t('slotsCrud.createFirstSlot') }}
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Create/Edit Modal -->
    <div v-if="showCreateModal || editingSlot" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ editingSlot ? $t('slotsCrud.editSlot') : $t('slotsCrud.createSlot') }}</h2>
          <button class="modal-close" @click="closeModal">✕</button>
        </div>
        <form @submit.prevent="saveSlot" class="modal-body">
          <div class="form-row">
            <div class="form-group">
              <label for="street">{{ $t('slotsCrud.street') }} *</label>
              <select 
                id="street" 
                v-model="slotForm.street" 
                class="form-control"
                required
                :disabled="!!editingSlot"
              >
                <option value="">{{ $t('slotsCrud.selectStreet') }}</option>
                <option v-for="street in streetOptions" :key="street" :value="street">
                  {{ street }}
                </option>
              </select>
            </div>
            <div class="form-group">
              <label for="position">{{ $t('slotsCrud.position') }} *</label>
              <input 
                id="position" 
                v-model.number="slotForm.position" 
                type="number" 
                min="1" 
                max="30"
                class="form-control"
                required
                :disabled="!!editingSlot"
              />
            </div>
            <div class="form-group">
              <label for="lane">{{ $t('slotsCrud.lane') }} *</label>
              <select 
                id="lane" 
                v-model="slotForm.lane" 
                class="form-control"
                required
                :disabled="!!editingSlot"
              >
                <option value="">{{ $t('slotsCrud.selectLane') }}</option>
                <option value="N1">N1</option>
                <option value="N2">N2</option>
                <option value="N3">N3</option>
              </select>
            </div>
          </div>
          <div class="form-row">
            <div class="form-group">
              <label for="status">{{ $t('slotsCrud.status') }}</label>
              <select id="status" v-model="slotForm.status" class="form-control">
                <option value="free">{{ $t('slotsCrud.free') }}</option>
                <option value="occupied">{{ $t('slotsCrud.occupied') }}</option>
              </select>
            </div>
            <div class="form-group">
              <label for="sku">{{ $t('slotsCrud.sku') }}</label>
              <input 
                id="sku" 
                v-model="slotForm.sku" 
                type="text" 
                class="form-control"
                :placeholder="$t('slotsCrud.skuPlaceholder')"
              />
            </div>
          </div>
          <div class="form-actions">
            <button type="button" class="btn btn-secondary" @click="closeModal">
              {{ $t('slotsCrud.cancel') }}
            </button>
            <button type="submit" class="btn btn-primary" :disabled="saving">
              <span v-if="saving" class="btn-spinner"></span>
              {{ editingSlot ? $t('slotsCrud.update') : $t('slotsCrud.create') }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Delete Confirmation Modal -->
    <div v-if="deletingSlot" class="modal-overlay" @click="cancelDelete">
      <div class="modal-content modal-sm" @click.stop>
        <div class="modal-header">
          <h2>{{ $t('slotsCrud.deleteSlot') }}</h2>
          <button class="modal-close" @click="cancelDelete">✕</button>
        </div>
        <div class="modal-body">
          <p>{{ $t('slotsCrud.deleteConfirmation', { address: deletingSlot.address }) }}</p>
          <div class="form-actions">
            <button class="btn btn-secondary" @click="cancelDelete">
              {{ $t('slotsCrud.cancel') }}
            </button>
            <button class="btn btn-danger" @click="deleteSlot" :disabled="deleting">
              <span v-if="deleting" class="btn-spinner"></span>
              {{ $t('slotsCrud.delete') }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Slot, CreateSlotRequest, UpdateSlotRequest } from '~/types'

// Page meta
definePageMeta({
  layout: 'default',
  title: 'Slots CRUD',
  requiresAuth: true
})

// Composables
const { t } = useI18n()
const api = useWarehouseApi()
const { push: toast } = useAlerts()

// State
const slots = ref<Slot[]>([])
const loading = ref(true)
const saving = ref(false)
const deleting = ref(false)

// Filters
const searchQuery = ref('')
const selectedStreet = ref('')
const selectedStatus = ref('')

// Modal states
const showCreateModal = ref(false)
const editingSlot = ref<Slot | null>(null)
const deletingSlot = ref<Slot | null>(null)

// Form
const slotForm = ref<CreateSlotRequest & { status?: string; sku?: string }>({
  street: '',
  position: 1,
  lane: '',
  status: 'free',
  sku: ''
})

// Computed
const filteredSlots = computed(() => {
  let filtered = slots.value

  // Search filter
  if (searchQuery.value) {
    const query = searchQuery.value.toLowerCase()
    filtered = filtered.filter(slot => 
      slot.address.toLowerCase().includes(query) ||
      slot.street.toLowerCase().includes(query) ||
      slot.sku?.toLowerCase().includes(query)
    )
  }

  // Street filter
  if (selectedStreet.value) {
    filtered = filtered.filter(slot => slot.street === selectedStreet.value)
  }

  // Status filter
  if (selectedStatus.value) {
    filtered = filtered.filter(slot => slot.status === selectedStatus.value)
  }

  return filtered
})

const streets = computed(() => {
  const uniqueStreets = [...new Set(slots.value.map(slot => slot.street))]
  return uniqueStreets.sort()
})

const streetOptions = computed(() => {
  const options = []
  for (let i = 65; i <= 90; i++) {
    options.push(String.fromCharCode(i))
  }
  return options
})

// Methods
const loadSlots = async () => {
  try {
    loading.value = true
    slots.value = await api.fetchSlots()
  } catch (error) {
    toast({ type: 'danger', message: t('slotsCrud.loadError') })
    console.error('Error loading slots:', error)
  } finally {
    loading.value = false
  }
}

const resetFilters = () => {
  searchQuery.value = ''
  selectedStreet.value = ''
  selectedStatus.value = ''
}

const editSlot = (slot: Slot) => {
  editingSlot.value = slot
  slotForm.value = {
    street: slot.street,
    position: slot.position,
    lane: slot.lane,
    status: slot.status,
    sku: slot.sku || ''
  }
}

const confirmDelete = (slot: Slot) => {
  deletingSlot.value = slot
}

const closeModal = () => {
  showCreateModal.value = false
  editingSlot.value = null
  slotForm.value = {
    street: '',
    position: 1,
    lane: '',
    status: 'free',
    sku: ''
  }
}

const cancelDelete = () => {
  deletingSlot.value = null
}

const saveSlot = async () => {
  try {
    saving.value = true
    
    if (editingSlot.value) {
      // Update existing slot
      const updateData: UpdateSlotRequest = {
        status: slotForm.value.status as 'free' | 'occupied',
        sku: slotForm.value.sku || undefined
      }
      await api.updateSlot(editingSlot.value.id, updateData)
      toast({ type: 'success', message: t('slotsCrud.updateSuccess') })
    } else {
      // Create new slot
      await api.createSlot(slotForm.value as CreateSlotRequest)
      toast({ type: 'success', message: t('slotsCrud.createSuccess') })
    }
    
    await loadSlots()
    closeModal()
  } catch (error: any) {
    const message = error.error || error.message || t('slotsCrud.saveError')
    toast({ type: 'danger', message })
    console.error('Error saving slot:', error)
  } finally {
    saving.value = false
  }
}

const deleteSlot = async () => {
  if (!deletingSlot.value) return
  
  try {
    deleting.value = true
    await api.deleteSlot(deletingSlot.value.id)
    toast({ type: 'success', message: t('slotsCrud.deleteSuccess') })
    await loadSlots()
    cancelDelete()
  } catch (error: any) {
    const message = error.error || error.message || t('slotsCrud.deleteError')
    toast({ type: 'danger', message })
    console.error('Error deleting slot:', error)
  } finally {
    deleting.value = false
  }
}

const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleString()
}

// Lifecycle
onMounted(() => {
  loadSlots()
})
</script>

<style scoped>
/* Page layout */
.map-page { 
  display: flex; 
  flex-direction: column; 
  height: 100vh; 
}

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

.topbar-left { 
  display: flex; 
  align-items: center; 
  gap: 20px; 
}

.page-title {  
  font-size: 13px; 
  font-weight: 600; 
  letter-spacing: .16em; 
  color: var(--text); 
}

.global-stats { 
  display: flex; 
  gap: 10px; 
  align-items: center; 
}

.stat-chip {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 11px;
  color: var(--text-2);
  font-weight: 500;
}

.stat-chip--pct { 
  font-weight: 600; 
}

.chip-dot { 
  width: 6px; 
  height: 6px; 
  border-radius: 50%; 
}

.chip-dot--red   { 
  background: var(--red); 
}

.chip-dot--green { 
  background: var(--green); 
}

.topbar-right { 
  display: flex; 
  align-items: center; 
  gap: 8px; 
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

.icon-btn:hover { 
  color: var(--text); 
  border-color: var(--border-2); 
}

.icon-btn--blue { 
  color: var(--blue); 
  border-color: #0c4a6e; 
  background: var(--blue-bg); 
}

.icon-btn--blue:hover { 
  background: #032033; 
}

.icon-btn--red {
  color: var(--red);
  border-color: var(--red-dim);
  background: var(--red-bg);
}

.icon-btn--red:hover {
  background: #2a0808;
}

/* Content grid */
.content-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
  flex: 1;
  overflow: hidden;
}

/* Map area */
.map-area {
  display: flex;
  flex-direction: column;
  gap: 20px;
  overflow: hidden;
  flex: 1;
}

/* Table card */
.table-card {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
  flex: 1;
  display: flex;
  flex-direction: column;
  position: relative;
}

/* Table wrapper for scrolling */
.table-wrapper {
  flex: 1;
  overflow: auto;
  min-height: 0;
}

/* Detail table */
.detail-table {
  width: 100%;
  border-collapse: collapse;
  font-family: var(--mono);
}

.detail-table th {
  background: var(--bg-2);
  padding: 12px 16px;
  text-align: left;
  font-weight: 600;
  color: var(--text);
  border-bottom: 1px solid var(--border);
  font-size: 11px;
  letter-spacing: .06em;
  text-transform: uppercase;
  position: sticky;
  top: 0;
  z-index: 10;
}

.detail-table td {
  padding: 12px 16px;
  border-bottom: 1px solid var(--bg-2);
  font-size: 12px;
  color: var(--text);
}

.detail-table tr:hover {
  background: var(--bg-2);
}

.detail-table tr:last-child td {
  border-bottom: none;
}

/* Custom components */
.address-code {
  background: var(--bg-2);
  color: var(--text);
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius);
  font-family: var(--mono);
  font-size: 0.875rem;
  border: 1px solid var(--border);
}

.sku-badge {
  background: var(--blue-bg);
  color: var(--blue);
  padding: 0.25rem 0.5rem;
  border-radius: var(--radius);
  font-family: var(--mono);
  font-size: 0.75rem;
  border: 1px solid var(--blue);
}

.text-muted {
  color: var(--text-2);
}

.date-cell {
  color: var(--text-2);
  font-size: 0.875rem;
}

.actions-cell {
  display: flex;
  gap: 0.5rem;
}

/* Loading */
.loading-container {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem;
  color: var(--text-2);
}

.spinner {
  width: 2rem;
  height: 2rem;
  border: 2px solid var(--border);
  border-top: 2px solid var(--green);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

/* Empty state */
.empty-state {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 4rem 2rem;
  color: var(--text-2);
  background: var(--bg-1);
}

.empty-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.empty-state h3 {
  font-size: 1.25rem;
  margin-bottom: 0.5rem;
  color: var(--text);
}

/* Modals */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.5);
}

.modal-sm {
  max-width: 400px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--border);
}

.modal-header h2 {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text);
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-2);
  padding: 0.25rem;
  border-radius: var(--radius);
}

.modal-close:hover {
  color: var(--text);
  background: var(--bg-2);
}

.modal-body {
  padding: 1.5rem;
}

/* Forms */
.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
}

.form-group label {
  font-weight: 500;
  color: var(--text);
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
}

.form-control {
  font-family: var(--mono);
  font-size: 13px;
  background: var(--bg-2);
  border: 1px solid var(--border);
  color: var(--text);
  border-radius: var(--radius);
  padding: 0 10px;
  height: 34px;
  outline: none;
  transition: border-color .15s;
}

.form-control:focus {
  border-color: var(--green);
  box-shadow: 0 0 0 2px rgba(34,197,94,.12);
}

.form-control::placeholder { 
  color: var(--text-3); 
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  margin-top: 2rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border);
}

.btn {
  font-family: var(--mono);
  font-size: 12px;
  font-weight: 500;
  letter-spacing: .04em;
  cursor: pointer;
  border-radius: var(--radius);
  transition: all .12s;
  padding: 0.75rem 1.5rem;
  border: none;
}

.btn:active { 
  transform: scale(.97); 
}

.btn-primary {
  background: var(--green);
  color: var(--bg);
}

.btn-primary:hover {
  background: #16a34a;
}

.btn-secondary {
  background: var(--bg-2);
  color: var(--text);
  border: 1px solid var(--border);
}

.btn-secondary:hover {
  background: var(--bg-3);
  border-color: var(--border-2);
}

.btn-danger {
  background: var(--red);
  color: var(--bg);
}

.btn-danger:hover {
  background: #dc2626;
}

.btn-spinner {
  width: 1rem;
  height: 1rem;
  border: 1px solid currentColor;
  border-top: 1px solid transparent;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

@media (max-width: 768px) {
  .form-row {
    grid-template-columns: 1fr;
  }
  
  .detail-table {
    font-size: 0.75rem;
  }
  
  .detail-table th,
  .detail-table td {
    padding: 0.5rem;
  }
  
  .topbar {
    padding: 10px;
  }
  
  .topbar-left {
    flex-direction: column;
    align-items: flex-start;
    gap: 10px;
  }
  
  .topbar-right {
    flex-wrap: wrap;
  }
}
</style>
