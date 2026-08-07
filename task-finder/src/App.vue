<template>
  <div style="padding: 2rem; text-align: center;">
    <h1>Vue OTA Demo (v1.0.0)</h1>
    <p>Status: {{ statusMessage }}</p>

    <button :disabled="isUpdating" @click="checkForUpdate">
      Check For Updates
    </button>
  </div>
</template>

<script setup ts>
import { useOta } from './composables/useOta';

const { checkForUpdate, statusMessage, isUpdating } = useOta();
</script>