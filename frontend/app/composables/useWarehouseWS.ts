import type { Slot, WarehouseStats, WsMessage } from '~/types'

export const useWarehouseWS = () => {
  const config = useRuntimeConfig()
  const store = useWarehouseStore()
  const { setSlot } = store
  const stats = useState<WarehouseStats | null>('stats', () => null)
  const connected = useState<boolean>('ws-connected', () => false)
  const lastEvent = useState<string>('ws-last-event', () => '')

  let ws: WebSocket | null = null
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null

  function connect() {
    if (ws && ws.readyState === WebSocket.OPEN) return

    console.log('Tentando conectar para:', `${config.public.wsBase}/ws/live`)
    
    ws = new WebSocket(`${config.public.wsBase}/ws/live`)
    
    // Log do ready state inicial
    console.log('WS Ready State inicial:', ws.readyState)

    ws.onopen = () => {
      console.log('WS OPEN: Connection established')
      if (ws) console.log('WS Ready State após open:', ws.readyState)
      connected.value = true
      lastEvent.value = 'Conectado ao servidor'
      if (reconnectTimer) clearTimeout(reconnectTimer)
      
      // Não enviar mensagem automaticamente - esperar backend enviar
      console.log('WS: Conectado, aguardando mensagens do backend...')
    }

    ws.onmessage = (event) => {
      console.log('WS RAW DATA:', event.data)
      
      // Tentar parse como JSON primeiro
      try {
        const msg: WsMessage = JSON.parse(event.data)
        console.log('WS PARSED MSG:', msg)
        if (msg.event === 'slot_updated') {
          const slot = msg.payload as any
          console.log('WS SLOT UPDATE:', slot)
          
          const slotAddress = slot.address || slot.id
          
          const currentSlot = store.getSlot(slotAddress)
          const shouldUpdate = !currentSlot || currentSlot.status !== slot.status || currentSlot.updatedAt !== slot.updatedAt
          
          if (shouldUpdate) {
            const normalizedSlot: Slot = {
              id: slotAddress,
              street: slot.street,
              position: slot.position,
              lane: slot.lane,
              status: slot.status,
              updatedAt: slot.updatedAt || slot.updated_at || new Date().toISOString(),
              updatedBy: slot.updatedBy,
              sku: slot.sku
            }
            
            setSlot(normalizedSlot)
            lastEvent.value = `${slotAddress} → ${slot.status === 'occupied' ? 'OCUPADO' : 'LIVRE'}`
          } else {
            console.log('WS: Ignorando atualização duplicada do slot', slotAddress)
          }
        } else if (msg.event === 'stats_updated') {
          console.log('WS STATS UPDATE:', msg.payload)
          const warehouseStats = msg.payload as WarehouseStats
          stats.value = warehouseStats
          
          // Enviar toast se mudança significativa
          const { push } = useAlerts()
          if (warehouseStats.pct >= 80) {
            push({ 
              type: 'danger', 
              message: `Atenção: Armazém com ${warehouseStats.pct.toFixed(1)}% de ocupação`
            })
          } else if (warehouseStats.pct >= 60) {
            push({ 
              type: 'warning', 
              message: `Armazém com ${warehouseStats.pct.toFixed(1)}% de ocupação`
            })
          }
        } else if (msg.event === 'alert') {
          console.log('WS ALERT:', msg.payload)
          const alert = msg.payload as { message: string; pct: number }
          const { push } = useAlerts()
          push({ type: 'danger', message: alert.message })
        } else {
          console.log('WS UNKNOWN EVENT:', msg)
          const { push } = useAlerts()
          push({ type: 'info', message: `Evento recebido: ${msg.event}` })
        }
      } catch (e) {
        // Se não for JSON, tratar como texto plano
        console.log('WS TEXT MESSAGE:', event.data)
        lastEvent.value = event.data
        
        // Se for mensagem de conexão, não tratar como erro
        if (event.data.includes('Connected') || event.data.includes('connected')) {
          console.log('WS: Mensagem de conexão recebida')
          const { push } = useAlerts()
          push({ type: 'success', message: 'WebSocket conectado' })
        } else {
          console.log('WS: Mensagem texto não processada:', event.data)
          const { push } = useAlerts()
          push({ type: 'info', message: event.data })
        }
      }
    }

    ws.onclose = (event) => {
      console.log('WS CLOSE:', event.code, event.reason, event.wasClean)
      connected.value = false
      lastEvent.value = `Desconectado (${event.code}) — reconectando...`
      reconnectTimer = setTimeout(connect, 3000)
    }

    ws.onerror = (error) => {
      console.log('WS ERROR:', error)
      console.log('WS Ready State no erro:', ws?.readyState)
      ws?.close()
    }
  }

  function disconnect() {
    if (reconnectTimer) clearTimeout(reconnectTimer)
    ws?.close()
    ws = null
  }

  return { stats, connected, lastEvent, connect, disconnect }
}