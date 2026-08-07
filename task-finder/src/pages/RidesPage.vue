<template>
  <div class="page">
    <PageHeader title="Rides Near You" subtitle="Find a cab in minutes" />

    <section class="search-card">
      <label>
        <span class="dot pickup" />
        <input type="text" placeholder="Pickup location" v-model="pickup" />
      </label>
      <div class="divider" />
      <label>
        <span class="dot drop" />
        <input type="text" placeholder="Where to?" v-model="drop" />
      </label>
      <button class="cta">Find Rides</button>
    </section>

    <h3 class="section-title">Available now</h3>
    <div class="ride-list">
      <article v-for="r in rides" :key="r.id" class="ride-card">
        <div class="ride-icon">🚗</div>
        <div class="ride-body">
          <div class="ride-name">{{ r.type }}</div>
          <div class="ride-eta">{{ r.eta }} away · {{ r.seats }} seats</div>
        </div>
        <div class="ride-price">${{ r.price }}</div>
      </article>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import PageHeader from '../components/PageHeader.vue';

const pickup = ref('Current location');
const drop = ref('');
const rides = [
  { id: 1, type: 'Economy', eta: '3 min', seats: 4, price: 8 },
  { id: 2, type: 'Comfort', eta: '5 min', seats: 4, price: 12 },
  { id: 3, type: 'XL (6 seats)', eta: '7 min', seats: 6, price: 18 },
  { id: 4, type: 'Bike', eta: '2 min', seats: 1, price: 4 },
];
</script>

<style scoped>
.search-card {
  margin: 12px 20px 20px;
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
  padding: 12px 14px;
}
.search-card label { display: flex; align-items: center; gap: 10px; padding: 8px 0; }
.search-card input {
  flex: 1;
  border: none;
  outline: none;
  font-size: 15px;
  background: transparent;
  color: var(--text, #111827);
}
.dot { width: 10px; height: 10px; border-radius: 50%; display: inline-block; }
.pickup { background: #22c55e; }
.drop { background: #ef4444; }
.divider { height: 1px; background: var(--border, #e5e7eb); margin-left: 20px; }
.cta {
  margin-top: 10px;
  width: 100%;
  padding: 12px;
  background: var(--primary, #2563eb);
  color: #fff;
  border: none;
  border-radius: 10px;
  font-weight: 600;
  font-size: 15px;
}
.section-title { margin: 4px 20px 10px; font-size: 15px; color: var(--text, #111827); }
.ride-list { padding: 0 20px 20px; display: flex; flex-direction: column; gap: 10px; }
.ride-card {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 14px;
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
}
.ride-icon { font-size: 24px; }
.ride-body { flex: 1; }
.ride-name { font-weight: 600; color: var(--text, #111827); }
.ride-eta { font-size: 12px; color: var(--muted, #6b7280); margin-top: 2px; }
.ride-price { font-weight: 700; color: var(--primary, #2563eb); }
</style>
