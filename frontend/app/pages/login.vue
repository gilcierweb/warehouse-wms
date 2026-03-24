<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900 px-4">
    <div class="max-w-md w-full space-y-8">
      <div class="text-center">
        <h1 class="text-3xl font-bold text-white mb-2">WMS Login</h1>
        <p class="text-gray-400">Warehouse Management System</p>
      </div>
      
      <form class="mt-8 space-y-6" @submit.prevent="handleLogin">
        <div class="space-y-4">
          <div>
            <label for="username" class="block text-sm font-medium text-gray-300 mb-1">
              Username or Email
            </label>
            <input
              id="username"
              v-model="username"
              type="text"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Enter your username or email"
            />
          </div>
          
          <div>
            <label for="password" class="block text-sm font-medium text-gray-300 mb-1">
              Password
            </label>
            <input
              id="password"
              v-model="password"
              type="password"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Enter your password"
            />
          </div>
        </div>
        
        <div v-if="error" class="bg-red-900/50 border border-red-700 rounded-lg p-3 text-red-300 text-sm text-center">
          <span class="mr-2">⚠️</span>{{ error }}
        </div>
        
        <button
          type="submit"
          :disabled="isLoading"
          class="w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-blue-600 to-blue-700 hover:from-blue-700 hover:to-blue-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
        >
          <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span v-if="isLoading">Signing in...</span>
          <span v-else>Sign in →</span>
        </button>
        
        <div class="mt-6 pt-6 border-t border-gray-700 text-center">
          <span class="text-gray-400 text-sm">Don't have an account?</span>
          <NuxtLink to="/register" class="ml-2 text-sm font-medium text-blue-400 hover:text-blue-300 transition-colors">
            Create account →
          </NuxtLink>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

definePageMeta({
  layout: 'auth'
})

const { login, initAuth } = useAuth()
const { push } = useAlerts()

const username = ref('')
const password = ref('')
const isLoading = ref(false)
const error = ref('')

const handleLogin = async () => {
  error.value = ''
  isLoading.value = true
  
  try {
    await login(username.value, password.value)
    push({ type: 'success', message: 'Login successful!' })
    
    // Check for redirect after login
    const redirect = sessionStorage.getItem('redirectAfterLogin')
    if (redirect) {
      sessionStorage.removeItem('redirectAfterLogin')
      await navigateTo(redirect)
    } else {
      await navigateTo('/')
    }
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || 'Invalid credentials'
    push({ type: 'danger', message: error.value })
  } finally {
    isLoading.value = false
  }
}

// Try to restore auth state on mount
onMounted(() => {
  initAuth()
})
</script>
