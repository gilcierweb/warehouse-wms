// server/api/proxy/[...path].ts
// Proxy seguro para backend Rust - preserva cookies HttpOnly e injeta API_KEY

export default defineEventHandler(async (event) => {
  const config = useRuntimeConfig()
  const path = getRouterParam(event, 'path')
  const method = getMethod(event)
  
  // 1. Extrair cookies do request original (inclui HttpOnly cookies)
  const cookies = getRequestHeaders(event).cookie || ''
  
  // 2. Ler corpo da requisição (se houver)
  let body = undefined
  if (method !== 'GET' && method !== 'HEAD') {
    try {
      body = await readBody(event)
    } catch {
      // No body or parse error
    }
  }
  
  // 3. Query params
  const query = getQuery(event)
  const queryString = Object.keys(query).length > 0 
    ? '?' + new URLSearchParams(query as Record<string, string>).toString()
    : ''
  
  // 4. Headers para backend (preserva auth do cliente + injeta API_KEY)
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
    'X-API-Key': config.apiSecret || config.public.apiKey, // Server-only API_KEY
  }
  
  // Forward authorization header if present
  const authHeader = getRequestHeader(event, 'authorization')
  if (authHeader) {
    headers['Authorization'] = authHeader
  }
  
  // Forward cookies (crucial for HttpOnly refresh_token)
  if (cookies) {
    headers['Cookie'] = cookies
  }
  
  try {
    // 5. Fazer request para backend Rust
    const response = await $fetch.raw(`${config.public.apiBase}/${path}${queryString}`, {
      method: method as any,
      headers,
      body,
      credentials: 'include',
    })
    
    // 6. Forward cookies do backend para o cliente
    let cookiesArray: string[] = []
    if (typeof response.headers.getSetCookie === 'function') {
      cookiesArray = response.headers.getSetCookie()
    } else {
      const fallback = response.headers.get('set-cookie')
      if (fallback) cookiesArray = [fallback]
    }
      
    for (let cookie of cookiesArray) {
      if (!cookie) continue
      // Garante que o cookie funcione localmente em todo o host de proxy no frontend
      cookie = cookie.replace(/Path=[^;]+/gi, 'Path=/')
      // Remove Domain para o browser basear no host da URL de proxy atual (localhost)
      cookie = cookie.replace(/;\s*Domain=[^;]+/gi, '')
      appendResponseHeader(event, 'set-cookie', cookie)
    }
    
    // 7. Retornar resposta
    return response._data
    
  } catch (error: any) {
    // Forward error status e message
    throw createError({
      statusCode: error.statusCode || 500,
      statusMessage: error.statusMessage || 'Proxy error',
      data: error.data,
    })
  }
})
