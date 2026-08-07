<template>
  <nav class="tab-bar" role="tablist" aria-label="Main navigation">
    <button
      v-for="tab in tabs"
      :key="tab.name"
      class="tab-btn"
      :class="{ active: current === tab.name }"
      role="tab"
      :aria-selected="current === tab.name"
      :aria-label="tab.label"
      @click="go(tab)"
    >
      <span class="tab-icon" v-html="tab.icon" />
      <span class="tab-label">{{ tab.label }}</span>
    </button>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { hapticTap } from '../composables/useNative';

type Tab = { name: string; label: string; path: string; icon: string };

const tabs: Tab[] = [
  {
    name: 'sandbox', label: 'Sandbox', path: '/sandbox',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M3 9h18"/><circle cx="7" cy="6.5" r="0.6" fill="currentColor"/><circle cx="9.5" cy="6.5" r="0.6" fill="currentColor"/></svg>',
  },
  {
    name: 'location', label: 'Location', path: '/location',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s-7-7.58-7-12a7 7 0 1 1 14 0c0 4.42-7 12-7 12z"/><circle cx="12" cy="10" r="2.5"/></svg>',
  },
  {
    name: 'device', label: 'Device', path: '/device',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="2" width="12" height="20" rx="2"/><path d="M11 19h2"/></svg>',
  },
  {
    name: 'settings', label: 'Settings', path: '/settings',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/></svg>',
  },
];

const route = useRoute();
const router = useRouter();
const current = computed(() => (route.meta?.tab as string) || 'home');

function go(tab: Tab) {
  hapticTap();
  if (route.path !== tab.path) router.push(tab.path);
}
</script>

<style scoped>
.tab-bar {
  position: fixed;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  justify-content: space-around;
  align-items: stretch;
  background: var(--surface, #ffffff);
  border-top: 1px solid var(--border, #e5e7eb);
  padding-bottom: env(safe-area-inset-bottom);
  z-index: 100;
  box-shadow: 0 -2px 12px rgba(0, 0, 0, 0.04);
}
.tab-btn {
  flex: 1;
  background: transparent;
  border: none;
  color: var(--muted, #6b7280);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  padding: 8px 4px 10px;
  min-height: 56px;
  cursor: pointer;
  transition: color 0.15s ease;
  -webkit-tap-highlight-color: transparent;
}
.tab-btn.active {
  color: var(--primary, #2563eb);
}
.tab-icon {
  display: inline-flex;
  width: 24px;
  height: 24px;
}
.tab-icon :deep(svg) {
  width: 100%;
  height: 100%;
}
.tab-label {
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.02em;
}
</style>
