// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  css: ['~/assets/css/main.css'],

  vite: {
    plugins: [
      tailwindcss(),
    ],
  },
  
  modules: ['@pinia/nuxt', 'pinia-plugin-persistedstate/nuxt', '@nuxtjs/i18n'],

  app: {
    head: {
      title: 'WMS - Gestão de Armazém',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
      ],
    },
  },

runtimeConfig: {
        // Server-only (private)
        apiSecret: '',
        // Public (exposed to client)
        public: {
            // @ts-ignore
            apiBase: process.env.NUXT_PUBLIC_API_BASE || 'http://localhost:8080/api/v1',
            // @ts-ignore
            wsUrl: process.env.NUXT_PUBLIC_WS_URL || 'ws://localhost:8080/ws',
            // @ts-ignore
            cdnUrl: process.env.NUXT_PUBLIC_CDN_URL || 'https://cdn.simple-chat.com',
            // @ts-ignore
            stripeKey: process.env.NUXT_PUBLIC_STRIPE_KEY || '',
            // @ts-ignore
            appName: process.env.NUXT_PUBLIC_APP_NAME || 'Simple Chat',
            // @ts-ignore
            apiKey: process.env.NUXT_PUBLIC_API_KEY || 'dev-api-key-change-in-production',
        },
    },
  
  i18n: {
    locales: [
      {
        code: 'en',
        iso: 'en-US',
        file: 'en.json',
        name: 'English'
      },
      {
        code: 'pt-BR',
        iso: 'pt-BR',
        file: 'pt-BR.json',
        name: 'Português Brasil'
      }
    ],
    defaultLocale: 'pt-BR', 
    strategy: 'prefix_except_default',
    lazy: true,
    detectBrowserLanguage: {
      useCookie: true,
      cookieKey: 'i18n_redirected',
      redirectOn: 'root'
    }
  },
    
 piniaPluginPersistedstate: {
        storage: 'cookies',
        cookieOptions: {
            sameSite: 'lax',           // Proteção CSRF + navegação amigável
            // NOTA: httpOnly: true NÃO pode ser usado aqui porque o plugin
            // precisa ler o cookie via JavaScript para hidratar o store.
            // Tokens de autenticação NÃO devem ser persistidos via este plugin!
            // Eles devem ficar em memória (ref) ou em cookies HttpOnly gerenciados
            // pelo backend Rust (proxy /api/proxy preserva esses cookies).
            maxAge: 7 * 24 * 60 * 60,  // 7 dias
        },
    },

  nitro: {
    compressPublicAssets: true,
  },
})
