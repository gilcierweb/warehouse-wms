<template>
  <div class="auth-page">
    <div class="auth-container">
      <!-- Background Decoration -->
      <div class="auth-decoration">
        <svg width="612" height="697" viewBox="0 0 612 697" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            d="M360.405 111.996C393.955 67.9448 456.863 59.4318 500.914 92.9818V92.9818C544.965 126.532 553.478 189.44 519.928 233.491L250.545 587.191C216.995 631.243 154.087 639.756 110.036 606.206V606.206C65.9845 572.656 57.4716 509.747 91.0216 465.696L360.405 111.996Z"
            fill="url(#paint0_linear_auth)"
            fill-opacity="0.08"
          />
          <path
            d="M519.53 233.188L250.147 586.888C216.765 630.72 154.17 639.19 110.339 605.808C66.5071 572.425 58.0367 509.831 91.4194 465.999L360.802 112.299C394.185 68.4674 456.78 59.9969 500.611 93.3796C544.443 126.762 552.913 189.357 519.53 233.188Z"
            stroke="var(--green)"
            stroke-opacity="0.2"
          />
          <defs>
            <linearGradient
              id="paint0_linear_auth"
              x1="500.914"
              y1="92.9818"
              x2="110.036"
              y2="606.206"
              gradientUnits="userSpaceOnUse"
            >
              <stop offset="0" stop-color="var(--green)" />
              <stop offset="1" stop-color="var(--green)" stop-opacity="0.2" />
            </linearGradient>
          </defs>
        </svg>
      </div>

      <!-- Auth Card -->
      <div class="auth-card">
        <!-- Logo & Header -->
        <div class="auth-header">
          <div class="auth-logo">
            <span class="auth-logo-icon">◪</span>
          </div>
          <h2 class="auth-title">WMS</h2>
        </div>
        
        <div class="auth-subheader">
          <h3 class="auth-heading">{{ $t('auth.signIn') }}</h3>
          <p class="auth-desc">Warehouse Management System</p>
        </div>

        <!-- Form -->
        <form class="auth-form" @submit.prevent="handleLogin">
          <!-- Email -->
          <div class="auth-field">
            <label class="auth-label" for="userEmail">{{ $t('auth.email') }}*</label>
            <input 
              id="userEmail" 
              v-model="email" 
              type="email" 
              :placeholder="$t('auth.email')" 
              class="auth-input" 
              required 
            />
          </div>

          <!-- Password -->
          <div class="auth-field">
            <label class="auth-label" for="userPassword">{{ $t('auth.password') }}*</label>
            <div class="auth-input-group">
              <input 
                id="userPassword" 
                v-model="password" 
                :type="showPassword ? 'text' : 'password'" 
                placeholder="············" 
                required 
                class="auth-input-password"
              />
              <button
                type="button"
                class="auth-toggle-password"
                aria-label="Toggle password visibility"
                @click="showPassword = !showPassword"
              >
                <svg v-if="showPassword" class="auth-eye-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 5.411m0 0L21 21" />
                </svg>
                <svg v-else class="auth-eye-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
                </svg>
              </button>
            </div>
          </div>

          <!-- Remember & Forgot -->
          <div class="auth-options">
            <label class="auth-checkbox-label">
              <input type="checkbox" class="auth-checkbox" id="rememberMe" v-model="rememberMe" />
              <span class="auth-checkbox-text">{{ $t('auth.rememberMe') }}</span>
            </label>
            <NuxtLink to="/password_recovery" class="auth-link">{{ $t('auth.forgotPassword') }}</NuxtLink>
          </div>

          <!-- Error Alert -->
          <div v-if="error" class="auth-error">
            <svg class="auth-error-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{{ error }}</span>
          </div>

          <!-- Submit Button -->
          <button 
            type="submit" 
            class="auth-submit-btn"
            :disabled="isLoading"
          >
            <span v-if="isLoading" class="auth-spinner"></span>
            <span v-else>{{ $t('auth.signInToWms') }}</span>
          </button>
        </form>

        <!-- Register Link -->
        <p class="auth-footer-text">
          {{ $t('auth.newOnPlatform') }}
          <NuxtLink to="/auth/register" class="auth-link">
            {{ $t('auth.createAccount') }}
          </NuxtLink>
        </p>

        <!-- Features -->
        <div class="auth-features">
          <div class="auth-feature">
            <svg class="auth-feature-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
            </svg>
            <span>{{ $t('auth.easySetup') }}</span>
          </div>
          <div class="auth-feature">
            <svg class="auth-feature-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z" />
            </svg>
            <span>{{ $t('auth.secure') }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

definePageMeta({
  layout: 'auth'
})

const { login, initAuth, isAuthenticated, user } = useAuth()
const { push } = useAlerts()
const { t } = useI18n()
const { connect: connectWS, isConnected } = useWarehouseWS()

const email = ref('')
const password = ref('')
const isLoading = ref(false)
const error = ref('')
const showPassword = ref(false)
const rememberMe = ref(false)

const handleLogin = async () => {
  error.value = ''
  isLoading.value = true

  try {
    await login(email.value, password.value)
    
    push({ type: 'success', message: 'Welcome back!' })

    console.log('[Login] Login success, isConnected:', isConnected())
    if (!isConnected()) {
      console.log('[Login] Calling connectWS()')
      connectWS()
      console.log('[Login] After connectWS, isConnected:', isConnected())
    }

    const redirect = sessionStorage.getItem('redirectAfterLogin')
    
    if (redirect) {
      sessionStorage.removeItem('redirectAfterLogin')
      await navigateTo(redirect)
    } else {
      await navigateTo('/')
    }
  } catch (err: any) {
    error.value = err?.data?.message || err?.message || t('auth.invalidCredentials')
    push({ type: 'danger', message: error.value })
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  initAuth()
  
  // Watch for auth state changes
  watch([isAuthenticated, user], ([newAuth, newUser]) => {
  }, { immediate: true })
})
</script>

<style scoped>
.auth-page {
  display: flex;
  min-height: 100vh;
  align-items: center;
  justify-content: center;
  background: var(--bg);
  background-image: url('https://cdn.flyonui.com/fy-assets/blocks/marketing-ui/auth/auth-background-2.png');
  background-size: cover;
  background-position: center;
  background-repeat: no-repeat;
  padding: 40px 16px;
}

.auth-container {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  max-width: 440px;
}

.auth-decoration {
  position: absolute;
  z-index: 0;
  pointer-events: none;
}

.auth-decoration svg {
  opacity: 0.6;
}

.auth-card {
  position: relative;
  z-index: 1;
  width: 100%;
  background: var(--bg-1);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 32px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
}

.auth-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 24px;
}

.auth-logo {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  border-radius: 10px;
  background: rgba(34, 197, 94, 0.1);
}

.auth-logo-icon {
  font-size: 24px;
  color: var(--green);
}

.auth-title {
  font-size: 20px;
  font-weight: 700;
  color: var(--text);
  letter-spacing: 0.05em;
}

.auth-subheader {
  margin-bottom: 20px;
}

.auth-heading {
  font-size: 24px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 4px;
}

.auth-desc {
  font-size: 14px;
  color: var(--text-2);
}

.auth-quick-login {
  font-size: 14px;
  color: var(--text-2);
  margin-bottom: 24px;
}

.auth-link {
  color: var(--green);
  font-weight: 400;
  text-decoration: none;
  transition: color 0.15s;
}

.auth-link:hover {
  color: var(--text);
}

.auth-form {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.auth-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.auth-label {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-2);
}

.auth-input {
  width: 100%;
  height: 42px;
  padding: 0 14px;
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  color: var(--text);
  font-size: 14px;
  outline: none;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.auth-input:focus {
  border-color: var(--green);
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.15);
}

.auth-input::placeholder {
  color: var(--text-3);
}

.auth-input-group {
  display: flex;
  align-items: center;
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  overflow: hidden;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.auth-input-group:focus-within {
  border-color: var(--green);
  box-shadow: 0 0 0 3px rgba(34, 197, 94, 0.15);
}

.auth-input-password {
  flex: 1;
  height: 42px;
  padding: 0 14px;
  background: transparent;
  border: none;
  color: var(--text);
  font-size: 14px;
  outline: none;
}

.auth-input-password::placeholder {
  color: var(--text-3);
}

.auth-toggle-password {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 42px;
  height: 42px;
  background: transparent;
  border: none;
  color: var(--text-3);
  cursor: pointer;
  transition: color 0.15s;
}

.auth-toggle-password:hover {
  color: var(--text);
}

.auth-eye-icon {
  width: 20px;
  height: 20px;
}

.auth-options {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.auth-checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.auth-checkbox {
  width: 16px;
  height: 16px;
  accent-color: var(--green);
  cursor: pointer;
}

.auth-checkbox-text {
  font-size: 14px;
  color: var(--text-2);
}

.auth-error {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px 14px;
  background: var(--red-bg);
  border: 1px solid var(--red-dim);
  border-radius: 6px;
  color: var(--red);
  font-size: 14px;
}

.auth-error-icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
}

.auth-submit-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  width: 100%;
  height: 48px;
  background: var(--green);
  border: none;
  border-radius: 6px;
  color: #000;
  font-size: 15px;
  font-weight: 600;
  letter-spacing: 0.04em;
  cursor: pointer;
  transition: background 0.15s, transform 0.1s;
}

.auth-submit-btn:hover:not(:disabled) {
  background: #16a34a;
}

.auth-submit-btn:active:not(:disabled) {
  transform: scale(0.98);
}

.auth-submit-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.auth-spinner {
  width: 18px;
  height: 18px;
  border: 2px solid transparent;
  border-top-color: #000;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.auth-footer-text {
  text-align: center;
  font-size: 14px;
  color: var(--text-2);
  margin-top: 20px;
}

.auth-divider {
  display: flex;
  align-items: center;
  gap: 16px;
  margin: 24px 0;
  color: var(--text-3);
  font-size: 13px;
}

.auth-divider::before,
.auth-divider::after {
  content: '';
  flex: 1;
  height: 1px;
  background: var(--border);
}

.auth-features {
  display: flex;
  justify-content: center;
  gap: 24px;
}

.auth-feature {
  display: flex;
  align-items: center;
  gap: 6px;
  color: var(--text-3);
  font-size: 13px;
}

.auth-feature-icon {
  width: 16px;
  height: 16px;
}
</style>
