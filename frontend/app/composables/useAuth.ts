// frontend/app/composables/useAuth.ts

import type { AuthResponse } from '~/types'
import { useAuthStore } from '~/stores/auth'

/**
 * Authentication composable — handles login, register, token management.
 */
export const useAuth = () => {
    const { $api } = useNuxtApp()
    const router = useRouter()
    const authStore = useAuthStore()
    const route = useRoute()

    const user = computed(() => authStore.user)
    const loading = ref(false)
    const error = ref<string | null>(null)
    const { t } = useI18n()

    const accessToken = computed(() => authStore.accessToken)
    const isAuthenticated = computed(() => authStore.isAuthenticated)
    const userRoles = computed(() => authStore.userRoles)

    async function register(email: string, password: string, passwordConfirmation?: string) {
        loading.value = true
        error.value = null
        try {
            const res = await $api('/auth/register', {
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
        let targetPath = '/'

        try {
            const data = await $api<AuthResponse>('/auth/login', {
                method: 'POST',
                body: { email, password, otp_code: otpCode || undefined },
            })

            authStore.setTokens(data.access_token, data.refresh_token)
            authStore.setUser(data.user)

            const redirect = route.query.redirect as string
            targetPath = redirect ? decodeURIComponent(redirect) : '/'
            loginSuccess = true

        } catch (e: any) {
            error.value = e?.data?.message || t('auth.errors.loginFailed')
            throw e
        } finally {
            loading.value = false
        }

        if (loginSuccess) {
            return navigateTo(targetPath)
        }
    }

    async function logout(preventRedirect = false) {
        const token = authStore.accessToken
        if (token) {
            try {
                await $api('/auth/logout', {
                    method: 'POST',
                    headers: {
                        Authorization: `Bearer ${token}`
                    },
                })
            } catch {
            }
        }

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
            const data = await $api<{ access_token: string }>('/auth/refresh', {
                method: 'POST',
                body: authStore.refreshToken ? { refresh_token: authStore.refreshToken } : {},
            })

            authStore.setTokens(data.access_token, authStore.refreshToken)
            return data.access_token
        } catch {
            return null
        }
    }

    async function forgotPassword(email: string) {
        loading.value = true
        error.value = null
        try {
            return await $api('/auth/recover', {
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
            return await $api('/auth/reset', {
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

    const authFetch = <T>(url: string, opts: {
        method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
        body?: any
        headers?: Record<string, string>
    } = {}) => $api<T>(url, { ...opts, auth: true })

    async function initAuth(): Promise<void> {
        if (!authStore.accessToken) return
        
        try {
            const userData = await $api<any>('/auth/me', {
                auth: true,
            })
            authStore.setUser(userData)
        } catch {
            authStore.logout()
        } finally {
            authStore.isInitialHydration = false
        }
    }

    return {
        user,
        loading,
        error,
        isAuthenticated,
        accessToken,
        userRoles,
        register,
        login,
        logout,
        authFetch,
        refreshAccessToken,
        forgotPassword,
        resetPassword,
        initAuth,
    }
}
