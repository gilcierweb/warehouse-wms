import type { Slot, WarehouseStats, WsMessage } from '~/types'

export const useWarehouseWS = () => {
  const config = useRuntimeConfig()
  const slots = useState<Record<string, Slot>>('slots', () => ({}))
  const stats = useState<WarehouseStats | null>('stats', () => null)
  const connected = useState<boolean>('ws-connected', () => false)
  const lastEvent = useState<string>('ws-last-event', () => '')

  let ws: WebSocket | null = null
  let reconnectTimer: ReturnType<typeof setTimeout> | null = null

  function connect() {
    if (ws && ws.readyState === WebSocket.OPEN) return

    ws = new WebSocket(`${config.public.wsBase}/ws/live`)

    ws.onopen = () => {
      connected.value = true
      lastEvent.value = 'Conectado ao servidor'
      if (reconnectTimer) clearTimeout(reconnectTimer)
    }

    ws.onmessage = (event) => {
      try {
        const msg: WsMessage = JSON.parse(event.data)
        if (msg.event === 'slot_updated') {
          const slot = msg.payload as Slot
          slots.value[slot.id] = slot
          lastEvent.value = `${slot.id} → ${slot.status === 'occupied' ? 'OCUPADO' : 'LIVRE'}`
        } else if (msg.event === 'stats_updated') {
          stats.value = msg.payload as WarehouseStats
        } else if (msg.event === 'alert') {
          const alert = msg.payload as { message: string; pct: number }
          useAlerts().push({ type: 'danger', message: alert.message })
        }
      } catch (e) {
        console.error('WS parse error', e)
      }
    }

    ws.onclose = () => {
      connected.value = false
      lastEvent.value = 'Desconectado — reconectando...'
      reconnectTimer = setTimeout(connect, 3000)
    }

    ws.onerror = () => {
      ws?.close()
    }
  }

  function disconnect() {
    if (reconnectTimer) clearTimeout(reconnectTimer)
    ws?.close()
    ws = null
  }

  return { slots, stats, connected, lastEvent, connect, disconnect }
}