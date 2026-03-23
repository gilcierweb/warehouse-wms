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
    // Tentar encontrar por ID primeiro
    let slot = slots.value[id]
    
    // Se não encontrar por ID, tentar por address
    if (!slot) {
      slot = Object.values(slots.value).find(s => (s.address || s.id) === id)
    }
    
    return slot
  }

  function setSlot(slot: any) {
    // Mapear campos da API para o formato do frontend
    const mappedSlot: Slot = {
      id: slot.address || slot.id,
      street: slot.street,
      position: slot.position,
      lane: slot.lane,
      status: slot.status || 'free',
      updatedAt: slot.updatedAt || slot.updated_at || new Date().toISOString(),
      updatedBy: slot.updatedBy || slot.updated_by,
      sku: slot.sku
    }
    slots.value[mappedSlot.id] = mappedSlot
  }

  function bulkLoad(incoming: any[]) {
    for (const s of incoming) {
      // Mapear campos da API para o formato do frontend
      const slot: Slot = {
        id: s.address || s.id,
        street: s.street,
        position: s.position,
        lane: s.lane,
        status: s.status || 'free',
        updatedAt: s.updatedAt || new Date().toISOString(),
        updatedBy: s.updatedBy,
        sku: s.sku
      }
      slots.value[slot.id] = slot
    }
  }

  function slotsByStreet(street: string, lane: string): Slot[] {
    return Array.from({ length: POSITIONS }, (_, i) => {
      const id = `${street}-${i + 1}-${lane}`
      const existingSlot = slots.value[id]
      
      return existingSlot ?? { 
        id, 
        street, 
        position: i + 1, 
        lane, 
        status: 'free', 
        updatedAt: new Date().toISOString() 
      }
    })
  }

  return { slots, streets, globalStats, getSlot, setSlot, bulkLoad, slotsByStreet, STREETS, LANES, POSITIONS }
}