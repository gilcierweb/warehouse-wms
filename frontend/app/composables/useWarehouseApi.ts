import type { Slot, Movement, WarehouseStats } from '~/types'

export const useWarehouseApi = () => {
  const config = useRuntimeConfig()
  const base = config.public.apiBase

  // ── Slots ──────────────────────────────────────────────
  async function fetchSlots(): Promise<Slot[]> {
    try {
      return await $fetch<Slot[]>(`${base}/api/slots`)
    } catch (error: any) {
      // Se for erro 404, tentar extrair corpo da resposta
      if (error.name === 'FetchError' && error.response?.status === 404) {
        throw error.response._data || error
      }
      throw error
    }
  }

  async function entry(slotId: string, sku?: string, note?: string) {
    console.log('API: Chamando entry para slot', slotId)
    const response = await fetch(`${base}/api/slots/${encodeURIComponent(slotId)}/entry`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ sku, note }),
    })
  
    if (response.ok) {
      const result = await response.json() as Slot
      console.log('API: Entry sucesso', result)
      return result
    } else {
      // Para erros, tentar extrair corpo da resposta
      const errorData = await response.json().catch(() => ({}))
      console.log('API: Entry erro', errorData)
      throw errorData
    }
  }

  async function exit(slotId: string, note?: string) {
    const response = await fetch(`${base}/api/slots/${encodeURIComponent(slotId)}/exit`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ note }),
    })
  
    if (response.ok) {
      return await response.json() as Slot
    } else {
      // Para erros, tentar extrair corpo da resposta
      const errorData = await response.json().catch(() => ({}))
      throw errorData
    }
  }

  // ── Stats ──────────────────────────────────────────────
  async function fetchStats(): Promise<WarehouseStats> {
    return $fetch<WarehouseStats>(`${base}/api/stats`)
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
    return $fetch<Movement[]>(`${base}/api/movements`, { params })
  }

  async function undoLastMovement(slotId: string) {
    return $fetch(`${base}/api/movements/undo`, {
      method: 'POST',
      body: { slotId },
    })
  }

  // ── Export ─────────────────────────────────────────────
  function downloadExcel() {
    window.open(`${base}/api/export/excel`, '_blank')
  }

  function downloadPdf() {
    window.open(`${base}/api/export/pdf`, '_blank')
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