import type { Slot, Movement, WarehouseStats } from '~/types'

export const useWarehouseApi = () => {
  const config = useRuntimeConfig()
  const base = config.public.apiBase
  const { token, logout } = useAuth()

  // Helper to get headers with auth
  const getHeaders = (contentType = true): Record<string, string> => {
    const headers: Record<string, string> = {}
    if (token.value) {
      headers['Authorization'] = `Bearer ${token.value}`
    }
    if (contentType) {
      headers['Content-Type'] = 'application/json'
    }
    return headers
  }

  // Handle 401 errors
  const handleResponse = async (response: Response) => {
    if (response.status === 401) {
      logout()
      throw new Error('Session expired. Please login again.')
    }
    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}))
      throw errorData
    }
    return response.json()
  }

  // ── Slots ──────────────────────────────────────────────
  async function fetchSlots(): Promise<Slot[]> {
    const response = await fetch(`${base}/api/slots`, {
      headers: getHeaders(false)
    })
    return handleResponse(response)
  }

  async function entry(slotId: string, sku?: string, note?: string): Promise<Slot> {
    console.log('API: Calling entry for slot', slotId)
    const response = await fetch(`${base}/api/slots/${encodeURIComponent(slotId)}/entry`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ sku, note })
    })
    const result = await handleResponse(response)
    console.log('API: Entry success', result)
    return result
  }

  async function exit(slotId: string, note?: string): Promise<Slot> {
    const response = await fetch(`${base}/api/slots/${encodeURIComponent(slotId)}/exit`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ note })
    })
    return handleResponse(response)
  }

  // ── Stats ──────────────────────────────────────────────
  async function fetchStats(): Promise<WarehouseStats> {
    const response = await fetch(`${base}/api/stats`, {
      headers: getHeaders(false)
    })
    return handleResponse(response)
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
    const response = await fetch(`${base}/api/movements${query ? `?${query}` : ''}`, {
      headers: getHeaders(false)
    })
    return handleResponse(response)
  }

  async function undoLastMovement(slotId: string): Promise<void> {
    const response = await fetch(`${base}/api/movements/undo`, {
      method: 'POST',
      headers: getHeaders(),
      body: JSON.stringify({ slotId })
    })
    return handleResponse(response)
  }

  // ── Export ─────────────────────────────────────────────
  function downloadExcel(): void {
    const url = token.value
      ? `${base}/api/export/excel?token=${token.value}`
      : `${base}/api/export/excel`
    window.open(url, '_blank')
  }

  function downloadPdf(): void {
    const url = token.value
      ? `${base}/api/export/pdf?token=${token.value}`
      : `${base}/api/export/pdf`
    window.open(url, '_blank')
  }

  return {
    fetchSlots,
    entry,
    exit,
    fetchStats,
    fetchMovements,
    undoLastMovement,
    downloadExcel,
    downloadPdf,
  }
}