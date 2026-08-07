<template>
  <div class="page">
    <PageHeader title="Location" subtitle="Test GPS + permissions" />

    <section class="section">
      <div class="kv"><span>Permission</span><code>{{ permission }}</code></div>
      <div class="kv"><span>Watching</span><code>{{ watching ? 'yes' : 'no' }}</code></div>
      <div class="kv" v-if="error"><span>Error</span><code class="err">{{ error }}</code></div>
    </section>

    <section class="section">
      <button class="btn" @click="onCheck">Check permission</button>
      <button class="btn primary" @click="onRequest">Request permission</button>
      <button class="btn" :disabled="!!error && permission === 'denied'" @click="onFix">Get current position</button>
      <button class="btn" v-if="!watching" @click="startWatch">Start watching</button>
      <button class="btn danger" v-else @click="stopWatch">Stop watching</button>
    </section>

    <section class="section" v-if="position">
      <h3>Current position</h3>
      <div class="kv"><span>Latitude</span><code>{{ position.latitude.toFixed(6) }}</code></div>
      <div class="kv"><span>Longitude</span><code>{{ position.longitude.toFixed(6) }}</code></div>
      <div class="kv"><span>Accuracy</span><code>{{ position.accuracy?.toFixed(1) }} m</code></div>
      <div class="kv" v-if="position.altitude != null">
        <span>Altitude</span><code>{{ position.altitude.toFixed(1) }} m</code>
      </div>
      <div class="kv" v-if="position.speed != null">
        <span>Speed</span><code>{{ position.speed.toFixed(2) }} m/s</code>
      </div>
      <div class="kv" v-if="timestamp">
        <span>Fix time</span><code>{{ new Date(timestamp).toLocaleTimeString() }}</code>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import { useLocation } from '../composables/useLocation';

const {
  position, timestamp, error, permission, watching,
  checkPermission, requestPermission, getCurrent, startWatch, stopWatch,
} = useLocation();

onMounted(() => { checkPermission(); });

async function onCheck() { await checkPermission(); }
async function onRequest() { await requestPermission(); }
async function onFix() {
  if (permission.value !== 'granted') await requestPermission();
  await getCurrent();
}
</script>

<style scoped>
.section { padding: 12px 20px; }
.section h3 { margin: 4px 0 10px; font-size: 15px; }
.kv {
  display: flex; justify-content: space-between; gap: 12px;
  padding: 8px 0; border-bottom: 1px solid var(--border); font-size: 13px;
}
.kv code { color: var(--muted); }
.kv code.err { color: #dc2626; }
.btn {
  width: 100%;
  padding: 12px;
  margin-bottom: 8px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-weight: 600;
  cursor: pointer;
}
.btn.primary { background: var(--primary); color: #fff; border-color: var(--primary); }
.btn.danger  { background: #fee2e2; color: #b91c1c; border-color: #fecaca; }
.btn:disabled { opacity: 0.5; }
</style>
