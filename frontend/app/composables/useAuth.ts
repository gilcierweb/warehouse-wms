import type { User, AuthResponse } from '~/types'

// Reactive localStorage helper for token
const useAuthToken = () => {
  const token = useState<string | null>('auth_token', () => {
    if (import.meta.client) {
      return localStorage.getItem('wms_token')
    }
    return null
  })
  
  const setToken = (value: string | null) => {
    token.value = value
    if (import.meta.client) {
      if (value) {
        localStorage.setItem('wms_token', value)
      } else {
        localStorage.removeItem('wms_token')
      }
    }
  }
  
  return { token, setToken }
}

export const useAuth = () => {
  const config = useRuntimeConfig()
  const { token, setToken } = useAuthToken()
  
  // User state
  const user = useState<User | null>('auth_user', () => null)
  const isAuthenticated = computed(() => !!token.value)
  
  // Role helpers
  const isAdmin = computed(() => user.value?.role === 1)
  const isOperator = computed(() => user.value?.role === 2 || user.value?.role === 1)
  
  // Login function
  const login = async (username: string, password: string): Promise<AuthResponse> => {
    const response = await $fetch<AuthResponse>(`${config.public.apiBase}/api/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { username, password }
    })
    
    setToken(response.token)
    user.value = {
      id: response.user_id,
      username: response.username,
      role: response.role
    }
    
    return response
  }
  
  // Register function
  const register = async (
    username: string,
    email: string,
    password: string,
    role: number = 2
  ): Promise<void> => {
    await $fetch(`${config.public.apiBase}/api/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { username, email, password, role }
    })
  }
  
  // Password recovery - send reset email
  const recoverPassword = async (email: string): Promise<{ message: string; token: string }> => {
    const response = await $fetch<{ message: string; token: string }>(`${config.public.apiBase}/api/auth/recover`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { email }
    })
    return response
  }
  
  // Reset password with token
  const resetPassword = async (token: string, password: string): Promise<{ message: string }> => {
    const response = await $fetch<{ message: string }>(`${config.public.apiBase}/api/auth/reset`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: { token, password }
    })
    return response
  }
  
  // Logout function
  const logout = () => {
    setToken(null)
    user.value = null
    navigateTo('/login')
  }
  
  // Fetch current user info
  const fetchCurrentUser = async (): Promise<void> => {
    if (!token.value) return
    
    try {
      const response = await $fetch<{ id: string; username: string; role: number }>(
        `${config.public.apiBase}/api/auth/me`,
        {
          headers: {
            'Authorization': `Bearer ${token.value}`
          }
        }
      )
      
      user.value = {
        id: response.id,
        username: response.username,
        role: response.role
      }
    } catch (error) {
      // Token invalid or expired
      logout()
    }
  }
  
  // Initialize auth state on app start
  const initAuth = async () => {
    if (token.value) {
      await fetchCurrentUser()
    }
  }
  
  return {
    token: readonly(token),
    user: readonly(user),
    isAuthenticated,
    isAdmin,
    isOperator,
    login,
    register,
    recoverPassword,
    resetPassword,
    logout,
    fetchCurrentUser,
    initAuth
  }
}
