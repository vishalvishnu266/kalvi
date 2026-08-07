<template>
  <div class="page">
    <PageHeader title="Device" subtitle="Native info & network status" />

    <section class="section">
      <h3>Network</h3>
      <div class="kv">
        <span>Status</span>
        <code :class="network.connected ? 'ok' : 'err'">
          {{ network.connected ? 'online' : 'offline' }}
        </code>
      </div>
      <div class="kv"><span>Type</span><code>{{ network.connectionType }}</code></div>
    </section>

    <section class="section" v-if="info">
      <h3>Device</h3>
      <div class="kv"><span>Platform</span><code>{{ info.platform }}</code></div>
      <div class="kv"><span>Model</span><code>{{ info.model }}</code></div>
      <div class="kv"><span>OS</span><code>{{ info.operatingSystem }} {{ info.osVersion }}</code></div>
      <div class="kv"><span>Manufacturer</span><code>{{ info.manufacturer }}</code></div>
      <div class="kv"><span>Virtual</span><code>{{ info.isVirtual ? 'yes (emulator)' : 'no (real device)' }}</code></div>
      <div class="kv"><span>Web view</span><code>{{ info.webViewVersion }}</code></div>
    </section>

    <section class="section" v-if="battery">
      <h3>Battery</h3>
      <div class="kv"><span>Level</span><code>{{ Math.round((battery.batteryLevel || 0) * 100) }}%</code></div>
      <div class="kv"><span>Charging</span><code>{{ battery.isCharging ? 'yes' : 'no' }}</code></div>
    </section>

    <button class="btn" @click="refresh">Refresh</button>
  </div>
</template>

<script setup lang="ts">
import PageHeader from '../components/PageHeader.vue';
import { useDevice } from '../composables/useDevice';

const { info, battery, network, refresh } = useDevice();
</script>

<style scoped>
.section { padding: 12px 20px; }
.section h3 { margin: 4px 0 10px; font-size: 15px; }
.kv {
  display: flex; justify-content: space-between; gap: 12px;
  padding: 8px 0; border-bottom: 1px solid var(--border); font-size: 13px;
}
.kv code { color: var(--muted); }
.kv code.ok { color: #059669; }
.kv code.err { color: #dc2626; }
.btn {
  width: calc(100% - 40px);
  margin: 12px 20px 24px;
  padding: 12px;
  border-radius: 10px;
  border: 1px solid var(--border);
  background: var(--surface);
  color: var(--text);
  font-weight: 600;
  cursor: pointer;
}
</style>
