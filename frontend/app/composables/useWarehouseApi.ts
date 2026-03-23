import type { Slot, Movement, WarehouseStats } from '~/types'

export const useWarehouseApi = () => {
  const config = useRuntimeConfig()
  const base = config.public.apiBase

  // ── Slots ──────────────────────────────────────────────
  async function fetchSlots(): Promise<Slot[]> {
    return $fetch<Slot[]>(`${base}/api/slots`)
  }

  async function entry(slotId: string, sku?: string, note?: string) {
    return $fetch<Slot>(`${base}/api/slots/${encodeURIComponent(slotId)}/entry`, {
      method: 'POST',
      body: { sku, note },
    })
  }

  async function exit(slotId: string, note?: string) {
    return $fetch<Slot>(`${base}/api/slots/${encodeURIComponent(slotId)}/exit`, {
      method: 'POST',
      body: { note },
    })
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