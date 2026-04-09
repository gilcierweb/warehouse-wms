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
  const guestOnly = meta.guestOnly ?? path.startsWith('/auth/login')
  const loginRedirect = meta.loginRedirect || '/auth/login'
  const homeRedirect = meta.homeRedirect || '/'
  const requiredRoles = meta.requiredRoles || []
  
  // Auto-detect admin routes and apply admin-like roles requirement
  const isAdminRoute = path.startsWith('/admin/')
  if (isAdminRoute && requiredRoles.length === 0) {
    requiredRoles.push('admin')
  }
  
  // Read auth directly from the Pinia store.
  const authStore = useAuthStore()

  // Session Hydration: try refresh if we have user but no valid token
  const shouldAttemptRefresh = (!authStore.accessToken || authStore.isTokenExpired) 
    && authStore.user 
    && path !== '/auth/login'
  
  if (shouldAttemptRefresh) {
    if (import.meta.client) {
      try {
        console.log('[Auth] Silent refresh...')
        
        const data = await $fetch<{ access_token: string }>(
          '/api/auth/refresh',
          {
            method: 'POST',
            body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
            credentials: 'include',
          }
        )
        
        authStore.setTokens(data.access_token, authStore.refreshToken)
        console.log('[Auth] Refresh OK')
      } catch (e: any) {
        // Silent fail
      } finally {
        authStore.isInitialHydration = false
      }
    } else {
      try {
        const data = await $fetch<{ access_token: string }>(
          `${useRuntimeConfig().public.apiBase}/api/auth/refresh`,
          {
            method: 'POST',
            headers: useRequestHeaders(['cookie', 'authorization']),
            body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
          }
        )
        
        authStore.setTokens(data.access_token, authStore.refreshToken)
      } catch {
        authStore.isInitialHydration = false
      }
    }
  }

  // Don't modify isAuthenticated or hydration state here - let the store handle it
  const isAuthenticated = authStore.isAuthenticated && !authStore.isTokenExpired

  // On the server, if we have a user object, we treat them as "authenticated for SSR purposes"
  const isServerWithUser = import.meta.server && !!authStore.user

  const userRoles = authStore.userRoles as Role[]
  
  // Case 1: Route requires authentication
  if (requiresAuth) {
    if (!isAuthenticated && !isServerWithUser) {
      const redirectPath = encodeURIComponent(to.fullPath)
      return navigateTo(`${loginRedirect}?redirect=${redirectPath}`)
    }
    
    // Check role requirements
    if (isAuthenticated && requiredRoles.length > 0) {
      const adminLikeRoles = ['admin', 'seller', 'moderator'] as Role[]
      
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
