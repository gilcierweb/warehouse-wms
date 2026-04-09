// Plugin client-only para inicializar autenticação e WebSocket
// Este plugin roda uma vez quando o app inicia (no primeiro carregamento)
// Não deve rodar novamente em navegações SPA subsequentes
import { useAuthStore } from '~/stores/auth'
import { useAuth } from '~/composables/useAuth'
import { useWarehouseWS } from '~/composables/useWarehouseWS'

let initialized = false

export default defineNuxtPlugin(async (nuxtApp) => {
  if (initialized) {
    console.log('[AuthPlugin] Already initialized, skipping')
    return
  }
  initialized = true
  
  const authStore = useAuthStore()
  const { connect, isConnected } = useWarehouseWS()
  
  console.log('[AuthPlugin] Initializing...', {
    hasUser: !!authStore.user,
    hasToken: !!authStore.accessToken,
  })

  async function tryConnectWS() {
    if (isConnected()) {
      console.log('[AuthPlugin] WS already connected')
      return true
    }
    
    if (authStore.user && authStore.accessToken) {
      console.log('[AuthPlugin] Connecting WebSocket')
      connect()
      return true
    }
    
    console.log('[AuthPlugin] Cannot connect WS yet', {
      hasUser: !!authStore.user,
      hasToken: !!authStore.accessToken
    })
    return false
  }

  if (authStore.user) {
    if (!authStore.accessToken) {
      console.log('[AuthPlugin] Has user but no token, refreshing...')
      try {
        const { refreshAccessToken } = useAuth()
        const newToken = await refreshAccessToken()
        console.log('[AuthPlugin] Token refresh:', newToken ? 'success' : 'failed')
        
        if (newToken) {
          await tryConnectWS()
        }
      } catch (error) {
        console.error('[AuthPlugin] Token refresh error:', error)
      }
    } else {
      await tryConnectWS()
    }
  }

  nuxtApp.hook('app:mounted', () => {
    console.log('[AuthPlugin] App mounted, checking WS status...')
    if (!isConnected()) {
      tryConnectWS()
    }
  })
})
