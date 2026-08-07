<template>
  <div class="page">
    <PageHeader title="Settings" />

    <section class="profile">
      <div class="avatar">👤</div>
      <div>
        <div class="name">Guest User</div>
        <div class="email">Sign in to sync your tasks & rides</div>
      </div>
    </section>

    <section class="group">
      <div class="group-title">Account</div>
      <button class="row"><span>Profile</span><span class="chev">›</span></button>
      <button class="row"><span>Payment methods</span><span class="chev">›</span></button>
      <button class="row"><span>Notifications</span><span class="chev">›</span></button>
    </section>

    <section class="group">
      <div class="group-title">Preferences</div>
      <button class="row"><span>Language</span><span class="value">English ›</span></button>
      <button class="row"><span>Currency</span><span class="value">USD ›</span></button>
    </section>

    <section class="group">
      <div class="group-title">App</div>
      <div class="row static">
        <div>
          <div>App version</div>
          <div class="sub">v{{ appVersion }}</div>
        </div>
        <span class="value">{{ statusMessage }}</span>
      </div>
      <button class="row" :disabled="isUpdating" @click="handleCheckUpdate">
        <span>{{ isUpdating ? 'Checking…' : 'Check for updates' }}</span>
        <span class="chev">{{ isUpdating ? '…' : '↻' }}</span>
      </button>
      <button class="row"><span>About</span><span class="chev">›</span></button>
      <button class="row"><span>Privacy Policy</span><span class="chev">›</span></button>
      <button class="row"><span>Terms of Service</span><span class="chev">›</span></button>
    </section>

    <button class="signout">Sign out</button>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import PageHeader from '../components/PageHeader.vue';
import { useOta } from '../composables/useOta';
import { hapticTap } from '../composables/useNative';

declare const __APP_VERSION__: string;
const appVersion = __APP_VERSION__;

const { checkForUpdate, statusMessage, isUpdating, startAutoUpdate, stopAutoUpdate } = useOta();

async function handleCheckUpdate() {
  await hapticTap();
  await checkForUpdate(false);
}

onMounted(() => {
  // Keep background auto-check running while user is on Settings tab too
  startAutoUpdate(30_000);
});
onUnmounted(() => {
  stopAutoUpdate();
});
</script>

<style scoped>
.profile {
  display: flex;
  align-items: center;
  gap: 14px;
  margin: 12px 20px 20px;
  padding: 16px;
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
}
.avatar {
  width: 48px;
  height: 48px;
  background: #eff6ff;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
}
.name { font-weight: 700; color: var(--text, #111827); }
.email { font-size: 12px; color: var(--muted, #6b7280); margin-top: 2px; }

.group {
  margin: 0 20px 20px;
  background: var(--surface, #fff);
  border: 1px solid var(--border, #e5e7eb);
  border-radius: 14px;
  overflow: hidden;
}
.group-title {
  font-size: 11px;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--muted, #6b7280);
  padding: 12px 14px 6px;
  font-weight: 600;
}
.row {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px;
  background: transparent;
  border: none;
  border-top: 1px solid var(--border, #e5e7eb);
  font-size: 14px;
  color: var(--text, #111827);
  text-align: left;
  cursor: pointer;
}
.row:first-of-type { border-top: none; }
.row.static { cursor: default; }
.row .sub { font-size: 11px; color: var(--muted, #6b7280); margin-top: 2px; }
.row .value { font-size: 13px; color: var(--muted, #6b7280); }
.row .chev { color: var(--muted, #9ca3af); font-size: 18px; }
.row:disabled { opacity: 0.6; }

.signout {
  margin: 4px 20px 24px;
  width: calc(100% - 40px);
  padding: 14px;
  background: transparent;
  color: #dc2626;
  border: 1px solid #fecaca;
  border-radius: 12px;
  font-weight: 600;
  cursor: pointer;
}
</style>
