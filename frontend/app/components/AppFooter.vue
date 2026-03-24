<template>
  <footer class="app-footer">
    <div class="footer-left">
      <div class="system-info">
        <span class="system-name">{{ $t('footer.systemName') }}</span>
        <span class="system-version">{{ $t('footer.version') }}</span>
      </div>
      <div class="footer-stats">
        <span class="stat-item">
          <span class="stat-label">{{ $t('footer.slots') }}:</span>
          <span class="stat-value">{{ globalStats.value.total }}</span>
        </span>
        <span class="stat-item">
          <span class="stat-label">{{ $t('footer.rate') }}:</span>
          <span class="stat-value">{{ globalStats.value.pct.toFixed(1) }}%</span>
        </span>
        <span class="stat-item">
          <span class="stat-label">{{ $t('footer.status') }}:</span>
          <span class="stat-value" :class="statusClass">{{ wsStatus }}</span>
        </span>
      </div>
    </div>

    <div class="footer-center">
      <div class="last-sync">
        <span class="sync-icon" :class="{ 'sync-icon--active': isSyncing }">⟳</span>
        <span class="sync-text">
          {{ $t('footer.lastUpdate') }}: {{ lastSyncFormatted }}
        </span>
      </div>
    </div>

    <div class="footer-right">
      <div class="footer-actions">
        <button class="footer-btn" @click="refreshData" :disabled="isRefreshing">
          <span>{{ $t('footer.refresh') }}</span>
        </button>
        <button class="footer-btn" @click="showInfo = true">
          <span>ℹ</span>
        </button>
      </div>
    </div>

    <!-- Info Modal -->
    <Teleport to="body">
      <div v-if="showInfo" class="modal-overlay" @click.self="showInfo = false">
        <div class="modal-content">
          <div class="modal-header">
            <h3>{{ $t('footer.info') }}</h3>
            <button class="modal-close" @click="showInfo = false">✕</button>
          </div>
          <div class="modal-body">
            <div class="info-grid">
              <div class="info-item">
                <span class="info-label">{{ $t('footer.system') }}:</span>
                <span class="info-value">{{ $t('footer.wmsSystem') }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">{{ $t('footer.version') }}:</span>
                <span class="info-value">1.0.0</span>
              </div>
              <div class="info-item">
                <span class="info-label">{{ $t('footer.api') }}:</span>
                <span class="info-value">{{ apiBaseUrl }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">{{ $t('footer.websocket') }}:</span>
                <span class="info-value">{{ wsStatus }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">{{ $t('footer.totalSlots') }}:</span>
                <span class="info-value">{{ globalStats.value.total }}</span>
              </div>
              <div class="info-item">
                <span class="info-label">{{ $t('footer.occupancy') }}:</span>
                <span class="info-value">{{ globalStats.value.pct.toFixed(1) }}%</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Teleport>
  </footer>
</template>

<script setup lang="ts">
const { t } = useI18n()
const store = useWarehouseStore()
const ws = useWarehouseWS()
const { push } = useAlerts()
const config = useRuntimeConfig()

const showInfo = ref(false)
const isRefreshing = ref(false)
const lastSync = ref(new Date())

const globalStats = computed(() => store.globalStats)
const wsStatus = computed(() => ws.connected.value ? t('footer.connected') : t('footer.disconnected'))
const statusClass = computed(() => ws.connected.value ? 'stat-value--online' : 'stat-value--offline')
const isSyncing = computed(() => ws.connected.value && ws.lastEvent.value)
const apiBaseUrl = computed(() => config.public.apiBase)

const lastSyncFormatted = computed(() => {
  const now = new Date()
  const diff = now.getTime() - lastSync.value.getTime()
  
  if (diff < 60000) return t('footer.justNow')
  if (diff < 3600000) return `${Math.floor(diff / 60000)} ${t('footer.minutes')}`
  if (diff < 86400000) return `${Math.floor(diff / 3600000)} ${t('footer.hours')}`
  return lastSync.value.toLocaleDateString('pt-BR')
})

async function refreshData() {
  isRefreshing.value = true
  try {
    const api = useWarehouseApi()
    const slots = await api.fetchSlots()
    store.bulkLoad(slots)
    lastSync.value = new Date()
    push({ type: 'success', message: t('footer.dataUpdated') })
  } catch {
    push({ type: 'danger', message: t('footer.updateFailed') })
  } finally {
    isRefreshing.value = false
  }
}

watch(() => ws.lastEvent.value, () => {
  lastSync.value = new Date()
})
</script>

<style scoped>
.app-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 20px;
  background: var(--bg-1);
  border-top: 1px solid var(--border);
  min-height: 48px;
  font-size: 11px;
  color: var(--text-2);
}

.footer-left {
  display: flex;
  align-items: center;
  gap: 20px;
}

.system-info {
  display: flex;
  align-items: center;
  gap: 8px;
}

.system-name {
  font-weight: 600;
  color: var(--text);
}

.system-version {
  background: var(--bg-2);
  padding: 2px 6px;
  border-radius: 10px;
  font-size: 9px;
  font-weight: 500;
}

.footer-stats {
  display: flex;
  gap: 12px;
}

.stat-item {
  display: flex;
  gap: 4px;
}

.stat-label {
  color: var(--text-3);
}

.stat-value {
  font-weight: 500;
  color: var(--text);
}

.stat-value--online {
  color: var(--green);
}

.stat-value--offline {
  color: var(--red);
}

.footer-center {
  flex: 1;
  display: flex;
  justify-content: center;
}

.last-sync {
  display: flex;
  align-items: center;
  gap: 6px;
}

.sync-icon {
  display: inline-block;
  transition: transform 0.3s;
}

.sync-icon--active {
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.sync-text {
  color: var(--text-3);
}

.footer-right {
  display: flex;
  align-items: center;
}

.footer-actions {
  display: flex;
  gap: 8px;
}

.footer-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  color: var(--text-2);
  font-size: 10px;
  cursor: pointer;
  transition: all 0.12s;
}

.footer-btn:hover:not(:disabled) {
  color: var(--text);
  border-color: var(--border-2);
}

.footer-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Modal Styles */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  max-width: 400px;
  width: 90%;
  max-height: 80vh;
  overflow: auto;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
}

.modal-header h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: var(--text);
}

.modal-close {
  background: none;
  border: none;
  color: var(--text-2);
  cursor: pointer;
  font-size: 16px;
  padding: 4px;
}

.modal-body {
  padding: 20px;
}

.info-grid {
  display: grid;
  gap: 12px;
}

.info-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.info-label {
  color: var(--text-3);
  font-size: 11px;
}

.info-value {
  color: var(--text);
  font-weight: 500;
  font-size: 11px;
}
</style>    