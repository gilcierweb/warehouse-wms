import type { Slot, WarehouseStats, WsMessage } from '~/types'

declare global {
  interface Window {
    __wsInstance?: WebSocket
    __wsConnected?: boolean
    __wsListeners?: ((data: string) => void)[]
    __wsReconnectTimer?: ReturnType<typeof setTimeout> | null
    __wsOnCloseCallbacks?: Array<(code: number) => void>
  }
}

const DEFAULT_WS_URL = 'ws://localhost:8080/ws'

function getOrCreateState() {
  if (import.meta.client) {
    if (!window.__wsConnected && window.__wsConnected !== false) {
      window.__wsConnected = false
    }
    if (!Array.isArray(window.__wsListeners)) {
      window.__wsListeners = []
    }
    if (!Array.isArray(window.__wsOnCloseCallbacks)) {
      window.__wsOnCloseCallbacks = []
    }
  }
}

function getWsUrl(): string {
  const config = useRuntimeConfig()
  const base = config.public.wsBase || 'ws://localhost:8080'
  return `${base.replace(/\/$/, '')}/ws`
}

export const useWarehouseWS = () => {
  getOrCreateState()
  
  const store = useWarehouseStore()
  const stats = useState<WarehouseStats | null>('ws-stats', () => null)
  const connected = useState<boolean>('ws-connected', () => false)
  const lastEvent = useState<string>('ws-last-event', () => '')
  
  function syncStateFromWindow() {
    if (import.meta.client) {
      connected.value = window.__wsConnected || false
    }
  }

  function connect() {
    if (import.meta.server) return

    const ws = window.__wsInstance
    console.log('[WS] connect() called, ws readyState:', ws?.readyState, 'window.__wsConnected:', window.__wsConnected)
    
    if (ws && ws.readyState === WebSocket.OPEN) {
      console.log('[WS] Already connected')
      window.__wsConnected = true
      connected.value = true
      return
    }

    if (ws && ws.readyState === WebSocket.CONNECTING) {
      console.log('[WS] Already connecting')
      return
    }

    if (ws && (ws.readyState === WebSocket.CLOSED || ws.readyState === WebSocket.CLOSING)) {
      console.log('[WS] Existing WS is closed/closing, cleaning up')
      window.__wsInstance = undefined
    }

    console.log('[WS] Creating new WebSocket connection to:', getWsUrl())
    
    const newWs = new WebSocket(getWsUrl())
    window.__wsInstance = newWs
    window.__wsConnected = false
    connected.value = false

    newWs.onopen = () => {
      console.log('[WS] onopen fired')
      window.__wsConnected = true
      connected.value = true
      lastEvent.value = 'Conectado'
      
      if (window.__wsReconnectTimer) {
        clearTimeout(window.__wsReconnectTimer)
        window.__wsReconnectTimer = null
      }
    }

    newWs.onmessage = (event) => {
      console.log('[WS] onmessage:', event.data)
      
      try {
        const msg: WsMessage = JSON.parse(event.data)
        
        if (msg.event === 'slot_updated') {
          const slot = msg.payload as any
          store.setSlot(slot)
          lastEvent.value = `${slot.address}: ${slot.status}`
        } else if (msg.event === 'stats_updated') {
          stats.value = msg.payload as WarehouseStats
        }
        
        if (Array.isArray(window.__wsListeners)) {
          window.__wsListeners.forEach(cb => cb(event.data))
        }
      } catch {
        lastEvent.value = event.data
      }
    }

    newWs.onclose = (event) => {
      console.log('[WS] onclose fired, code:', event.code, 'wasClean:', event.wasClean)
      window.__wsConnected = false
      connected.value = false
      
      if (window.__wsOnCloseCallbacks) {
        window.__wsOnCloseCallbacks.forEach(cb => cb(event.code))
      }

      if (event.code === 1000) {
        console.log('[WS] Clean close (1000) - not reconnecting')
        window.__wsInstance = undefined
      } else {
        console.log('[WS] Unexpected close - scheduling reconnect in 3s')
        window.__wsReconnectTimer = setTimeout(connect, 3000)
      }
    }

    newWs.onerror = (error) => {
      console.log('[WS] onerror:', error)
    }
  }

  function disconnect() {
    if (import.meta.server) return
    
    console.log('[WS] disconnect() called')
    
    if (window.__wsReconnectTimer) {
      clearTimeout(window.__wsReconnectTimer)
      window.__wsReconnectTimer = null
    }
    
    const ws = window.__wsInstance
    if (ws) {
      console.log('[WS] Closing WebSocket with code 1000')
      ws.onclose = null
      ws.close(1000, 'Client disconnect')
      window.__wsInstance = undefined
    }
    
    window.__wsConnected = false
    connected.value = false
  }

  function onMessage(callback: (data: string) => void) {
    if (!import.meta.client) return () => {}
    
    if (!Array.isArray(window.__wsListeners)) {
      window.__wsListeners = []
    }
    window.__wsListeners.push(callback)
    
    return () => {
      if (Array.isArray(window.__wsListeners)) {
        window.__wsListeners = window.__wsListeners.filter(cb => cb !== callback)
      }
    }
  }

  function onClose(callback: (code: number) => void) {
    if (!import.meta.client) return () => {}
    
    if (!Array.isArray(window.__wsOnCloseCallbacks)) {
      window.__wsOnCloseCallbacks = []
    }
    window.__wsOnCloseCallbacks.push(callback)
    
    return () => {
      if (Array.isArray(window.__wsOnCloseCallbacks)) {
        window.__wsOnCloseCallbacks = window.__wsOnCloseCallbacks.filter(cb => cb !== callback)
      }
    }
  }

  return {
    stats,
    connected,
    lastEvent,
    connect,
    disconnect,
    onMessage,
    onClose,
    isConnected: () => import.meta.client ? (window.__wsConnected || false) : false
  }
}
