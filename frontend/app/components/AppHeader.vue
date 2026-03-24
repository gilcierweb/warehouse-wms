<template>
  <header class="app-header">
    <div class="header-brand">
      <h1 class="brand-title">WMS</h1>
      <span class="brand-subtitle">Gestão de Armazém</span>
    </div>

    <nav class="header-nav">
      <NuxtLink 
        v-for="item in navItems" 
        :key="item.path"
        :to="item.path" 
        class="nav-link"
        :class="{ 'nav-link--active': isActive(item.path) }"
      >
        <span class="nav-icon">{{ item.icon }}</span>
        <span class="nav-label">{{ item.label }}</span>
      </NuxtLink>
    </nav>

    <div class="header-actions">
      <button class="action-btn" title="Atualizar dados" @click="refreshData">
        <span>↻</span>
      </button>
      <button class="action-btn" title="Configurações" @click="$router.push('/settings')">
        <span>⚙</span>
      </button>
      <button v-if="isAuthenticated" class="action-btn logout-btn" title="Logout" @click="handleLogout">
        <span>⏻</span>
      </button>
    </div>
  </header>
</template>

<script setup lang="ts">
const route = useRoute()
const router = useRouter()
const store = useWarehouseStore()
const { push } = useAlerts()
const { isAuthenticated, logout } = useAuth()

const navItems = [
  { path: '/', label: 'Mapa', icon: '◪' },
  { path: '/dashboard', label: 'Dashboard', icon: '▤' },
  { path: '/history', label: 'Histórico', icon: '◖' },
  { path: '/settings', label: 'Configurações', icon: '⚙' }
]

function isActive(path: string): boolean {
  if (path === '/' && route.path === '/') return true
  if (path !== '/' && route.path.startsWith(path)) return true
  return false
}

async function refreshData() {
  try {
    const api = useWarehouseApi()
    const slots = await api.fetchSlots()
    store.bulkLoad(slots)
    push({ type: 'success', message: 'Dados atualizados com sucesso' })
  } catch {
    push({ type: 'danger', message: 'Falha ao atualizar dados' })
  }
}

function handleLogout() {
  logout()
}
</script>

<style scoped>
.app-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 12px 20px;
  background: var(--bg-1);
  border-bottom: 1px solid var(--border);
  min-height: 60px;
}

.header-brand {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.brand-title {
  font-size: 16px;
  font-weight: 700;
  letter-spacing: .12em;
  color: var(--text);
  margin: 0;
}

.brand-subtitle {
  font-size: 10px;
  font-weight: 500;
  letter-spacing: .08em;
  color: var(--text-3);
  text-transform: uppercase;
}

.header-nav {
  display: flex;
  gap: 4px;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border-radius: var(--radius);
  text-decoration: none;
  color: var(--text-2);
  font-size: 11px;
  font-weight: 500;
  letter-spacing: .04em;
  transition: all .15s;
  border: 1px solid transparent;
}

.nav-link:hover {
  color: var(--text);
  background: var(--bg-2);
  border-color: var(--border);
}

.nav-link--active {
  color: var(--green);
  background: var(--green-bg);
  border-color: var(--green-dim);
}

.nav-icon {
  font-size: 12px;
}

.nav-label {
  text-transform: uppercase;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border-radius: var(--radius);
  background: var(--bg-2);
  border: 1px solid var(--border);
  color: var(--text-2);
  font-size: 12px;
  cursor: pointer;
  transition: all .12s;
}

.action-btn:hover {
  color: var(--text);
  border-color: var(--border-2);
  background: var(--bg-3);
}

.action-btn:active {
  transform: scale(.95);
}

.logout-btn {
  background: var(--red-bg);
  border-color: var(--red-dim);
  color: var(--red);
}

.logout-btn:hover {
  background: var(--red);
  color: white;
}
</style>