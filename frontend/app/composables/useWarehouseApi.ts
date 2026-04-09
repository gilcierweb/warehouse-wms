import type { Slot, Movement, WarehouseStats, CreateSlotRequest, UpdateSlotRequest } from '~/types'
import { useAuthStore } from '~/stores/auth'

export const useWarehouseApi = () => {
  const { authFetch } = useAuth()

  async function fetchSlots(): Promise<Slot[]> {
    return authFetch('/slots')
  }

  async function getSlotById(id: string): Promise<Slot> {
    return authFetch(`/slots/${id}`)
  }

  async function createSlot(slot: CreateSlotRequest): Promise<Slot> {
    return authFetch('/slots', {
      method: 'POST',
      body: slot,
    })
  }

  async function updateSlot(id: string, slot: UpdateSlotRequest): Promise<Slot> {
    return authFetch(`/slots/${id}`, {
      method: 'PUT',
      body: slot,
    })
  }

  async function deleteSlot(id: string): Promise<void> {
    return authFetch(`/slots/${id}`, {
      method: 'DELETE',
    })
  }

  async function entry(slotId: string, sku?: string, note?: string): Promise<Slot> {
    return authFetch(`/slots/${encodeURIComponent(slotId)}/entry`, {
      method: 'POST',
      body: { sku, note },
    })
  }

  async function exit(slotId: string, note?: string): Promise<Slot> {
    return authFetch(`/slots/${encodeURIComponent(slotId)}/exit`, {
      method: 'POST',
      body: { note },
    })
  }

  async function fetchStats(): Promise<WarehouseStats> {
    return authFetch('/stats')
  }

  async function fetchMovements(params?: {
    slotId?: string
    type?: 'entry' | 'exit'
    from?: string
    to?: string
    limit?: number
    offset?: number
  }): Promise<Movement[]> {
    const queryParams = new URLSearchParams()
    if (params?.slotId) queryParams.append('slot_address', params.slotId)
    if (params?.type) queryParams.append('type', params.type)
    if (params?.from) queryParams.append('from', params.from)
    if (params?.to) queryParams.append('to', params.to)
    if (params?.limit) queryParams.append('limit', params.limit.toString())
    if (params?.offset) queryParams.append('offset', params.offset.toString())

    const query = queryParams.toString()
    return authFetch(`/movements/filtered${query ? `?${query}` : ''}`)
  }

  async function undoLastMovement(slotId: string): Promise<void> {
    return authFetch('/movements/undo', {
      method: 'POST',
      body: { slotId },
    })
  }

  function downloadExcel(): void {
    const authStore = useAuthStore()
    const url = authStore.accessToken
      ? `${useRuntimeConfig().public.apiBase}/api/export/excel?token=${authStore.accessToken}`
      : `${useRuntimeConfig().public.apiBase}/api/export/excel`
    window.open(url, '_blank')
  }

  function downloadPdf(): void {
    const authStore = useAuthStore()
    const url = authStore.accessToken
      ? `${useRuntimeConfig().public.apiBase}/api/export/pdf?token=${authStore.accessToken}`
      : `${useRuntimeConfig().public.apiBase}/api/export/pdf`
    window.open(url, '_blank')
  }

  return {
    fetchSlots,
    getSlotById,
    createSlot,
    updateSlot,
    deleteSlot,
    entry,
    exit,
    fetchStats,
    fetchMovements,
    undoLastMovement,
    downloadExcel,
    downloadPdf,
  }
}
