<template>
  <div class="ws-test-page">
    <header class="page-header">
      <h1>Diagnóstico WebSocket</h1>
    </header>

    <div class="test-container">
      <div class="config-section">
        <h2>Configuração</h2>
        <div class="config-item">
          <label>WebSocket URL:</label>
          <code>{{ wsUrl }}</code>
        </div>
        <div class="config-item">
          <label>API Base:</label>
          <code>{{ apiBase }}</code>
        </div>
      </div>

      <div class="status-section">
        <h2>Status da Conexão</h2>
        <div class="status-grid">
          <div class="status-item">
            <span class="status-label">Conectado:</span>
            <span class="status-value" :class="connectedClass">
              {{ ws.connected.value ? 'SIM' : 'NÃO' }}
            </span>
          </div>
          <div class="status-item">
            <span class="status-label">Último Evento:</span>
            <span class="status-value">{{ ws.lastEvent.value || 'Nenhum' }}</span>
          </div>
          <div class="status-item">
            <span class="status-label">Ready State:</span>
            <span class="status-value">{{ readyStateText }}</span>
          </div>
        </div>
      </div>

      <div class="actions-section">
        <h2>Ações</h2>
        <div class="action-buttons">
          <button @click="testConnection" :disabled="testing">
            {{ testing ? 'Testando...' : 'Testar Conexão' }}
          </button>
          <button @click="ws.connect()" :disabled="ws.connected.value">
            Conectar
          </button>
          <button @click="ws.disconnect()" :disabled="!ws.connected.value">
            Desconectar
          </button>
          <button @click="sendMessage" :disabled="!ws.connected.value">
            Enviar Teste
          </button>
        </div>
      </div>

      <div class="log-section">
        <h2>Log de Eventos</h2>
        <div class="log-container">
          <div 
            v-for="(log, index) in logs" 
            :key="index"
            class="log-entry"
            :class="`log-${log.type}`"
          >
            <span class="log-time">{{ log.time }}</span>
            <span class="log-message">{{ log.message }}</span>
          </div>
        </div>
        <button @click="logs = []" class="clear-log">Limpar Log</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'default'
})

const config = useRuntimeConfig()
const ws = useWarehouseWS()
const { push } = useAlerts()

const testing = ref(false)
const logs = ref<Array<{ time: string; message: string; type: string }>>([])

const wsUrl = computed(() => `${config.public.wsBase}/ws`)
const apiBase = computed(() => config.public.apiBase)
const connectedClass = computed(() => ws.connected.value ? 'status-online' : 'status-offline')

const readyStateText = computed(() => {
  // Acessar o WebSocket interno para verificar ready state
  const wsInstance = (ws as any).ws
  if (!wsInstance) return 'N/A'
  
  switch (wsInstance.readyState) {
    case WebSocket.CONNECTING: return 'CONNECTING (0)'
    case WebSocket.OPEN: return 'OPEN (1)'
    case WebSocket.CLOSING: return 'CLOSING (2)'
    case WebSocket.CLOSED: return 'CLOSED (3)'
    default: return 'UNKNOWN'
  }
})

function addLog(message: string, type: 'info' | 'success' | 'error' | 'warning' = 'info') {
  logs.value.unshift({
    time: new Date().toLocaleTimeString(),
    message,
    type
  })
  if (logs.value.length > 50) logs.value.pop()
}

async function testConnection() {
  testing.value = true
  addLog('Iniciando teste de conexão...', 'info')
  
  try {
    // Testar API
    const api = useWarehouseApi()
    await api.fetchSlots()
    addLog('API respondeu com sucesso', 'success')
    
    // Testar WebSocket com diagnóstico
    if (!ws.connected.value) {
      addLog('Tentando conectar WebSocket...', 'info')
      
      // Criar WebSocket manual para diagnóstico
      const testWs = new WebSocket(`${config.public.wsBase}/ws`)
      
      testWs.onopen = () => {
        addLog('WebSocket aberto com sucesso', 'success')
        addLog(`Ready State: ${testWs.readyState}`, 'info')
        
        // Enviar ping se suportado
        try {
          testWs.send(JSON.stringify({ type: 'ping', timestamp: Date.now() }))
          addLog('Ping enviado', 'info')
        } catch (e) {
          addLog(`Erro ao enviar ping: ${e}`, 'warning')
        }
        
        // Fechar após 5 segundos
        setTimeout(() => {
          testWs.close()
        }, 5000)
      }
      
      testWs.onmessage = (event) => {
        addLog(`Mensagem recebida: ${event.data}`, 'success')
      }
      
      testWs.onclose = (event) => {
        addLog(`WebSocket fechado - Code: ${event.code}, Reason: ${event.reason}, Clean: ${event.wasClean}`, 'warning')
        
        // Interpretar códigos comuns
        switch (event.code) {
          case 1000:
            addLog('Fechamento normal', 'info')
            break
          case 1001:
            addLog('Endpoint está indo embora', 'warning')
            break
          case 1002:
            addLog('Erro de protocolo', 'error')
            break
          case 1003:
            addLog('Dados não suportados', 'error')
            break
          case 1006:
            addLog('Conexão fechada anormalmente (backend offline?)', 'error')
            break
          case 1007:
            addLog('Dados UTF-8 inválidos', 'error')
            break
          case 1008:
            addLog('Mensagem viola política', 'error')
            break
          case 1009:
            addLog('Mensagem muito grande', 'error')
            break
          case 1010:
            addLog('Extensão esperada não enviada', 'error')
            break
          case 1011:
            addLog('Erro inesperado no servidor', 'error')
            break
          case 1015:
            addLog('Handshake TLS falhou', 'error')
            break
          default:
            addLog(`Código desconhecido: ${event.code}`, 'warning')
        }
      }
      
      testWs.onerror = (error) => {
        addLog(`Erro WebSocket: ${error}`, 'error')
      }
      
      // Conectar usando o composable também
      ws.connect()
      
      // Esperar um pouco para conectar
      await new Promise(resolve => setTimeout(resolve, 3000))
      
      if (ws.connected.value) {
        addLog('WebSocket conectado com sucesso!', 'success')
      } else {
        addLog('Falha ao manter conexão WebSocket', 'error')
      }
    } else {
      addLog('WebSocket já está conectado', 'success')
    }
  } catch (error) {
    addLog(`Erro no teste: ${error}`, 'error')
  } finally {
    testing.value = false
  }
}

function sendMessage() {
  if (!ws.connected.value) return
  
  // Enviar mensagem de teste (se o backend suportar)
  try {
    const wsInstance = (ws as any).ws
    if (wsInstance && wsInstance.readyState === WebSocket.OPEN) {
      // Testar diferentes formatos
      const testMessages = [
        'test',
        'ping',
        JSON.stringify({ type: 'ping', timestamp: Date.now() }),
        JSON.stringify({ action: 'test' }),
        JSON.stringify({ event: 'test' })
      ]
      
      // Enviar cada mensagem com delay
      testMessages.forEach((msg, index) => {
        setTimeout(() => {
          try {
            wsInstance.send(msg)
            addLog(`Enviado: ${msg}`, 'info')
          } catch (e) {
            addLog(`Erro ao enviar "${msg}": ${e}`, 'error')
          }
        }, index * 1000)
      })
    } else {
      addLog('WebSocket não está em estado OPEN', 'error')
    }
  } catch (error) {
    addLog(`Erro ao enviar mensagem: ${error}`, 'error')
  }
}

// Monitorar eventos do WebSocket
watch(() => ws.connected.value, (connected, prev) => {
  if (connected && !prev) {
    addLog('WebSocket conectado', 'success')
  } else if (!connected && prev) {
    addLog('WebSocket desconectado', 'warning')
  }
})

watch(() => ws.lastEvent.value, (event) => {
  if (event) {
    addLog(`Evento recebido: ${event}`, 'info')
  }
})

onMounted(() => {
  addLog('Página de diagnóstico carregada', 'info')
})
</script>

<style scoped>
.ws-test-page {
  padding: 20px;
  max-width: 1200px;
  margin: 0 auto;
}

.test-container {
  display: grid;
  gap: 20px;
}

.config-section,
.status-section,
.actions-section,
.log-section {
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  padding: 20px;
}

.config-section h2,
.status-section h2,
.actions-section h2,
.log-section h2 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--text);
}

.config-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.config-item label {
  font-weight: 500;
  color: var(--text);
}

.config-item code {
  background: var(--bg-1);
  padding: 4px 8px;
  border-radius: 4px;
  font-family: monospace;
  font-size: 12px;
  color: var(--green);
}

.status-grid {
  display: grid;
  gap: 12px;
}

.status-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.status-label {
  font-weight: 500;
  color: var(--text);
}

.status-value {
  font-weight: 600;
}

.status-online {
  color: var(--green);
}

.status-offline {
  color: var(--red);
}

.action-buttons {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
}

.action-buttons button {
  padding: 8px 16px;
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  color: var(--text);
  cursor: pointer;
  transition: all 0.12s;
}

.action-buttons button:hover:not(:disabled) {
  background: var(--bg-3);
  border-color: var(--border-2);
}

.action-buttons button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.log-container {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: 4px;
  padding: 12px;
  height: 300px;
  overflow-y: auto;
  font-family: monospace;
  font-size: 12px;
}

.log-entry {
  display: flex;
  gap: 12px;
  padding: 2px 0;
  border-bottom: 1px solid var(--border-dim);
}

.log-time {
  color: var(--text-3);
  min-width: 80px;
}

.log-message {
  flex: 1;
}

.log-info { color: var(--text-2); }
.log-success { color: var(--green); }
.log-warning { color: var(--yellow); }
.log-error { color: var(--red); }

.clear-log {
  margin-top: 12px;
  padding: 6px 12px;
  background: var(--red-bg);
  border: 1px solid var(--red-dim);
  color: var(--red);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.clear-log:hover {
  background: var(--red);
  color: white;
}
</style>
