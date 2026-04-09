// Plugin client-only para inicializar autenticação e WebSocket
import { useAuthStore } from '~/stores/auth'
import { useAuth } from '~/composables/useAuth'
import { useWarehouseWS } from '~/composables/useWarehouseWS'

export default defineNuxtPlugin(async (nuxtApp) => {
  const authStore = useAuthStore()
  const { connect, isConnected } = useWarehouseWS()
  const router = useRouter()
  
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

  // Try to connect on init
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

  // Try to connect on app mounted
  nuxtApp.hook('app:mounted', () => {
    console.log('[AuthPlugin] App mounted, checking WS status...')
    tryConnectWS()
  })

  // Try to connect on route change (handles post-login scenario)
  router.beforeEach((to, from, next) => {
    if (authStore.user && authStore.accessToken && !isConnected()) {
      console.log('[AuthPlugin] Route change with user, connecting WS...')
      tryConnectWS()
    }
    next()
  })

  // Also watch for user changes (in case user is set after plugin runs)
  watch(() => authStore.user, (newUser) => {
    if (newUser && authStore.accessToken && !isConnected()) {
      console.log('[AuthPlugin] User changed, connecting WS...')
      tryConnectWS()
    }
  })
})
