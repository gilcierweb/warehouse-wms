// Middleware global para inicializar autenticação antes de qualquer navegação
export default defineNuxtRouteMiddleware(async (to, from) => {
  
  // Executar apenas no cliente
  if (import.meta.client) {
    const { initAuth, isAuthenticated, user } = useAuth()
    
    // Verificar se já está inicializado
    if (!isAuthenticated.value && !user.value) {
      await initAuth()
    } else {
      console.log('Auth already initialized, skipping')
    }
  } else {
    console.log('Server-side, skipping auth initialization')
  }
  
})
