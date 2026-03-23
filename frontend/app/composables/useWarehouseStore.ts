import type { Slot, Street } from '~/types'

const STREETS = ['A', 'B', 'C', 'D', 'E', 'F']
const LANES   = ['N1', 'N2', 'N3']
const POSITIONS = 20

// Build mock initial slots (replaced by API on mount)
function buildMockSlots(): Record<string, Slot> {
  const map: Record<string, Slot> = {}
  for (const s of STREETS) {
    for (const l of LANES) {
      for (let p = 1; p <= POSITIONS; p++) {
        const id = `${s}-${p}-${l}`
        map[id] = { id, street: s, position: p, lane: l, status: 'free', updatedAt: new Date().toISOString() }
      }
    }
  }
  return map
}

export const useWarehouseStore = () => {
  const slots = useState<Record<string, Slot>>('slots', buildMockSlots)

  const streets = computed<Street[]>(() =>
    STREETS.map(name => {
      const streetSlots = Object.values(slots.value).filter(s => s.street === name)
      const occupied = streetSlots.filter(s => s.status === 'occupied').length
      const total = streetSlots.length
      return {
        name,
        occupied,
        free: total - occupied,
        total,
        pct: total ? Math.round((occupied / total) * 1000) / 10 : 0,
        slots: streetSlots,
      }
    })
  )

  const globalStats = computed(() => {
    const all = Object.values(slots.value)
    const occupied = all.filter(s => s.status === 'occupied').length
    const total = all.length
    return { occupied, free: total - occupied, total, pct: total ? Math.round((occupied / total) * 1000) / 10 : 0 }
  })

  function getSlot(id: string): Slot | undefined {
    return slots.value[id]
  }

  function setSlot(slot: Slot) {
    slots.value[slot.id] = slot
  }

  function bulkLoad(incoming: Slot[]) {
    for (const s of incoming) slots.value[s.id] = s
  }

  function slotsByStreet(street: string, lane: string): Slot[] {
    return Array.from({ length: POSITIONS }, (_, i) => {
      const id = `${street}-${i + 1}-${lane}`
      return slots.value[id] ?? { id, street, position: i + 1, lane, status: 'free', updatedAt: '' }
    })
  }

  return { slots, streets, globalStats, getSlot, setSlot, bulkLoad, slotsByStreet, STREETS, LANES, POSITIONS }
}