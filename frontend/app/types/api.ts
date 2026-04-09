// Types for $apiFetch plugin

export interface ApiFetchOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
  body?: any
  headers?: Record<string, string>
  auth?: boolean
}

export type ApiFetchFunction = <T>(url: string, options?: ApiFetchOptions) => Promise<T>

declare module '#app' {
  interface NuxtApp {
    $apiFetch: ApiFetchFunction
    $api: ApiFetchFunction
  }
}

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $apiFetch: ApiFetchFunction
    $api: ApiFetchFunction
  }
}

export {}
