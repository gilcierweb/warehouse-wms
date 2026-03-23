export interface Alert {
  id: string
  type: 'success' | 'danger' | 'warning' | 'info'
  message: string
}

const alerts = ref<Alert[]>([])

export const useAlerts = () => {
  function push(a: Omit<Alert, 'id'>, duration = 4000) {
    const id = Math.random().toString(36).slice(2)
    alerts.value.push({ ...a, id })
    setTimeout(() => remove(id), duration)
  }

  function remove(id: string) {
    alerts.value = alerts.value.filter(a => a.id !== id)
  }

  return { alerts: readonly(alerts), push, remove }
}