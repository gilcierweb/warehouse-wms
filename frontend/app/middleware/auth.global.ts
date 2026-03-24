// Middleware global para inicializar autenticação antes de qualquer navegação
export default defineNuxtRouteMiddleware(async (to, from) => {
  console.log('🛡️ Global auth middleware - STARTING...')
  console.log('📍 To:', to.path)
  console.log('🌐 Client-side:', import.meta.client)
  
  // Executar apenas no cliente
  if (import.meta.client) {
    const { initAuth, isAuthenticated, user } = useAuth()
    
    // Verificar se já está inicializado
    if (!isAuthenticated.value && !user.value) {
      console.log('🔄 Auth not initialized, calling initAuth...')
      await initAuth()
      console.log('✅ Global auth middleware - Initialization complete')
      console.log('🔍 Auth state - isAuthenticated:', isAuthenticated.value, 'User:', user.value)
    } else {
      console.log('✅ Auth already initialized, skipping')
    }
  } else {
    console.log('🖥 Server-side, skipping auth initialization')
  }
  
  console.log('🏁 Global auth middleware - ENDING...')
})
