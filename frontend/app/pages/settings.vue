<template>
  <div class="settings-page">
    <header class="page-header">
      <h1 class="page-title">CONFIGURAÇÕES</h1>
    </header>

    <div class="settings-body">

      <!-- Alert thresholds -->
      <section class="settings-section">
        <div class="section-title">ALERTAS DE CAPACIDADE</div>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">Limite de alerta global (%)</div>
              <div class="setting-desc">Alerta disparado quando ocupação total atingir este valor</div>
            </div>
            <div class="setting-control">
              <input v-model.number="config.threshold" type="number" min="1" max="100" style="width:70px" />
              <span class="unit">%</span>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">Notificação no navegador</div>
              <div class="setting-desc">Exibe toast de alerta quando limite for atingido</div>
            </div>
            <label class="toggle">
              <input v-model="config.notifyBrowser" type="checkbox" />
              <span class="toggle-track"><span class="toggle-thumb" /></span>
            </label>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">E-mail de alerta</div>
              <div class="setting-desc">Endereço para receber notificações por e-mail</div>
            </div>
            <input v-model="config.email" type="email" placeholder="logistica@empresa.com" style="width:220px" />
          </div>
        </div>
      </section>

      <!-- Warehouse layout -->
      <section class="settings-section">
        <div class="section-title">LAYOUT DO ARMAZÉM</div>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">Ruas ativas</div>
              <div class="setting-desc">Selecione quais ruas estão em uso</div>
            </div>
            <div class="setting-control">
              <label v-for="s in ALL_STREETS" :key="s" class="street-toggle">
                <input v-model="config.activeStreets" type="checkbox" :value="s" />
                <span class="street-chip" :class="config.activeStreets.includes(s) ? 'chip-active' : ''">{{ s }}</span>
              </label>
            </div>
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">Posições por rua</div>
              <div class="setting-desc">Número de posições em cada corredor (1–30)</div>
            </div>
            <div class="setting-control">
              <input v-model.number="config.positions" type="number" min="1" max="30" style="width:70px" />
              <span class="unit">pos.</span>
            </div>
          </div>
        </div>
      </section>

      <!-- API connection -->
      <section class="settings-section">
        <div class="section-title">CONEXÃO COM API</div>
        <div class="section-body">
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">URL da API (Rust/Actix-Web)</div>
              <div class="setting-desc">Endpoint base do backend</div>
            </div>
            <input v-model="config.apiBase" type="text" placeholder="http://localhost:8080" style="width:260px;font-family:var(--mono)" />
          </div>
          <div class="setting-row">
            <div class="setting-info">
              <div class="setting-name">URL WebSocket</div>
              <div class="setting-desc">Para atualizações em tempo real</div>
            </div>
            <input v-model="config.wsBase" type="text" placeholder="ws://localhost:8080" style="width:260px;font-family:var(--mono)" />
          </div>
        </div>
      </section>

      <div class="save-row">
        <button class="save-btn" @click="save">Salvar configurações</button>
        <span v-if="saved" class="saved-msg">✓ Salvo com sucesso</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({ layout: 'default' })
const { push } = useAlerts()

const ALL_STREETS = ['A','B','C','D','E','F','G','H']

const config = reactive({
  threshold: 80,
  notifyBrowser: true,
  email: '',
  activeStreets: ['A','B','C','D','E','F'],
  positions: 20,
  apiBase: 'http://localhost:8080',
  wsBase: 'ws://localhost:8080',
})

const saved = ref(false)

function save() {
  // In production: POST to /api/config
  localStorage.setItem('wms-config', JSON.stringify(config))
  saved.value = true
  push({ type: 'success', message: 'Configurações salvas com sucesso' })
  setTimeout(() => saved.value = false, 3000)
}

onMounted(() => {
  const stored = localStorage.getItem('wms-config')
  if (stored) Object.assign(config, JSON.parse(stored))
})
</script>

<style scoped>
.settings-page { display: flex; flex-direction: column; height: 100vh; }

.page-header {
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
  background: var(--bg-1);
  flex-shrink: 0;
}
.page-title { font-size: 13px; font-weight: 600; letter-spacing: .16em; }

.settings-body { flex: 1; overflow-y: auto; padding: 20px; display: flex; flex-direction: column; gap: 16px; max-width: 800px; }

.settings-section {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  overflow: hidden;
}
.section-title {
  padding: 10px 16px;
  font-size: 9px;
  font-weight: 600;
  letter-spacing: .14em;
  color: var(--text-3);
  background: var(--bg-2);
  border-bottom: 1px solid var(--border);
}
.section-body { display: flex; flex-direction: column; }

.setting-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 20px;
  padding: 14px 16px;
  border-bottom: 1px solid var(--border);
}
.setting-row:last-child { border-bottom: none; }
.setting-info  { flex: 1; }
.setting-name  { font-size: 12px; font-weight: 500; color: var(--text); margin-bottom: 2px; }
.setting-desc  { font-size: 11px; color: var(--text-3); }
.setting-control { display: flex; align-items: center; gap: 6px; flex-shrink: 0; }
.unit { font-size: 11px; color: var(--text-3); }

/* Toggle switch */
.toggle { cursor: pointer; }
.toggle input { display: none; }
.toggle-track {
  display: flex;
  width: 36px;
  height: 20px;
  background: var(--bg-3);
  border: 1px solid var(--border);
  border-radius: 10px;
  position: relative;
  transition: background .2s;
}
.toggle input:checked + .toggle-track { background: var(--green-dim); border-color: var(--green); }
.toggle-thumb {
  position: absolute;
  width: 14px;
  height: 14px;
  background: var(--text-3);
  border-radius: 50%;
  top: 2px;
  left: 2px;
  transition: transform .2s, background .2s;
}
.toggle input:checked + .toggle-track .toggle-thumb { transform: translateX(16px); background: var(--green); }

/* Street toggles */
.setting-control { flex-wrap: wrap; max-width: 300px; }
.street-toggle { cursor: pointer; }
.street-toggle input { display: none; }
.street-chip {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border-radius: var(--radius);
  font-size: 11px;
  font-weight: 600;
  border: 1px solid var(--border);
  background: var(--bg-2);
  color: var(--text-3);
  transition: all .12s;
}
.chip-active {
  background: var(--green-bg);
  border-color: var(--green-dim);
  color: var(--green);
}

/* Save row */
.save-row { display: flex; align-items: center; gap: 12px; }
.save-btn {
  height: 36px;
  padding: 0 20px;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: .06em;
  background: var(--green-bg);
  color: var(--green);
  border: 1px solid var(--green-dim);
}
.save-btn:hover { background: #064e1e; border-color: var(--green); }
.saved-msg { font-size: 11px; color: var(--green); }
</style>