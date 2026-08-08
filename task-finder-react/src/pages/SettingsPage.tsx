import PageHeader from '../components/PageHeader';
import { useOta } from '../hooks/useOta';
import { hapticTap } from '../hooks/useNative';
import { useNotifications } from '../hooks/useNotifications';

declare const __APP_VERSION__: string;
const appVersion = __APP_VERSION__;

export default function SettingsPage() {
  // Global auto-poll is started in App.tsx — this hook just subscribes.
  const { checkForUpdate, statusMessage, isUpdating } = useOta();

  const {
    permission: notifPermission,
    error: notifError,
    busy: notifBusy,
    isNative,
    requestPermission: requestNotifPermission,
    notify,
  } = useNotifications();

  async function handleCheckUpdate() {
    await hapticTap();
    await checkForUpdate();
  }

  async function handleRequestNotif() {
    await hapticTap();
    await requestNotifPermission();
  }

  async function handleTriggerNotif() {
    await hapticTap();
    await notify('DailyGig 🔔', 'Test notification with sound');
  }

  return (
    <div className="page">
      <PageHeader title="Settings" />

      <section className="profile">
        <div className="avatar">👤</div>
        <div>
          <div className="name">Guest User</div>
          <div className="email">Sign in to sync your tasks & rides</div>
        </div>
      </section>

      {/* --- Notifications: real, working, with sound ---------------- */}
      <section className="group">
        <div className="group-title">Notifications</div>
        <div className="row static">
          <div>
            <div>Permission</div>
            <div className="sub">
              {isNative ? 'Native (LocalNotifications)' : 'Web (Notification API + WebAudio)'}
            </div>
          </div>
          <span className="value">{notifPermission}</span>
        </div>
        {notifPermission !== 'granted' && (
          <button className="row" disabled={notifBusy} onClick={handleRequestNotif}>
            <span>{notifBusy ? 'Requesting…' : 'Request permission'}</span>
            <span className="chev">›</span>
          </button>
        )}
        <button
          className="row"
          disabled={notifBusy}
          onClick={handleTriggerNotif}
        >
          <span>{notifBusy ? 'Sending…' : 'Send test notification 🔔'}</span>
          <span className="chev">▶</span>
        </button>
        {notifError && (
          <div className="row static" style={{ color: '#dc2626' }}>
            <span>Error</span>
            <span className="value" style={{ color: '#dc2626' }}>{notifError}</span>
          </div>
        )}
      </section>

      {/* --- App / OTA (real, working) ------------------------------- */}
      <section className="group">
        <div className="group-title">App</div>
        <div className="row static">
          <div>
            <div>App version</div>
            <div className="sub">v{appVersion}</div>
          </div>
          <span className="value">{statusMessage}</span>
        </div>
        <button className="row" disabled={isUpdating} onClick={handleCheckUpdate}>
          <span>{isUpdating ? 'Checking…' : 'Check for updates'}</span>
          <span className="chev">{isUpdating ? '…' : '↻'}</span>
        </button>
      </section>
    </div>
  );
}
