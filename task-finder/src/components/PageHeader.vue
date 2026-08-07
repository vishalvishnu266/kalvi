<template>
  <header class="page-header">
    <h1>{{ title }}</h1>
    <p v-if="subtitle" class="subtitle">{{ subtitle }}</p>
  </header>
</template>

<script setup lang="ts">
defineProps<{ title: string; subtitle?: string }>();
</script>

<style scoped>
.page-header {
  padding: 20px 20px 12px;
  padding-top: calc(20px + env(safe-area-inset-top));
  background: var(--surface, #fff);
}
h1 {
  margin: 0;
  font-size: 24px;
  font-weight: 700;
  color: var(--text, #111827);
}
.subtitle {
  margin: 4px 0 0;
  font-size: 14px;
  color: var(--muted, #6b7280);
}
</style>
