<template>
  <Transition name="fade">
    <div v-if="visible" class="overlay">
      <div class="card">
        <div class="spinner" />
        <div class="msg">{{ message }}</div>
        <div class="sub">Do not close the app</div>
      </div>
    </div>
  </Transition>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useOta } from '../composables/useOta';

const { isApplying, statusMessage } = useOta();

// Only show the overlay during the *apply/reload* phase, NOT during the
// silent 15-second polls. That way the user never sees "Checking version…"
// as an intrusive dialog.
const visible = computed(() => isApplying.value);
const message = computed(() => statusMessage.value || 'Updating…');
</script>

<style scoped>
.overlay {
  position: fixed;
  inset: 0;
  background: rgba(15, 23, 42, 0.55);
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}
.card {
  background: #fff;
  padding: 22px 28px;
  border-radius: 14px;
  min-width: 220px;
  text-align: center;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.25);
}
.spinner {
  width: 32px;
  height: 32px;
  margin: 0 auto 12px;
  border-radius: 50%;
  border: 3px solid #e5e7eb;
  border-top-color: #2563eb;
  animation: spin 0.9s linear infinite;
}
@keyframes spin { to { transform: rotate(360deg); } }
.msg  { font-weight: 700; color: #111827; font-size: 15px; }
.sub  { font-size: 12px; color: #6b7280; margin-top: 4px; }

.fade-enter-active, .fade-leave-active { transition: opacity 0.15s ease; }
.fade-enter-from, .fade-leave-to { opacity: 0; }
</style>
