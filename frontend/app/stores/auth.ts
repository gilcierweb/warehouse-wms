import { defineStore } from 'pinia'
import type { User, Profile } from '~/types'

interface JwtPayload {
  sub: string
  exp: number
  iat: number
  roles?: string[]
}

/**
 * Authentication Store - SSR-friendly with cookies
 * 
 * Features:
 * - Uses cookies for persistence (SSR-compatible via pinia-plugin-persistedstate/nuxt)
 * - Reactive authentication state
 * - JWT payload parsing for user info and roles
 * - Automatic hydration on server and client
 */
export const useAuthStore = defineStore('auth', () => {
  // State - all persisted to cookies automatically
  const user = ref<User | null>(null)
  const profile = ref<Profile | null>(null)
  const accessToken = ref<string | null>(null)
  const refreshToken = ref<string | null>(null)
  const userRoles = ref<string[]>([])
  const isInitialHydration = ref(true)

  // Getters
  const isAuthenticated = computed(() => {
    // If we have a user and (token exists OR we are still hydrating), consider auth'd
    return !!user.value && (!!accessToken.value || isInitialHydration.value)
  })
  
  const isTokenExpired = computed(() => {
    if (!accessToken.value) return true
    try {
      const payload = parseJwt(accessToken.value)
      return payload.exp * 1000 < Date.now()
    } catch {
      return true
    }
  })

  const hasRole = computed(() => (role: string) => 
    userRoles.value.includes(role) || userRoles.value.includes('admin')
  )

  const hasAnyRole = computed(() => (roles: string[]) => 
    roles.some(role => userRoles.value.includes(role)) || userRoles.value.includes('admin')
  )

  // Actions
  function setUser(u: User | null) {
    user.value = u
  }

  function setProfile(p: Profile | null) {
    profile.value = p
  }

  function setTokens(access: string | null, refresh: string | null) {
    accessToken.value = access
    refreshToken.value = refresh
    
    // Parse JWT for user info and roles
    if (access) {
      try {
        const payload = parseJwt(access)
        userRoles.value = payload.roles || []
        
        // Reconstruct minimal user from JWT if not already set
        if (!user.value && payload.sub) {
          user.value = {
            id: payload.sub,
            email: '',
            confirmed_at: null,
            totp_enabled: false,
            created_at: '',
          }
        }
      } catch {
        userRoles.value = []
      }
    } else {
      userRoles.value = []
    }
  }

  function logout() {
    user.value = null
    profile.value = null
    accessToken.value = null
    refreshToken.value = null
    userRoles.value = []
    // Cookies are cleared automatically by pinia-plugin-persistedstate
  }

  // Helper to parse JWT payload
  function parseJwt(token: string): JwtPayload {
    const parts = token.split('.')
    if (parts.length !== 3) {
      throw new Error('Invalid JWT format')
    }
    
    const payloadPart = parts[1]
    if (!payloadPart) {
      throw new Error('Invalid JWT payload')
    }
    
    const padding = 4 - (payloadPart.length % 4)
    const base64 = padding === 4 ? payloadPart : payloadPart + '='.repeat(padding)
    
    return JSON.parse(atob(base64.replace(/-/g, '+').replace(/_/g, '/')))
  }

  return {
    // State
    user,
    profile,
    accessToken,
    refreshToken,
    userRoles,
    // Getters
    isAuthenticated,
    isTokenExpired,
    hasRole,
    hasAnyRole,
    // Actions
    setUser,
    setProfile,
    setTokens,
    logout,
    parseJwt,
    isInitialHydration,
  }
}, {
  // Persistence via cookies - NUNCA persistir tokens!
  // Tokens (access/refresh) ficam em memória (ref) ou HttpOnly cookies (backend)
  persist: {
    pick: ['user', 'profile', 'userRoles'],  // Apenas dados públicos
    storage: 'cookies'
    // accessToken e refreshToken NÃO estão na lista - memória apenas
  }
})
