// app/composables/useApi.ts
// SSR-friendly data fetching com useFetch - evita double fetch

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

/**
 * useApi - Composable SSR-friendly para data fetching
 * 
 * Usa useFetch por baixo para:
 * - Evitar double fetch (SSR + client)
 * - Payload extraction automática
 * - Reatividade nativa (data, pending, error, refresh)
 * 
 * @param url - Path da API (ex: '/auctions', '/auctions/123')
 * @param options - Opções de fetch
 * @returns { data, pending, error, refresh, execute }
 * 
 * @example
 * // Em uma página - executa no SSR automaticamente
 * const { data: auctions, pending, error, refresh } = useApi<Auction[]>('/auctions')
 * 
 * @example
 * // Com opções
 * const { data: auction } = useApi<Auction>('/auctions/123', {
 *   server: true,
 *   default: () => null,
 * })
 */
export function useApi<T>(
  url: string | (() => string),
  options: UseApiOptions = {}
) {
  const config = useRuntimeConfig()
  
  // Usar o proxy interno para preservar cookies HttpOnly
  const baseURL = '/api/proxy'
  
  // Headers com cookies do request original (importante para SSR)
  const headers = {
    ...(import.meta.server ? useRequestHeaders(['cookie', 'authorization']) : {}),
    ...options.headers,
  }
  
  // Construir URL completa
  const fullUrl = computed<string | null>(() => {
    const path = typeof url === 'function' ? url() : url
    // Guarda contra path invalido (null, undefined, vazio, placeholder, invalid, ou empty)
    if (!path || path === 'null' || path === 'undefined' || path === '' || path.includes('/invalid') || path.includes('/placeholder') || path.includes('/empty')) {
      return null // Retorna null para impedir requisição inválida
    }
    const queryString = options.query 
      ? '?' + new URLSearchParams(options.query).toString()
      : ''
    return `${baseURL}${path}${queryString}`
  })
  
  // useFetch com configuração SSR-friendly
  const { data, pending, error, refresh, execute, status } = useFetch<T>(fullUrl, {
    method: (options.method || 'get') as any,
    headers,
    body: options.body,
    server: options.server !== false, // Default: executa no SSR
    immediate: options.immediate !== false, // Default: executa imediatamente
    watch: options.watch,
    default: options.default,
    // Transformar erro para formato consistente
    onResponseError({ response }) {
      console.error('[useApi] Error:', response.status, response._data)
    },
  })
  
  return {
    data,
    pending,
    loading: pending, // Alias para consistência
    error,
    refresh,
    execute,
    status,
  }
}

/**
 * useApiLazy - Versão lazy (não executa automaticamente)
 * Útil para ações de usuário (botões, submits)
 */
export function useApiLazy<T>(
  url: string | (() => string),
  options: UseApiOptions = {}
) {
  return useApi<T>(url, {
    ...options,
    immediate: false,
  })
}

/**
 * useAuthApi - useApi com autenticação obrigatória
 * Redireciona para login se 401
 */
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
        // Nao faz logout automatico - deixa o middleware/auth.global.ts tratar
        // Apenas redireciona para login se necessario
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
