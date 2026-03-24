<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-900 px-4">
    <div class="max-w-md w-full space-y-8">
      <div class="text-center">
        <h1 class="text-3xl font-bold text-white mb-2">Create Account</h1>
        <p class="text-gray-400">Warehouse Management System</p>
      </div>
      
      <form class="mt-8 space-y-6" @submit.prevent="handleRegister">
        <div class="space-y-4">
          <div>
            <label for="username" class="block text-sm font-medium text-gray-300 mb-1">
              Username
            </label>
            <input
              id="username"
              v-model="username"
              type="text"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Enter username"
            />
          </div>
          
          <div>
            <label for="email" class="block text-sm font-medium text-gray-300 mb-1">
              Email
            </label>
            <input
              id="email"
              v-model="email"
              type="email"
              required
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Enter email"
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
              minlength="8"
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
              placeholder="Enter password (min 8 characters)"
            />
          </div>
          
          <div>
            <label for="role" class="block text-sm font-medium text-gray-300 mb-1">
              Account Type
            </label>
            <select
              id="role"
              v-model="role"
              class="w-full px-4 py-3 bg-gray-800 border border-gray-700 rounded-lg text-white focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            >
              <option :value="2">Operator (Standard)</option>
              <option :value="1">Administrator</option>
              <option :value="3">Viewer (Read-only)</option>
            </select>
          </div>
        </div>
        
        <div v-if="error" class="bg-red-900/50 border border-red-700 rounded-lg p-3 text-red-300 text-sm text-center">
          <span class="mr-2">⚠️</span>{{ error }}
        </div>
        
        <button
          type="submit"
          :disabled="isLoading"
          class="w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-white bg-gradient-to-r from-green-600 to-green-700 hover:from-green-700 hover:to-green-800 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-green-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all"
        >
          <svg v-if="isLoading" class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" fill="none" viewBox="0 0 24 24">
            <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
            <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
          </svg>
          <span v-if="isLoading">Creating account...</span>
          <span v-else>Create Account →</span>
        </button>
        
        <div class="mt-6 pt-6 border-t border-gray-700 text-center">
          <span class="text-gray-400 text-sm">Already have an account?</span>
          <NuxtLink to="/login" class="ml-2 text-sm font-medium text-blue-400 hover:text-blue-300 transition-colors">
            Sign in →
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

const { register } = useAuth()
const { push } = useAlerts()

const username = ref('')
const email = ref('')
const password = ref('')
const role = ref(2)
const isLoading = ref(false)
const error = ref('')

const handleRegister = async () => {
  error.value = ''
  isLoading.value = true
  
  try {
    await register(username.value, email.value, password.value, role.value)
    push({ type: 'success', message: 'Account created! Please login.' })
    await navigateTo('/login')
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || 'Registration failed'
    push({ type: 'danger', message: error.value })
  } finally {
    isLoading.value = false
  }
}
</script>
