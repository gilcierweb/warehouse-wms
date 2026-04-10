<template>
  <div>
    <AppHeader /> 
    <slot />
    <AppFooter />
    <AppToast />
  </div>
</template>

<script setup>
import { onMounted, watch } from 'vue'

useHead({
  htmlAttrs: {
    class: 'admin-theme'
  }
})

const { isAuthenticated } = useAuth()
const { connect, disconnect, isConnected } = useWarehouseWS()

onMounted(() => {
  if (isAuthenticated.value && !isConnected()) {
    connect()
  }
})

watch(isAuthenticated, (valid) => {
  if (valid && !isConnected()) {
    connect()
  } else if (!valid && isConnected()) {
    disconnect()
  }
})
</script>
