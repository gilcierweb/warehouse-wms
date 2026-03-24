import type { Slot, Movement, WarehouseStats } from '~/types'

// Composable for authenticated API calls
export const useAuthApi = () => {
  const config = useRuntimeConfig()
  const { token, logout } = useAuth()
  
  // Base fetch with authentication
  const fetchWithAuth = async <T>(endpoint: string, options: any = {}): Promise<T> => {
    if (!token.value) {
      throw new Error('Not authenticated')
    }
    
    try {
      return await $fetch<T>(`${config.public.apiBase}${endpoint}`, {
        ...options,
        headers: {
          ...options.headers,
          'Authorization': `Bearer ${token.value}`,
          'Content-Type': 'application/json'
        }
      })
    } catch (error: any) {
      // Handle 401 Unauthorized - token expired or invalid
      if (error.response?.status === 401) {
        logout()
        throw new Error('Session expired. Please login again.')
      }
      throw error
    }
  }
  
  // ── Slots ──────────────────────────────────────────────
  async function fetchSlots(): Promise<Slot[]> {
    return fetchWithAuth<Slot[]>('/api/slots')
  }
  
  async function entry(slotId: string, sku?: string, note?: string): Promise<Slot> {
    return fetchWithAuth<Slot>(`/api/slots/${encodeURIComponent(slotId)}/entry`, {
      method: 'POST',
      body: { sku, note }
    })
  }
  
  async function exit(slotId: string, note?: string): Promise<Slot> {
    return fetchWithAuth<Slot>(`/api/slots/${encodeURIComponent(slotId)}/exit`, {
      method: 'POST',
      body: { note }
    })
  }
  
  // ── Stats ──────────────────────────────────────────────
  async function fetchStats(): Promise<WarehouseStats> {
    return fetchWithAuth<WarehouseStats>('/api/stats')
  }
  
  // ── History ────────────────────────────────────────────
  async function fetchMovements(params?: {
    slotId?: string
    type?: 'entry' | 'exit'
    from?: string
    to?: string
    limit?: number
    offset?: number
  }): Promise<Movement[]> {
    const queryParams = new URLSearchParams()
    if (params?.slotId) queryParams.append('slotId', params.slotId)
    if (params?.type) queryParams.append('type', params.type)
    if (params?.from) queryParams.append('from', params.from)
    if (params?.to) queryParams.append('to', params.to)
    if (params?.limit) queryParams.append('limit', params.limit.toString())
    if (params?.offset) queryParams.append('offset', params.offset.toString())
    
    const query = queryParams.toString()
    return fetchWithAuth<Movement[]>(`/api/movements${query ? `?${query}` : ''}`)
  }
  
  async function undoLastMovement(slotId: string): Promise<void> {
    return fetchWithAuth('/api/movements/undo', {
      method: 'POST',
      body: { slotId }
    })
  }
  
  // ── Export ─────────────────────────────────────────────
  function downloadExcel(): void {
    if (!token.value) {
      throw new Error('Not authenticated')
    }
    window.open(`${config.public.apiBase}/api/export/excel?token=${token.value}`, '_blank')
  }
  
  function downloadPdf(): void {
    if (!token.value) {
      throw new Error('Not authenticated')
    }
    window.open(`${config.public.apiBase}/api/export/pdf?token=${token.value}`, '_blank')
  }
  
  return {
    fetchWithAuth,
    fetchSlots,
    entry,
    exit,
    fetchStats,
    fetchMovements,
    undoLastMovement,
    downloadExcel,
    downloadPdf
  }
}
