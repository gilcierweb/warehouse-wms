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
  
  modules: ['@nuxtjs/i18n'],

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
})