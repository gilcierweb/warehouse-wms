<template>
  <Teleport to="body">
    <div class="toast-container">
      <TransitionGroup name="toast" tag="div">
        <div
          v-for="alert in alerts"
          :key="alert.id"
          class="toast"
          :class="`toast--${alert.type}`"
          @click="remove(alert.id)"
        >
          <div class="toast-icon">
            <span v-if="alert.type === 'success'">✓</span>
            <span v-else-if="alert.type === 'danger'">✕</span>
            <span v-else-if="alert.type === 'warning'">⚠</span>
            <span v-else-if="alert.type === 'info'">ℹ</span>
          </div>
          <div class="toast-content">
            <div class="toast-message">{{ alert.message }}</div>
          </div>
          <button class="toast-close" @click.stop="remove(alert.id)">✕</button>
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
const { alerts, remove } = useAlerts()
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 1000;
  pointer-events: none;
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 400px;
}

.toast {
  pointer-events: all;
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  border-radius: var(--radius);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border: 1px solid;
  background: var(--bg-1);
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  overflow: hidden;
}

.toast::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 4px;
  background: currentColor;
}

.toast:hover {
  transform: translateX(-4px);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.2);
}

.toast--success {
  color: var(--green);
  border-color: var(--green-dim);
  background: var(--green-bg);
}

.toast--danger {
  color: var(--red);
  border-color: var(--red-dim);
  background: var(--red-bg);
}

.toast--warning {
  color: var(--yellow);
  border-color: var(--yellow-dim);
  background: var(--yellow-bg);
}

.toast--info {
  color: var(--blue);
  border-color: var(--blue-dim);
  background: var(--blue-bg);
}

.toast-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: currentColor;
  color: white;
  font-size: 12px;
  font-weight: bold;
  flex-shrink: 0;
}

.toast-content {
  flex: 1;
  min-width: 0;
}

.toast-message {
  font-size: 13px;
  font-weight: 500;
  line-height: 1.4;
  color: var(--text);
  word-break: break-word;
}

.toast-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border: none;
  background: transparent;
  color: var(--text-2);
  cursor: pointer;
  border-radius: 50%;
  font-size: 12px;
  transition: all 0.12s;
  flex-shrink: 0;
}

.toast-close:hover {
  background: rgba(0, 0, 0, 0.1);
  color: var(--text);
}

/* Animations */
.toast-enter-active {
  transition: all 0.3s ease;
}

.toast-leave-active {
  transition: all 0.2s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.toast-move {
  transition: transform 0.3s ease;
}

/* Progress bar */
.toast::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 0;
  height: 2px;
  background: currentColor;
  animation: toast-progress 4s linear forwards;
  opacity: 0.3;
}

@keyframes toast-progress {
  from { width: 100%; }
  to { width: 0%; }
}

/* Mobile responsive */
@media (max-width: 640px) {
  .toast-container {
    left: 12px;
    right: 12px;
    max-width: none;
  }
  
  .toast {
    padding: 10px 12px;
  }
  
  .toast-message {
    font-size: 12px;
  }
}
</style>
