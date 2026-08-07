<template>
  <div class="page">
    <PageHeader :title="`Task #${id}`" subtitle="Opened via deep link or navigation" />
    <section class="section">
      <div class="kv"><span>Task ID</span><code>{{ id }}</code></div>
      <div class="kv" v-if="ref"><span>Ref (query param)</span><code>{{ ref }}</code></div>
      <div class="kv"><span>Route path</span><code>{{ $route.fullPath }}</code></div>
      <p class="note">
        Try opening <code>dailygig://task/{{ id }}?ref=push</code> from adb or a
        note-taking app on the phone.
      </p>
      <button class="btn" @click="$router.back()">← Back</button>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useRoute } from 'vue-router';
import PageHeader from '../components/PageHeader.vue';

const route = useRoute();
const id = computed(() => String(route.params.id ?? ''));
const ref = computed(() => (route.query.ref as string) || '');
</script>

<style scoped>
.section { padding: 12px 20px; }
.kv {
  display: flex; justify-content: space-between; gap: 12px;
  padding: 8px 0; border-bottom: 1px solid var(--border); font-size: 13px;
}
.kv code { color: var(--muted); word-break: break-all; text-align: right; }
.note { font-size: 12px; color: var(--muted); margin-top: 12px; }
.btn {
  margin-top: 16px;
  padding: 12px 16px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-weight: 600;
  cursor: pointer;
}
</style>
