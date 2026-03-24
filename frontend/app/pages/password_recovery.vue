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
          <h3 class="auth-heading">Reset Password</h3>
          <p class="auth-desc">Enter your email to receive a reset link</p>
        </div>

        <!-- Form -->
        <form v-if="!emailSent" class="auth-form" @submit.prevent="handleSubmit">
          <!-- Email -->
          <div class="auth-field">
            <label class="auth-label" for="recoveryEmail">Email*</label>
            <input 
              id="recoveryEmail" 
              v-model="email" 
              type="email" 
              placeholder="Enter your email" 
              class="auth-input" 
              required 
            />
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
            <span v-else>Send Reset Link</span>
          </button>
        </form>

        <!-- Success Message -->
        <div v-else-if="emailSent" class="auth-success">
          <div class="auth-success-icon">
            <svg fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
          <h4 class="auth-success-title">Check your email</h4>
          <p class="auth-success-desc">We've sent a password reset link to <strong>{{ email }}</strong></p>
          <div class="auth-token-box">
            <p class="auth-token-label">Reset Token (for testing):</p>
            <code class="auth-token">{{ recoveryToken }}</code>
          </div>
          <NuxtLink :to="`/password_reset?token=${recoveryToken}`" class="auth-submit-btn auth-submit-btn-link">
            Continue to Reset
          </NuxtLink>
        </div>

        <!-- Login Link -->
        <p class="auth-footer-text">
          Remember your password?
          <NuxtLink to="/login" class="auth-link">
            Sign in
          </NuxtLink>
        </p>

        <!-- Register Link -->
        <p class="auth-footer-text">
          New on our platform?
          <NuxtLink to="/register" class="auth-link">
            Create an account
          </NuxtLink>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

definePageMeta({
  layout: 'auth'
})

const { recoverPassword } = useAuth()
const { push } = useAlerts()

const email = ref('')
const isLoading = ref(false)
const error = ref('')
const emailSent = ref(false)
const recoveryToken = ref('')

const handleSubmit = async () => {
  error.value = ''
  isLoading.value = true
  
  try {
    const response = await recoverPassword(email.value)
    emailSent.value = true
    // Token returned from backend API (in production this would come from email)
    recoveryToken.value = response.token || ''
    push({ type: 'success', message: 'Reset link sent to your email!' })
  } catch (err: any) {
    error.value = err?.message || 'Failed to send reset link'
    push({ type: 'danger', message: error.value })
  } finally {
    isLoading.value = false
  }
}
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

.auth-success {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  padding: 20px 0;
}

.auth-success-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: rgba(34, 197, 94, 0.1);
  color: var(--green);
  margin-bottom: 16px;
}

.auth-success-icon svg {
  width: 32px;
  height: 32px;
}

.auth-success-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text);
  margin-bottom: 8px;
}

.auth-success-desc {
  font-size: 14px;
  color: var(--text-2);
}

.auth-success-desc strong {
  color: var(--text);
}

.auth-footer-text {
  text-align: center;
  font-size: 14px;
  color: var(--text-2);
  margin-top: 16px;
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

.auth-submit-btn-link {
  text-decoration: none;
  margin-top: 16px;
}

.auth-token-box {
  margin-top: 16px;
  padding: 12px;
  background: var(--bg-2);
  border: 1px solid var(--border);
  border-radius: 6px;
  width: 100%;
}

.auth-token-label {
  font-size: 12px;
  color: var(--text-3);
  margin-bottom: 6px;
}

.auth-token {
  font-family: var(--mono);
  font-size: 12px;
  color: var(--green);
  word-break: break-all;
}

.auth-submit-btn-link {
  text-decoration: none;
  margin-top: 16px;
}
</style>
