// Middleware específico para rotas /admin/*
// Protege automaticamente todas as páginas do admin

import type { Role } from '~/types/auth'

export default defineNuxtRouteMiddleware(async (to, from) => {
  const authStore = useAuthStore()
  
  // Verifica autenticação
  if (!authStore.isAuthenticated) {
    return navigateTo(`/auth/login?redirect=${encodeURIComponent(to.fullPath)}`)
  }
  
  // Verifica se tem alguma role administrativa
  const adminLikeRoles: Role[] = ['admin', 'seller', 'moderator']
  const userRoles = (authStore.userRoles || []) as Role[]
  
  const hasAdminAccess = userRoles.some((r: Role) => adminLikeRoles.includes(r))
  
  if (!hasAdminAccess) {
    console.log('[AdminMiddleware] Access denied. User roles:', userRoles)
    return navigateTo('/')
  }
  
  console.log('[AdminMiddleware] Access granted. User roles:', userRoles)
})
