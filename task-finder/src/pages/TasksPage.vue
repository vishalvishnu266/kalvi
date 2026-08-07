<template>
  <div class="page">
    <PageHeader title="Nearby Tasks" subtitle="Pick up a gig around you" />

    <div class="filter-row">
      <button
        v-for="c in categories"
        :key="c"
        class="chip"
        :class="{ active: activeCategory === c }"
        @click="activeCategory = c"
      >{{ c }}</button>
    </div>

    <div class="list">
      <article v-for="task in filteredTasks" :key="task.id" class="task-card">
        <div class="task-top">
          <span class="task-cat">{{ task.category }}</span>
          <span class="task-price">${{ task.price }}</span>
        </div>
        <div class="task-title">{{ task.title }}</div>
        <div class="task-meta">
          <span>📍 {{ task.distance }} km</span>
          <span>⏱ {{ task.duration }}</span>
        </div>
        <button class="apply-btn">Apply</button>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import PageHeader from '../components/PageHeader.vue';

const categories = ['All', 'Delivery', 'Cleaning', 'Handyman', 'Pets'];
const activeCategory = ref('All');

const tasks = [
  { id: 1, category: 'Delivery', title: 'Deliver package to downtown', distance: 1.2, duration: '30 min', price: 15 },
  { id: 2, category: 'Cleaning', title: '2-hour apartment clean', distance: 0.6, duration: '2 hr', price: 55 },
  { id: 3, category: 'Handyman', title: 'Assemble IKEA shelf', distance: 3.4, duration: '1 hr', price: 40 },
  { id: 4, category: 'Pets', title: 'Walk two dogs', distance: 0.9, duration: '45 min', price: 22 },
  { id: 5, category: 'Delivery', title: 'Grocery run', distance: 2.1, duration: '1 hr', price: 20 },
];

const filteredTasks = computed(() =>
  activeCategory.value === 'All' ? tasks : tasks.filter(t => t.category === activeCategory.value)
);
</script>

<style scoped>
.filter-row {
  display: flex;
  gap: 8px;
  padding: 8px 20px 4px;
  overflow-x: auto;
  scrollbar-width: none;
}
.filter-row::-webkit-scrollbar { display: none; }
.chip {
  border: 1px solid var(--border, #e5e7eb);
  background: var(--surface, #fff);
  padding: 6px 14px;
  border-radius: 999px;
  font-size: 13px;
  color: var(--muted, #6b7280);
  white-space: nowrap;
  cursor: pointer;
}
.chip.active {
  background: var(--primary, #2563eb);
  color: #fff;
  border-color: var(--primary, #2563eb);
}
.list { padding: 12px 20px 20px; display: flex; flex-direction: column; gap: 12px; }
.task-card {
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
  padding: 14px;
}
.task-top { display: flex; justify-content: space-between; align-items: center; }
.task-cat {
  font-size: 11px;
  font-weight: 600;
  color: var(--primary, #2563eb);
  background: #eff6ff;
  padding: 3px 8px;
  border-radius: 6px;
}
.task-price { font-weight: 700; color: var(--text, #111827); }
.task-title { font-size: 15px; font-weight: 600; margin: 8px 0 6px; color: var(--text, #111827); }
.task-meta { display: flex; gap: 12px; font-size: 12px; color: var(--muted, #6b7280); }
.apply-btn {
  margin-top: 10px;
  width: 100%;
  padding: 10px;
  background: var(--primary, #2563eb);
  color: #fff;
  border: none;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
}
.apply-btn:active { opacity: 0.85; }
</style>
