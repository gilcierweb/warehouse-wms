import { computed } from 'vue'
import { useAuthStore } from '~/stores/auth'

// Composable wrapper para manter compatibilidade com código existente
export const useAuth = () => {
  const authStore = useAuthStore()
  
  return {
    // State (readonly para manter compatibilidade)
    token: computed(() => authStore.token || null),
    user: computed(() => authStore.user || null),
    isAuthenticated: computed(() => !!authStore.token),
    isAdmin: computed(() => authStore.user?.role === 1),
    isOperator: computed(() => authStore.user?.role === 2 || authStore.user?.role === 1),
    
    // Actions
    login: authStore.login,
    register: authStore.register,
    recoverPassword: authStore.recoverPassword,
    resetPassword: authStore.resetPassword,
    logout: authStore.logout,
    fetchCurrentUser: authStore.fetchCurrentUser,
    initAuth: authStore.initAuth,
    
    // Legacy compatibility
    setToken: authStore.setToken
  }
}
