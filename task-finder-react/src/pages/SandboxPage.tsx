import { useEffect, useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Capacitor } from '@capacitor/core';
import PageHeader from '../components/PageHeader';
import { useCamera } from '../hooks/useCamera';
import { useSqlite } from '../hooks/useSqlite';

declare const __APP_VERSION__: string;

interface NoteRow { id: number; body: string; created_at: number }

export default function SandboxPage() {
  const navigate = useNavigate();

  // ---- SQLite ------------------------------------------------------
  const [noteBody, setNoteBody] = useState('');
  const [notes, setNotes] = useState<NoteRow[]>([]);
  const [sqlError, setSqlError] = useState<string | null>(null);
  const { run, query, reset, error: sqliteInitError } = useSqlite();

  async function onLoadNotes() {
    try {
      setSqlError(null);
      const rows = await query<NoteRow>(
        'SELECT id, body, created_at FROM notes ORDER BY id DESC LIMIT 100'
      );
      setNotes(rows);
    } catch (e: any) {
      setSqlError(e?.message || 'query failed');
    }
  }
  async function onAddNote() {
    const body = noteBody.trim();
    if (!body) return;
    try {
      setSqlError(null);
      await run('INSERT INTO notes (body) VALUES (?)', [body]);
      setNoteBody('');
      await onLoadNotes();
    } catch (e: any) {
      setSqlError(e?.message || 'insert failed');
    }
  }
  async function onDeleteNote(id: number) {
    try {
      await run('DELETE FROM notes WHERE id = ?', [id]);
      await onLoadNotes();
    } catch (e: any) {
      setSqlError(e?.message || 'delete failed');
    }
  }
  async function onClearNotes() {
    try { await reset(); await onLoadNotes(); }
    catch (e: any) { setSqlError(e?.message || 'clear failed'); }
  }
  function formatWhen(secs: number) {
    return new Date(secs * 1000).toLocaleString();
  }

  useEffect(() => {
    onLoadNotes();
    // Propagate any bootstrap failure to the UI
    const t = setTimeout(() => {
      if (sqliteInitError) setSqlError(sqliteInitError);
    }, 500);
    return () => clearTimeout(t);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  // ---- Camera ------------------------------------------------------
  const {
    dataUrl: photoDataUrl,
    error: camError,
    busy: camBusy,
    permission: camPermission,
    checkPermission, requestPermission, takePhoto, pickPhoto, clear: camClear,
  } = useCamera();

  useEffect(() => { checkPermission(); }, [checkPermission]);

  const estimatedKB = useMemo(
    () => (photoDataUrl ? Math.round((photoDataUrl.length * 3 / 4) / 1024) : 0),
    [photoDataUrl]
  );

  async function camCheck() { await checkPermission(); }
  async function camRequest() { await requestPermission(); }
  async function onTake() {
    if (camPermission !== 'granted') await requestPermission();
    await takePhoto();
  }
  async function onPick() { await pickPhoto(); }

  // ---- Build info --------------------------------------------------
  const appVersion = __APP_VERSION__;
  const platform = Capacitor.getPlatform();
  const ua = typeof navigator !== 'undefined' ? navigator.userAgent : 'unknown';

  return (
    <div className="page">
      <PageHeader title="Sandbox" subtitle="Test primitives before building UI" />

      <section className="section">
        <h3>What this app needs (before UI)</h3>
        <ol className="checklist">
          <li>✅ OTA hot updates (working)</li>
          <li>✅ Geolocation / GPS (Location tab)</li>
          <li>✅ Network status (Device tab)</li>
          <li>✅ Device info (Device tab)</li>
          <li>✅ Camera (below)</li>
          <li>⏳ Push notifications</li>
          <li>⏳ Local notifications</li>
          <li>⏳ Secure key/value storage (hook ready)</li>
          <li>✅ SQLite (below)</li>
          <li>⏳ Filesystem (offline cache)</li>
          <li>✅ Deep links (dailygig://…)</li>
          <li>⏳ HTTP client with auth interceptor</li>
          <li>⏳ Auth flow (sign-in, refresh token)</li>
          <li>⏳ Error reporting (Sentry / Crashlytics)</li>
          <li>⏳ Analytics</li>
          <li>⏳ Map SDK (Google / Mapbox)</li>
        </ol>
        <p className="note">Legend: ✅ done · ⏳ to do</p>
      </section>

      <section className="section">
        <h3>Take a selfie 📸</h3>
        <div className="kv"><span>Permission</span><code>{camPermission}</code></div>
        {camError && (
          <div className="kv"><span>Error</span><code className="err">{camError}</code></div>
        )}

        <div className="btn-row">
          <button className="btn" onClick={camCheck}>Check</button>
          <button className="btn primary" onClick={camRequest}>Request</button>
        </div>
        <div className="btn-row">
          <button className="btn primary" disabled={camBusy} onClick={onTake}>
            {camBusy ? 'Opening…' : 'Take photo'}
          </button>
          <button className="btn" disabled={camBusy} onClick={onPick}>Pick from library</button>
        </div>

        {photoDataUrl && (
          <div className="preview">
            <img src={photoDataUrl} alt="Captured" />
            <div className="kv"><span>Size</span><code>{estimatedKB} KB (base64)</code></div>
            <button className="btn danger" onClick={camClear}>Clear</button>
          </div>
        )}
      </section>

      <section className="section">
        <h3>SQLite 🗄️</h3>
        {sqlError && (
          <div className="kv"><span>Error</span><code className="err">{sqlError}</code></div>
        )}
        <input
          className="input"
          value={noteBody}
          onChange={(e) => setNoteBody(e.target.value)}
          placeholder="Type a note and press Save"
        />
        <div className="btn-row">
          <button className="btn primary" disabled={!noteBody.trim()} onClick={onAddNote}>Save</button>
          <button className="btn" onClick={onLoadNotes}>Refresh</button>
          <button className="btn danger" onClick={onClearNotes}>Clear all</button>
        </div>
        {notes.length > 0 ? (
          <div className="notes">
            {notes.map((n) => (
              <div key={n.id} className="note-row">
                <div>
                  <div className="note-body">{n.body}</div>
                  <div className="note-meta">#{n.id} · {formatWhen(n.created_at)}</div>
                </div>
                <button className="link" onClick={() => onDeleteNote(n.id)}>Delete</button>
              </div>
            ))}
          </div>
        ) : (
          <p className="note">No notes yet — data persists across app restarts.</p>
        )}
      </section>

      <section className="section">
        <h3>Deep links 🔗</h3>
        <p className="note">
          Trigger from inside the app, or from the phone shell:
        </p>
        <pre className="code">{`adb shell am start -a android.intent.action.VIEW \\
  -d "dailygig://task/42?ref=push"`}</pre>

        <div className="btn-row">
          <button className="btn" onClick={() => navigate('/task/42?ref=in-app')}>Open /task/42</button>
          <button className="btn" onClick={() => navigate('/ride/7')}>Open /ride/7</button>
        </div>
      </section>

      <section className="section">
        <h3>Build info</h3>
        <div className="kv"><span>Bundle version</span><code>{appVersion}</code></div>
        <div className="kv"><span>Platform</span><code>{platform}</code></div>
        <div className="kv"><span>User agent</span><code className="small">{ua}</code></div>
      </section>
    </div>
  );
}
