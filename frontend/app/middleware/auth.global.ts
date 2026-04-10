// frontend/app/middleware/auth.global.ts

import type { AuthRouteMeta } from '~/types'
import { useAuthStore } from '~/stores/auth'

import type { Role } from '~/types/auth'

declare module 'vue-router' {
  interface RouteMeta extends AuthRouteMeta {
    requiredRoles?: Role[]
  }
}

/**
 * Global Authentication Middleware
 * 
 * SSR-friendly route protection. 
 * pinia-plugin-persistedstate/nuxt automatically hydrates the store from cookies 
 * on both client and server before route middleware runs.
 */

// Routes that are always public (no auth check needed)
const ALWAYS_PUBLIC_PATHS = ['/', '/404', '/_error']

export default defineNuxtRouteMiddleware(async (to, from) => {
  const path = to.path
  
  // Always allow public paths
  if (ALWAYS_PUBLIC_PATHS.includes(path)) {
    return
  }
  
  // Get route meta
  const meta = (to.meta || {}) as AuthRouteMeta & { requiredRoles?: Role[] }
  const requiresAuth = meta.requiresAuth ?? false
  const guestOnly = meta.guestOnly ?? path.startsWith('/auth/')
  const loginRedirect = meta.loginRedirect || '/auth/login'
  const homeRedirect = meta.homeRedirect || '/'
  const requiredRoles = meta.requiredRoles ? [...meta.requiredRoles] : []
  
  // Auto-detect admin routes and apply admin-like roles requirement
  // Any route starting with /admin automatically requires admin, seller, or moderator role
  const isAdminRoute = path.startsWith('/admin/')
  if (isAdminRoute && requiredRoles.length === 0) {
    requiredRoles.push('admin') // Will be validated with admin-like roles expansion
  }
  
  // Read auth directly from the Pinia store.
  const authStore = useAuthStore()

  // Session Hydration: Sempre tentar refresh na primeira carga ou se tivermos user em cache
  // O refresh token estara no cookie HttpOnly (independente do store de user estar setado)
  const shouldAttemptRefresh = (!authStore.accessToken || authStore.isTokenExpired) 
    && (!!authStore.user || authStore.isInitialHydration)
    && path !== '/auth/login'
  
  if (shouldAttemptRefresh) {
    // SILENT REFRESH via Proxy /api/proxy
    // O proxy permite que o SSR acesse cookies HttpOnly via useRequestHeaders
    if (import.meta.client) {
      try {
        console.log('[Auth] Silent refresh...')
        
        // Usar proxy /api/proxy para preservar cookies HttpOnly
        const data = await $fetch<{ access_token: string, refresh_token: string }>(
          '/api/proxy/auth/refresh',
          {
            method: 'POST',
            body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
            credentials: 'include', // Forward cookies HttpOnly
          }
        )
        
        authStore.setTokens(data.access_token, data.refresh_token)
        console.log('[Auth] Refresh OK')
      } catch (e: any) {
        // Silent fail - não desloga automaticamente
      } finally {
        authStore.isInitialHydration = false
      }
    } else {
      // On Server (SSR): Tentar refresh via proxy
      try {
        
        // O proxy /api/proxy vai usar useRequestHeaders(['cookie']) para pegar cookies
        const data = await $fetch<{ access_token: string, refresh_token: string }>(
          '/api/proxy/auth/refresh',
          {
            method: 'POST',
            headers: useRequestHeaders(['cookie', 'authorization']),
            body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
          }
        )
        
        authStore.setTokens(data.access_token, data.refresh_token)
      } catch {
        // SSR refresh falhou - cliente vai tentar
      } finally {
        authStore.isInitialHydration = false
      }
    }
  } else {
    authStore.isInitialHydration = false
  }

  const isAuthenticated = authStore.isAuthenticated && !authStore.isTokenExpired

  // On the server, if we have a user object, we treat them as "authenticated for SSR purposes"
  // This prevents the server from redirecting to login before the client-side silent refresh
  // has a chance to run and obtain a fresh accessToken via HttpOnly cookies.
  const isServerWithUser = import.meta.server && !!authStore.user

  const userRoles = (authStore.userRoles || []) as Role[]
  
  // Case 1: Route requires authentication
  if (requiresAuth) {
    if (!isAuthenticated && !isServerWithUser) {
      const redirectPath = encodeURIComponent(to.fullPath)
      return navigateTo(`${loginRedirect}?redirect=${redirectPath}`)
    }
    
    // Check role requirements (only if truly authenticated or on client)
    // On server, we skip role checks if we're in this "defer" state
    if (isAuthenticated && requiredRoles.length > 0) {
      // Admin-like roles that can access admin routes
      const adminLikeRoles = ['admin', 'seller', 'moderator'] as Role[]
      
      // Check if user has any required role OR has any admin-like role
      const hasRequiredRole = requiredRoles.some((requiredRole: Role) => 
        userRoles.includes(requiredRole)
      ) || userRoles.some((userRole: Role) => adminLikeRoles.includes(userRole))

      if (!hasRequiredRole) {
        return navigateTo(homeRedirect)
      }
    }
    
    return
  }
  
  // Case 2: Guest-only route (login pages)
  if (guestOnly && isAuthenticated) {
    return navigateTo(homeRedirect)
  }
  
  // Case 3: Public route - allow access
})
