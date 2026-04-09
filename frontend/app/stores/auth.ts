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
    // Simply check if user exists - token refresh is handled separately
    return !!user.value
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
    if (u?.roles) {
      userRoles.value = u.roles
    }
  }

  function setProfile(p: Profile | null) {
    profile.value = p
  }

  function setTokens(access: string | null, refresh: string | null) {
    accessToken.value = access
    refreshToken.value = refresh
    
    if (access) {
      try {
        const payload = parseJwt(access)
        userRoles.value = payload.roles || []
        
        if (!user.value && payload.sub) {
          user.value = {
            id: payload.sub,
            email: '',
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
  }

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
  persist: {
    // Tokens are NOT persisted here for security!
    // Access token stays in memory
    // Refresh token is in HttpOnly cookie (managed by backend)
    pick: ['user', 'profile', 'userRoles']
  }
})
