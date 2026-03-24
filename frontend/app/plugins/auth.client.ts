// Plugin client-only para inicializar autenticação - executa apenas no cliente
import { nextTick } from 'vue'

export default defineNuxtPlugin(async () => {
  
  // Executar apenas no cliente
  if (import.meta.client) {
    try {
      const { initAuth } = useAuth()
      initAuth() // Background validation
    } catch (error) {
      console.error('Auth CLIENT plugin error:', error)
    }
  } else {
    console.log('Server-side detected, skipping client auth initialization')
  }  

})
