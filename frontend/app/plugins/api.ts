import { useAuthStore } from '~/stores/auth'

export default defineNuxtPlugin((nuxtApp) => {
  const config = useRuntimeConfig()
  const baseURL = `${config.public.apiBase}/api`
  const apiKey = config.public.apiKey || 'dev-api-key-change-in-production'

  const apiFetch = async <T>(
    url: string,
    options: {
      method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
      body?: any
      headers?: Record<string, string>
      auth?: boolean
    } = {}
  ): Promise<T> => {
    const authStore = useAuthStore()

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      Accept: 'application/json',
      'X-API-Key': apiKey,
    }

    if (options.auth && authStore.accessToken) {
      headers['Authorization'] = `Bearer ${authStore.accessToken}`
    }

    if (options.headers) {
      Object.assign(headers, options.headers)
    }

    try {
      const response = await $fetch<T>(`${baseURL}${url}`, {
        method: options.method || 'GET',
        headers,
        body: options.method !== 'GET' ? options.body : undefined,
        credentials: 'include',
      })
      return response
    } catch (error: any) {
      if (options.auth && error.statusCode === 401) {
        try {
          const refreshData = await $fetch<any>(`${baseURL}/auth/refresh`, {
            method: 'POST',
            headers: {
              'X-API-Key': apiKey,
            },
            credentials: 'include',
          })
          authStore.setTokens(refreshData.access_token, null)

          const retryResponse = await $fetch<T>(`${baseURL}${url}`, {
            method: options.method || 'GET',
            headers: {
              ...headers,
              Authorization: `Bearer ${refreshData.access_token}`,
            },
            body: options.method !== 'GET' ? options.body : undefined,
            credentials: 'include',
          })
          return retryResponse
        } catch {
          authStore.logout()
          throw createError({ statusCode: 401, statusMessage: 'Session expired' })
        }
      }
      throw error
    }
  }

  nuxtApp.provide('apiFetch', apiFetch)
  nuxtApp.provide('api', apiFetch)
})
