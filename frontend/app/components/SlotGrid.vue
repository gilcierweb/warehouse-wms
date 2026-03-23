<template>
  <div class="street-block" :class="{ 'street-block--danger': street.pct >= 80, 'street-block--warn': street.pct >= 50 && street.pct < 80 }">
    <!-- Header -->
    <div class="street-header">
      <div class="street-name">RUA {{ street.name }}</div>
      <div class="street-stats">
        <span class="stat-item stat-occupied">C:{{ street.occupied }}</span>
        <span class="stat-divider">|</span>
        <span class="stat-item stat-free">V:{{ street.free }}</span>
        <span class="stat-divider">|</span>
        <span class="stat-pct" :class="pctClass">{{ street.pct.toFixed(1) }}%</span>
      </div>
    </div>

    <!-- Progress bar -->
    <div class="street-bar">
      <div class="street-bar-fill" :style="{ width: street.pct + '%' }" :class="pctClass" />
    </div>

    <!-- Lanes -->
    <div class="lanes">
      <div v-for="lane in LANES" :key="lane" class="lane-row">
        <span class="lane-label">{{ lane }}</span>
        <div class="slots-wrap">
          <button
            v-for="slot in slotsByStreet(street.name, lane)"
            :key="slot.id"
            class="slot"
            :class="{
              'slot--free':     slot.status === 'free',
              'slot--occupied': slot.status === 'occupied',
              'slot--selected': selectedSlot === slot.id,
            }"
            :title="slot.id + (slot.sku ? ' · ' + slot.sku : '')"
            @click="$emit('select', slot)"
          >{{ slot.position }}</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Street, Slot } from '~/types'

const props = defineProps<{
  street: Street
  selectedSlot?: string
}>()
defineEmits<{ select: [slot: Slot] }>()

const { LANES, slotsByStreet } = useWarehouseStore()

const pctClass = computed(() => {
  if (props.street.pct >= 80) return 'pct-danger'
  if (props.street.pct >= 50) return 'pct-warn'
  return 'pct-ok'
})
</script>

<style scoped>
.street-block {
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: var(--radius-lg);
  padding: 12px 14px;
  transition: border-color .2s;
}
.street-block--warn   { border-color: #78350f; }
.street-block--danger { border-color: var(--red-dim); }

.street-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
}
.street-name {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: .1em;
  color: var(--text-2);
}
.street-stats { display: flex; align-items: center; gap: 5px; font-size: 10px; }
.stat-occupied { color: var(--red);   font-weight: 500; }
.stat-free     { color: var(--green); font-weight: 500; }
.stat-divider  { color: var(--text-3); }
.stat-pct      { font-weight: 600; }
.pct-ok     { color: var(--green); }
.pct-warn   { color: var(--amber); }
.pct-danger { color: var(--red); }

/* Progress bar */
.street-bar {
  height: 2px;
  background: var(--bg-3);
  border-radius: 1px;
  margin-bottom: 10px;
  overflow: hidden;
}
.street-bar-fill {
  height: 100%;
  border-radius: 1px;
  transition: width .4s;
}
.street-bar-fill.pct-ok     { background: var(--green); }
.street-bar-fill.pct-warn   { background: var(--amber); }
.street-bar-fill.pct-danger { background: var(--red); }

/* Lanes */
.lanes { display: flex; flex-direction: column; gap: 4px; }
.lane-row { display: flex; align-items: center; gap: 8px; }
.lane-label {
  font-size: 9px;
  font-weight: 600;
  letter-spacing: .06em;
  color: var(--text-3);
  width: 20px;
  flex-shrink: 0;
  text-align: right;
}
.slots-wrap { display: flex; gap: 3px; flex-wrap: wrap; }

/* Individual slot */
.slot {
  width: 26px;
  height: 22px;
  border-radius: 3px;
  font-size: 9px;
  font-weight: 500;
  border: 1px solid transparent;
  transition: all .1s;
  line-height: 1;
}
.slot--free {
  background: var(--green-bg);
  color: var(--green);
  border-color: var(--green-dim);
}
.slot--free:hover {
  background: #064e1e;
  border-color: var(--green);
}
.slot--occupied {
  background: var(--red-bg);
  color: var(--red);
  border-color: var(--red-dim);
}
.slot--occupied:hover {
  background: #2d0909;
  border-color: var(--red);
}
.slot--selected {
  outline: 1.5px solid #fff;
  outline-offset: 1px;
}
</style>