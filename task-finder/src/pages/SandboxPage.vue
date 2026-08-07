<template>
  <div class="page">
    <PageHeader title="Sandbox" subtitle="Test primitives before building UI" />

    <section class="section">
      <h3>What this app needs (before UI)</h3>
      <ol class="checklist">
        <li>✅ OTA hot updates (working)</li>
        <li>✅ Geolocation / GPS (Location tab)</li>
        <li>✅ Network status (Device tab)</li>
        <li>✅ Device info (Device tab)</li>
        <li>✅ Camera (below)</li>
        <li>⏳ Push notifications</li>
        <li>⏳ Local notifications</li>
        <li>⏳ Secure key/value storage (composable ready)</li>
        <li>✅ SQLite (below)</li>
        <li>⏳ Filesystem (offline cache)</li>
        <li>✅ Deep links (dailygig://…)</li>
        <li>⏳ HTTP client with auth interceptor</li>
        <li>⏳ Auth flow (sign-in, refresh token)</li>
        <li>⏳ Error reporting (Sentry / Crashlytics)</li>
        <li>⏳ Analytics</li>
        <li>⏳ Map SDK (Google / Mapbox)</li>
      </ol>
      <p class="note">Legend: ✅ done · ⏳ to do</p>
    </section>

    <section class="section">
      <h3>Take a selfie 📸</h3>
      <div class="kv"><span>Permission</span><code>{{ camPermission }}</code></div>
      <div class="kv" v-if="camError"><span>Error</span><code class="err">{{ camError }}</code></div>

      <div class="btn-row">
        <button class="btn" @click="camCheck">Check</button>
        <button class="btn primary" @click="camRequest">Request</button>
      </div>
      <div class="btn-row">
        <button class="btn primary" :disabled="camBusy" @click="onTake">
          {{ camBusy ? 'Opening…' : 'Take photo' }}
        </button>
        <button class="btn" :disabled="camBusy" @click="onPick">Pick from library</button>
      </div>

      <div v-if="photoDataUrl" class="preview">
        <img :src="photoDataUrl" alt="Captured photo" />
        <div class="kv"><span>Size</span><code>{{ estimatedKB }} KB (base64)</code></div>
        <button class="btn danger" @click="camClear">Clear</button>
      </div>
    </section>

    <section class="section">
      <h3>SQLite 🗄️</h3>
      <div class="kv" v-if="sqlError"><span>Error</span><code class="err">{{ sqlError }}</code></div>
      <input v-model="noteBody" class="input" placeholder="Type a note and press Save" />
      <div class="btn-row">
        <button class="btn primary" :disabled="!noteBody.trim()" @click="onAddNote">Save</button>
        <button class="btn" @click="onLoadNotes">Refresh</button>
        <button class="btn danger" @click="onClearNotes">Clear all</button>
      </div>
      <div class="notes" v-if="notes.length">
        <div v-for="n in notes" :key="n.id" class="note-row">
          <div>
            <div class="note-body">{{ n.body }}</div>
            <div class="note-meta">#{{ n.id }} · {{ formatWhen(n.created_at) }}</div>
          </div>
          <button class="link" @click="onDeleteNote(n.id)">Delete</button>
        </div>
      </div>
      <p v-else class="note">No notes yet — data persists across app restarts.</p>
    </section>

    <section class="section">
      <h3>Deep links 🔗</h3>
      <p class="note">
        Trigger from inside the app, or from the phone shell:
      </p>
      <pre class="code">adb shell am start -a android.intent.action.VIEW \
  -d "dailygig://task/42?ref=push"</pre>

      <div class="btn-row">
        <button class="btn" @click="$router.push('/task/42?ref=in-app')">Open /task/42</button>
        <button class="btn" @click="$router.push('/ride/7')">Open /ride/7</button>
      </div>
    </section>

    <section class="section">
      <h3>Build info</h3>
      <div class="kv"><span>Bundle version</span><code>{{ appVersion }}</code></div>
      <div class="kv"><span>Platform</span><code>{{ platform }}</code></div>
      <div class="kv"><span>User agent</span><code class="small">{{ ua }}</code></div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { Capacitor } from '@capacitor/core';
import PageHeader from '../components/PageHeader.vue';
import { useCamera } from '../composables/useCamera';
import { useSqlite } from '../composables/useSqlite';

// -- SQLite state ---------------------------------------------------
interface NoteRow { id: number; body: string; created_at: number }
const noteBody = ref('');
const notes = ref<NoteRow[]>([]);
const sqlError = ref<string | null>(null);
// Single-table test scaffold — do not extend, this is only to verify
// SQLite is installed and working on the device.
const { run, query, reset, error: sqliteInitError } = useSqlite();

async function onLoadNotes() {
  try {
    sqlError.value = null;
    notes.value = await query<NoteRow>('SELECT id, body, created_at FROM notes ORDER BY id DESC LIMIT 100');
  } catch (e: any) { sqlError.value = e?.message || 'query failed'; }
}
async function onAddNote() {
  const body = noteBody.value.trim();
  if (!body) return;
  try {
    sqlError.value = null;
    await run('INSERT INTO notes (body) VALUES (?)', [body]);
    noteBody.value = '';
    await onLoadNotes();
  } catch (e: any) { sqlError.value = e?.message || 'insert failed'; }
}
async function onDeleteNote(id: number) {
  try {
    await run('DELETE FROM notes WHERE id = ?', [id]);
    await onLoadNotes();
  } catch (e: any) { sqlError.value = e?.message || 'delete failed'; }
}
async function onClearNotes() {
  try { await reset(); await onLoadNotes(); }
  catch (e: any) { sqlError.value = e?.message || 'clear failed'; }
}
function formatWhen(secs: number) {
  return new Date(secs * 1000).toLocaleString();
}
onMounted(() => { onLoadNotes(); });
// Propagate any bootstrap failure to the UI
setTimeout(() => { if (sqliteInitError.value) sqlError.value = sqliteInitError.value; }, 500);

declare const __APP_VERSION__: string;
const appVersion = __APP_VERSION__;
const platform = Capacitor.getPlatform();
const ua = typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown';

const {
  dataUrl: photoDataUrl,
  error: camError,
  busy: camBusy,
  permission: camPermission,
  checkPermission, requestPermission, takePhoto, pickPhoto, clear: camClear,
} = useCamera();

const estimatedKB = computed(() =>
  photoDataUrl.value ? Math.round((photoDataUrl.value.length * 3 / 4) / 1024) : 0
);

onMounted(() => { checkPermission(); });

async function camCheck() { await checkPermission(); }
async function camRequest() { await requestPermission(); }
async function onTake() {
  if (camPermission.value !== 'granted') await requestPermission();
  await takePhoto();
}
async function onPick() {
  await pickPhoto();
}
</script>

<style scoped>
.section { padding: 12px 20px; }
.section h3 { margin: 4px 0 10px; font-size: 15px; }
.checklist { padding-left: 20px; margin: 0; line-height: 1.8; font-size: 14px; }
.note { font-size: 12px; color: var(--muted); margin-top: 10px; }

.kv {
  display: flex; justify-content: space-between; gap: 12px;
  padding: 8px 0; border-bottom: 1px solid var(--border); font-size: 13px;
}
.kv code { color: var(--muted); word-break: break-all; text-align: right; }
.kv code.small { font-size: 11px; }
.kv code.err { color: #dc2626; }

.btn-row { display: flex; gap: 8px; margin-top: 10px; }
.btn {
  flex: 1;
  padding: 12px;
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

.preview {
  margin-top: 14px;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 12px;
  background: var(--surface);
}
.preview img {
  width: 100%;
  border-radius: 8px;
  display: block;
  margin-bottom: 10px;
}
.code {
  background: #0f172a;
  color: #e2e8f0;
  padding: 10px 12px;
  border-radius: 8px;
  font-size: 11px;
  line-height: 1.5;
  overflow-x: auto;
  white-space: pre;
  margin: 8px 0 12px;
}
.input {
  width: 100%;
  padding: 10px 12px;
  border: 1px solid var(--border);
  border-radius: 10px;
  font-size: 14px;
  background: var(--surface);
  color: var(--text);
  margin: 8px 0;
  outline: none;
}
.input:focus { border-color: var(--primary); }
.notes {
  margin-top: 12px;
  border: 1px solid var(--border);
  border-radius: 10px;
  background: var(--surface);
  overflow: hidden;
}
.note-row {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 10px;
  padding: 10px 12px;
  border-top: 1px solid var(--border);
}
.note-row:first-child { border-top: none; }
.note-body { font-size: 13px; color: var(--text); word-break: break-word; }
.note-meta { font-size: 11px; color: var(--muted); margin-top: 2px; }
.link {
  background: transparent;
  border: none;
  color: #dc2626;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  padding: 0;
}
</style>
