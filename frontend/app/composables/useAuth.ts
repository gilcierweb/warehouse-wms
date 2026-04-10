// frontend/app/composables/useAuth.ts

import type { AuthResponse } from '~/types'
import { useAuthStore } from '~/stores/auth'

import type { Role } from '~/types/auth'

interface LoginRequiresOtpResponse {
    requires_otp: boolean
    message?: string
}

/**
 * Authentication composable — handles login, register, token management.
 * Uses Pinia store with cookie persistence (SSR-friendly via pinia-plugin-persistedstate/nuxt)
 */
export const useAuth = () => {
    const config = useRuntimeConfig()
    const router = useRouter()
    const authStore = useAuthStore()
    const route = useRoute()

    const user = computed(() => authStore.user)
    const loading = ref(false)
    const error = ref<string | null>(null)
    const { t } = useI18n()

    const accessToken = computed(() => authStore.accessToken)

    const isAuthenticated = computed(() => authStore.isAuthenticated)

    async function register(email: string, password: string, passwordConfirmation?: string) {
        loading.value = true
        error.value = null
        try {
            const res = await $fetch('/api/proxy/auth/register', {
                method: 'POST',
                body: {
                    email,
                    password,
                    password_confirmation: passwordConfirmation ?? password,
                },
            })
            return res
        } catch (e: any) {
            error.value = e?.data?.message || t('auth.errors.registrationFailed')
            throw e
        } finally {
            loading.value = false
        }
    }

    async function login(email: string, password: string, otpCode?: string) {
        loading.value = true
        error.value = null
        let loginSuccess = false
        let targetPath = '/admin/dashboard'

        try {
            const data = await $fetch<AuthResponse | LoginRequiresOtpResponse>('/api/proxy/auth/login', {
                method: 'POST',
                body: { email, password, otp_code: otpCode || undefined },
                credentials: 'include', // Crucial for HttpOnly cookies
            })

            if ('requires_otp' in data && data.requires_otp) {
                throw {
                    data: {
                        message: data.message || '2FA required',
                        requires_otp: true,
                    },
                }
            }

            // Type guard: agora sabemos que é AuthResponse
            const authData = data as AuthResponse

            // Store tokens in Pinia (access_token stays in memory)
            authStore.setTokens(authData.access_token, authData.refresh_token)
            authStore.setUser(authData.user)

            // Upload keys if not yet done
            const keyStore = useKeyStore()
            await keyStore.ensureKeys(authData.access_token)

            // Determine redirect target
            const redirect = route.query.redirect as string
            targetPath = redirect ? decodeURIComponent(redirect) : '/'
            loginSuccess = true

        } catch (e: any) {
            error.value = e?.data?.message || t('auth.errors.loginFailed')
            throw e
        } finally {
            loading.value = false
        }

        // Navigate after successful login (outside try-catch)
        if (loginSuccess) {
            return navigateTo(targetPath)
        }
    }

    async function logout(preventRedirect = false) {
        const token = authStore.accessToken
        if (token) {
            try {
                await $fetch('/api/proxy/auth/logout', {
                    method: 'POST',
                    headers: {
                        Authorization: `Bearer ${token}`
                    },
                    credentials: 'include', // Clear the backend HttpOnly cookie
                })
            } catch {
            }
        }

        // Clear everything via store (cookies cleared automatically)
        authStore.logout()

        if (!preventRedirect) {
            if (import.meta.client) {
                await router.push('/auth/login')
            } else {
                return navigateTo('/auth/login')
            }
        }
    }

    async function refreshAccessToken(): Promise<string | null> {
        try {
            // Usar proxy /api/proxy para refresh (preserva cookies HttpOnly)
            const data = await $fetch<{ access_token: string }>('/api/proxy/auth/refresh', {
                method: 'POST',
                headers: {
                    'X-API-Key': config.public.apiKey,
                    ...(import.meta.server ? useRequestHeaders(['cookie']) : {})
                },
                credentials: 'include',
            })

            // Update tokens in store
            authStore.setTokens(data.access_token, null)

            return data.access_token
        } catch {
            // Nao faz logout - apenas retorna null para o caller tratar
            return null
        }
    }

    async function forgotPassword(email: string) {
        loading.value = true
        error.value = null
        try {
            return await $fetch('/api/proxy/auth/recover', {
                method: 'POST',
                body: { email },
            })
        } catch (e: any) {
            error.value = e?.data?.message || t('auth.errors.unknown')
            throw e
        } finally {
            loading.value = false
        }
    }

    async function resetPassword(token: string, password: string, passwordConfirmation: string) {
        loading.value = true
        error.value = null
        try {
            return await $fetch('/api/proxy/auth/reset', {
                method: 'POST',
                body: {
                    token,
                    password,
                    password_confirmation: passwordConfirmation,
                },
            })
        } catch (e: any) {
            error.value = e?.data?.message || t('auth.errors.unknown')
            throw e
        } finally {
            loading.value = false
        }
    }

    // Use $apiFetch from plugin - re-export with auth: true for convenience
    const { $apiFetch } = useNuxtApp()
    const authFetch = <T>(url: string, opts: {
        method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
        body?: any
        headers?: Record<string, string>
    } = {}) => $apiFetch<T>(url, { ...opts, auth: true })

    // Role helpers (non-breaking enhancements)
    const userRoles = computed<Role[]>(() => (authStore.userRoles || []) as Role[])

    const hasRole = computed(() => authStore.hasRole)
    const hasAnyRole = computed(() => authStore.hasAnyRole)

    const isBidder = computed(() => userRoles.value.includes('bidder'))
    const isSeller = computed(() => userRoles.value.includes('seller'))
    const isAdmin = computed(() => userRoles.value.includes('admin'))
    const isModerator = computed(() => userRoles.value.includes('moderator'))

    // Future-ready permission check (optional)
    const can = (permission: string) => {
        return authStore.permissions?.includes(permission) || isAdmin.value
    }

    return {
        user,
        loading,
        error,
        isAuthenticated,
        accessToken,

        hasRole,
        hasAnyRole,
        userRoles,

        // Non-breaking additions
        isBidder,
        isSeller,
        isAdmin,
        isModerator,
        can,

        register,
        login,
        logout,
        authFetch,
        refreshAccessToken,
        forgotPassword,
        resetPassword,
    }
}