<template>
  <div class="page">
    <PageHeader title="Good day 👋" subtitle="What do you need today?" />

    <section class="hero">
      <h2>Find help around you</h2>
      <p>Post a quick task, book a nearby ride, or earn on the side.</p>
    </section>

    <section class="quick-actions">
      <button class="qa" @click="$router.push('/tasks')">
        <div class="qa-icon">🧰</div>
        <div class="qa-label">Post a Task</div>
      </button>
      <button class="qa" @click="$router.push('/rides')">
        <div class="qa-icon">🚕</div>
        <div class="qa-label">Book a Ride</div>
      </button>
      <button class="qa">
        <div class="qa-icon">💼</div>
        <div class="qa-label">Find Work</div>
      </button>
      <button class="qa">
        <div class="qa-icon">📦</div>
        <div class="qa-label">Delivery</div>
      </button>
    </section>

    <section class="section">
      <h3>Popular near you</h3>
      <div class="card-list">
        <article v-for="item in popular" :key="item.id" class="card">
          <div class="card-emoji">{{ item.emoji }}</div>
          <div class="card-body">
            <div class="card-title">{{ item.title }}</div>
            <div class="card-meta">{{ item.meta }}</div>
          </div>
          <div class="card-price">{{ item.price }}</div>
        </article>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import PageHeader from '../components/PageHeader.vue';

const popular = [
  { id: 1, emoji: '🛒', title: 'Grocery pickup', meta: '0.8 km away', price: '$12' },
  { id: 2, emoji: '🐕', title: 'Dog walking (30 min)', meta: '1.2 km away', price: '$18' },
  { id: 3, emoji: '🔧', title: 'Fix leaky faucet', meta: '2.1 km away', price: '$45' },
  { id: 4, emoji: '📦', title: 'Move small furniture', meta: '3.4 km away', price: '$60' },
];
</script>

<style scoped>
.hero {
  margin: 12px 20px;
  padding: 20px;
  border-radius: 16px;
  background: linear-gradient(135deg, #2563eb, #7c3aed);
  color: #fff;
}
.hero h2 { margin: 0 0 6px; font-size: 20px; }
.hero p { margin: 0; opacity: 0.9; font-size: 14px; }

.quick-actions {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 10px;
  padding: 0 20px;
  margin-top: 12px;
}
.qa {
  border: 1px solid var(--border, #e5e7eb);
  background: var(--surface, #fff);
  border-radius: 14px;
  padding: 12px 6px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
}
.qa:active { transform: scale(0.97); }
.qa-icon { font-size: 22px; }
.qa-label { font-size: 11px; font-weight: 600; color: var(--text, #111827); text-align: center; }

.section { padding: 20px; }
.section h3 { margin: 0 0 12px; font-size: 16px; color: var(--text, #111827); }
.card-list { display: flex; flex-direction: column; gap: 10px; }
.card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px;
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
}
.card-emoji { font-size: 24px; }
.card-body { flex: 1; }
.card-title { font-weight: 600; font-size: 14px; color: var(--text, #111827); }
.card-meta { font-size: 12px; color: var(--muted, #6b7280); margin-top: 2px; }
.card-price { font-weight: 700; color: var(--primary, #2563eb); }
</style>
