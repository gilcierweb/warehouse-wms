// app/composables/useApi.ts
// SSR-friendly data fetching com useFetch

import { useAuthStore } from '~/stores/auth'

interface UseApiOptions {
  method?: 'get' | 'post' | 'put' | 'delete' | 'patch'
  body?: any
  headers?: Record<string, string>
  query?: Record<string, string>
  server?: boolean
  immediate?: boolean
  watch?: any[]
  default?: () => any
  onResponseError?: (context: { response: any }) => void
}

export function useApi<T>(
  url: string | (() => string),
  options: UseApiOptions = {}
) {
  const config = useRuntimeConfig()
  const baseURL = `${config.public.apiBase}/api`
  
  const headers = {
    'X-API-Key': config.public.apiKey,
    'Content-Type': 'application/json',
    ...useRequestHeaders(['cookie', 'authorization']),
    ...options.headers,
  }
  
  const fullUrl = computed(() => {
    const path = typeof url === 'function' ? url() : url
    if (!path || path === 'null' || path === 'undefined' || path === '') {
      return ''
    }
    const queryString = options.query 
      ? '?' + new URLSearchParams(options.query).toString()
      : ''
    return `${baseURL}${path}${queryString}`
  })
  
  const { data, pending, error, refresh, execute, status } = useFetch<T>(fullUrl, {
    method: (options.method || 'get') as any,
    headers,
    body: options.body,
    server: options.server !== false,
    immediate: options.immediate !== false,
    watch: options.watch,
    default: options.default,
    onResponseError({ response }) {
      console.error('[useApi] Error:', response.status, response._data)
    },
  })
  
  return {
    data,
    pending,
    loading: pending,
    error,
    refresh,
    execute,
    status,
  }
}

export function useApiLazy<T>(
  url: string | (() => string),
  options: UseApiOptions = {}
) {
  return useApi<T>(url, {
    ...options,
    immediate: false,
  })
}

export function useAuthApi<T>(
  url: string | (() => string),
  options: UseApiOptions = {}
) {
  const router = useRouter()
  const authStore = useAuthStore()
  
  const { data, pending, error, refresh, execute, status } = useApi<T>(url, {
    ...options,
    onResponseError({ response }) {
      if (response.status === 401) {
        if (!authStore.isAuthenticated) {
          router.push('/auth/login')
        }
      }
    },
  })
  
  return {
    data,
    pending,
    loading: pending,
    error,
    refresh,
    execute,
    status,
  }
}
