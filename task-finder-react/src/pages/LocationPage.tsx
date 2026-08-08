import { useEffect } from 'react';
import PageHeader from '../components/PageHeader';
import { useLocation as useLocationHook } from '../hooks/useLocation';

export default function LocationPage() {
  const {
    position, timestamp, error, permission, watching,
    checkPermission, requestPermission, getCurrent, startWatch, stopWatch,
  } = useLocationHook();

  useEffect(() => { checkPermission(); }, [checkPermission]);

  async function onCheck() { await checkPermission(); }
  async function onRequest() { await requestPermission(); }
  async function onFix() {
    if (permission !== 'granted') await requestPermission();
    await getCurrent();
  }

  return (
    <div className="page">
      <PageHeader title="Location" subtitle="Test GPS + permissions" />

      <section className="section">
        <div className="kv"><span>Permission</span><code>{permission}</code></div>
        <div className="kv"><span>Watching</span><code>{watching ? 'yes' : 'no'}</code></div>
        {error && <div className="kv"><span>Error</span><code className="err">{error}</code></div>}
      </section>

      <section className="section">
        <button className="btn" style={{ marginBottom: 8, width: '100%' }} onClick={onCheck}>Check permission</button>
        <button className="btn primary" style={{ marginBottom: 8, width: '100%' }} onClick={onRequest}>Request permission</button>
        <button
          className="btn"
          style={{ marginBottom: 8, width: '100%' }}
          disabled={!!error && permission === 'denied'}
          onClick={onFix}
        >
          Get current position
        </button>
        {!watching ? (
          <button className="btn" style={{ marginBottom: 8, width: '100%' }} onClick={() => startWatch()}>Start watching</button>
        ) : (
          <button className="btn danger" style={{ marginBottom: 8, width: '100%' }} onClick={() => stopWatch()}>Stop watching</button>
        )}
      </section>

      {position && (
        <section className="section">
          <h3>Current position</h3>
          <div className="kv"><span>Latitude</span><code>{position.latitude.toFixed(6)}</code></div>
          <div className="kv"><span>Longitude</span><code>{position.longitude.toFixed(6)}</code></div>
          <div className="kv"><span>Accuracy</span><code>{position.accuracy?.toFixed(1)} m</code></div>
          {position.altitude != null && (
            <div className="kv"><span>Altitude</span><code>{position.altitude.toFixed(1)} m</code></div>
          )}
          {position.speed != null && (
            <div className="kv"><span>Speed</span><code>{position.speed.toFixed(2)} m/s</code></div>
          )}
          {timestamp && (
            <div className="kv">
              <span>Fix time</span><code>{new Date(timestamp).toLocaleTimeString()}</code>
            </div>
          )}
        </section>
      )}
    </div>
  );
}
