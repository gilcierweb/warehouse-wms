export default defineNuxtRouteMiddleware((to, from) => {
  const { isAuthenticated, user } = useAuth()
   
  // Public routes - accessible without authentication
  const publicRoutes = ['/login', '/register']
  
  if (publicRoutes.includes(to.path)) {
    // If already authenticated, redirect to home
    if (isAuthenticated.value) {
      return navigateTo('/')
    }
    return
  }
  
  // Protected routes - require authentication
  if (!isAuthenticated.value) {
    // Store the intended destination for redirect after login
    if (import.meta.client) {
      sessionStorage.setItem('redirectAfterLogin', to.fullPath)
    }
    return navigateTo('/login')
  }  
})
