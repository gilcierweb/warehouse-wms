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
  
  modules: ['@nuxtjs/i18n', '@pinia/nuxt'],

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
     public: {
       apiBase: process.env.API_BASE || 'http://localhost:8080',
       wsBase: process.env.WS_BASE || 'ws://localhost:8080',
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
              sameSite: 'lax',           // ✅ Proteção CSRF + navegação amigável
              // NOTA: httpOnly: true NÃO pode ser usado aqui porque o plugin
              // precisa ler o cookie via JavaScript para hidratar o store.
              // Tokens de autenticação NÃO devem ser persistidos via este plugin!
              // Eles devem ficar em memória (ref) ou em cookies HttpOnly gerenciados
              // pelo backend Rust (proxy /api/proxy preserva esses cookies).
              maxAge: 7 * 24 * 60 * 60,  // 7 dias
          },
  },
  
})