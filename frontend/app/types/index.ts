// -- API Types

export interface ApiFetchFunction {
  <T = any>(url: string, options?: ApiFetchOptions): Promise<T>
}

export interface ApiFetchOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  body?: any
  headers?: Record<string, string>
  auth?: boolean
}

// -- Auth

export interface User {
  id: string
  email: string
  profile_id?: string
  is_otp_enabled?: boolean
  roles?: string[]
}

export interface AuthResponse {
  access_token: string
  refresh_token: string
  token_type: string
  expires_in: number
  user: User
}

// -- Profile

export interface Profile {
  id: string
  user_id: string
  nickname: string | null
  bio: string | null
  avatar: string | null
  phone: string | null
  status: boolean
  social_network: Record<string, string>
}

// -- Route Meta

export interface AuthRouteMeta {
  requiresAuth?: boolean
  guestOnly?: boolean
  requiredRoles?: string[]
  loginRedirect?: string
  homeRedirect?: string
}

// -- Slot

export interface Slot {
  id: string
  address: string
  street: string
  position: number
  lane: string
  status: string
  sku: string | null
  updated_by: string | null
  created_at: string
  updated_at: string
}

export interface CreateSlotRequest {
  address: string
  street: string
  position: number
  lane: string
  status?: string
}

export interface UpdateSlotRequest {
  address?: string
  street?: string
  position?: number
  lane?: string
  status?: string
  sku?: string | null
}

// -- Movement

export interface Movement {
  id: string
  slot_id: string
  movement_type: number
  operator_id: string | null
  operator_name: string | null
  sku: string | null
  note: string | null
  created_at: string
  updated_at: string
}

// -- Stats

export interface WarehouseStats {
  total_slots: number
  occupied_slots: number
  available_slots: number
  occupancy_rate: number
}
