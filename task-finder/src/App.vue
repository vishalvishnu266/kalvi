<template>
  <div style="padding: 2rem; text-align: center;">
    <h1>Vue OTA Demo (v{{ appVersion }})</h1>
    <p>Status: {{ statusMessage }}</p>

    <button :disabled="isUpdating" @click="checkForUpdate">
      Check For Updates
    </button>

    <p style="margin-top:1rem; font-size:0.8rem; color:#888;">
      Auto-checking for updates every {{ pollSeconds }}s in the background.
    </p>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import { useOta } from './composables/useOta';

// __APP_VERSION__ is injected at build time by vite.config.js
declare const __APP_VERSION__: string;
const appVersion = __APP_VERSION__;

const pollSeconds = 15;
const { checkForUpdate, statusMessage, isUpdating, startAutoUpdate, stopAutoUpdate } = useOta();

onMounted(() => {
  startAutoUpdate(pollSeconds * 1000);
});

onUnmounted(() => {
  stopAutoUpdate();
});
</script>
