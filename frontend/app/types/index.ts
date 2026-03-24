export type SlotStatus = 'free' | 'occupied'

export interface Slot {
  id: string         // UUID
  address: string    // e.g. "A-5-N2"
  street: string     // "A"
  position: number   // 5
  lane: string       // "N2"
  status: SlotStatus
  sku?: string
  updated_by?: string
  created_at: string
  updated_at: string
}

export interface CreateSlotRequest {
  street: string    // A-Z
  position: number  // 1-30
  lane: string      // N1 | N2 | N3
}

export interface UpdateSlotRequest {
  status?: 'free' | 'occupied'
  sku?: string
}

export interface Street {
  name: string
  occupied: number
  free: number
  total: number
  pct: number
  slots: Slot[]
}

export interface Movement {
  id: string
  slotId: string
  type: 'entry' | 'exit'
  operator: string
  sku?: string
  note?: string
  createdAt: string
}

export interface WarehouseStats {
  total: number
  occupied: number
  free: number
  pct: number
  streets: { name: string; pct: number; occupied: number; total: number }[]
}

export interface WsMessage {
  event: 'slot_updated' | 'stats_updated' | 'alert'
  payload: Slot | WarehouseStats | { message: string; pct: number }
}

export interface AlertConfig {
  threshold: number
  notifyEmail: boolean
  notifyBrowser: boolean
}

export interface User {
  id: string
  username: string
  email?: string
  role: number
}

export interface AuthResponse {
  token: string
  user_id: string
  username: string
  role: number
}
