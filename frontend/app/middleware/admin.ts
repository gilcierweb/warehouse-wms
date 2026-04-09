// Middleware específico para rotas /admin/*
// Protege automaticamente todas as páginas do admin

import { useAuthStore } from '~/stores/auth'

export default defineNuxtRouteMiddleware(async (to, from) => {
  const authStore = useAuthStore()
  
  // Verifica autenticação
  if (!authStore.isAuthenticated) {
    return navigateTo(`/auth/login?redirect=${encodeURIComponent(to.fullPath)}`)
  }
  
  // Verifica se tem role admin
  const userRoles = authStore.userRoles
  const hasAdminAccess = userRoles.includes('admin')
  
  if (!hasAdminAccess) {
    console.log('[AdminMiddleware] Access denied. User roles:', userRoles)
    return navigateTo('/')
  }
  
  console.log('[AdminMiddleware] Access granted. User roles:', userRoles)
})
