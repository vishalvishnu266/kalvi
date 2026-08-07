<template>
  <div class="app-shell">
    <main class="app-main">
      <router-view v-slot="{ Component }">
        <transition name="fade" mode="out-in">
          <component :is="Component" />
        </transition>
      </router-view>
    </main>
    <TabBar />
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import TabBar from './components/TabBar.vue';
import { initNative } from './composables/useNative';

onMounted(() => {
  initNative();
});
</script>

<style scoped>
.app-shell {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  background: var(--bg, #f5f6f8);
}
.app-main {
  flex: 1;
  overflow-y: auto;
  /* Leave room for the fixed bottom tab bar + safe area */
  padding-bottom: calc(72px + env(safe-area-inset-bottom));
}
.fade-enter-active, .fade-leave-active { transition: opacity 0.15s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
