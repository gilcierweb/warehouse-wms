import { defineStore } from 'pinia'
import type { User, AuthResponse } from '~/types'

export const useAuthStore = defineStore('auth', () => {
  // State (using useCookie for SSR support)
  const token = useCookie<string | null>('wms_token', { 
    maxAge: 60 * 60 * 24 * 7, 
    path: '/',
    default: () => null 
  })
  
  const user = useCookie<User | null>('wms_user', { 
    maxAge: 60 * 60 * 24 * 7, 
    path: '/',
    default: () => null 
  })
  
  const isLoading = ref(false)

  // Getters
  const isAuthenticated = computed(() => !!token.value)
  const isAdmin = computed(() => user.value?.role === 1)
  const isOperator = computed(() => user.value?.role === 2 || user.value?.role === 1)

  const setToken = (newToken: string | null) => {
    token.value = newToken
  }

  const setUser = (newUser: User | null) => {
    user.value = newUser
  }

  const login = async (username: string, password: string): Promise<AuthResponse> => {
    const config = useRuntimeConfig()
    
    try {
      isLoading.value = true
      
      const response = await $fetch<AuthResponse>(`${config.public.apiBase}/api/auth/login`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { username, password }
      })
      
      setToken(response.token)
      setUser({
        id: response.user_id,
        username: response.username,
        role: response.role
      })
      
      return response
    } finally {
      isLoading.value = false
    }
  }

  const register = async (
    username: string,
    email: string,
    password: string,
    role: number = 2
  ): Promise<void> => {
    const config = useRuntimeConfig()
    
    await $fetch(`${config.public.apiBase}/api/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { username, email, password, role }
    })
  }

  const recoverPassword = async (email: string): Promise<{ message: string; token: string }> => {
    const config = useRuntimeConfig()
    
    const response = await $fetch<{ message: string; token: string }>(
      `${config.public.apiBase}/api/auth/recover`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { email }
      }
    )
    
    return response
  }

  const resetPassword = async (token: string, password: string): Promise<{ message: string }> => {
    const config = useRuntimeConfig()
    
    const response = await $fetch<{ message: string }>(
      `${config.public.apiBase}/api/auth/reset`,
      {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: { token, password }
      }
    )
    
    return response
  }

  const fetchCurrentUser = async (): Promise<void> => {
    if (!token.value) return
    
    const config = useRuntimeConfig()
    
    try {
      const response = await $fetch<{ id: string; username: string; role: number }>(
        `${config.public.apiBase}/api/auth/me`,
        {
          headers: {
            'Authorization': `Bearer ${token.value}`
          }
        }
      )
      
      setUser({
        id: response.id,
        username: response.username,
        role: response.role
      })
    } catch (error) {
      console.error('❌ Token validation failed:', error)
      // Token invalid or expired - clear auth state
      logout()
    }
  }

  const logout = () => {
    setToken(null)
    setUser(null)
    
    if (import.meta.client) {
      navigateTo('/login')
    }
  }

  const initAuth = () => {
    console.log('🚀 initAuth called')
    
    // Since we use useCookie, the token and user are already populated on both server and client.
    // We only need to asynchronously validate the token with the server if we have one.
    if (import.meta.client && token.value) {
      console.log('🔍 Validating cookie token with server...')
      fetchCurrentUser().catch(error => {
        console.error('❌ Server validation failed:', error)
      })
    }
  }

  return {
    // State
    token: readonly(token),
    user: readonly(user),
    isLoading: readonly(isLoading),
    
    // Getters
    isAuthenticated,
    isAdmin,
    isOperator,
    
    // Actions
    setToken,
    setUser,
    login,
    register,
    recoverPassword,
    resetPassword,
    fetchCurrentUser,
    logout,
    initAuth
  }
})
