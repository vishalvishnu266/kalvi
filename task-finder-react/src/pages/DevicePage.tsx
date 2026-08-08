import PageHeader from '../components/PageHeader';
import { useDevice } from '../hooks/useDevice';

export default function DevicePage() {
  const { info, battery, network, refresh } = useDevice();

  return (
    <div className="page">
      <PageHeader title="Device" subtitle="Native info & network status" />

      <section className="section">
        <h3>Network</h3>
        <div className="kv">
          <span>Status</span>
          <code className={network.connected ? 'ok' : 'err'}>
            {network.connected ? 'online' : 'offline'}
          </code>
        </div>
        <div className="kv"><span>Type</span><code>{network.connectionType}</code></div>
      </section>

      {info && (
        <section className="section">
          <h3>Device</h3>
          <div className="kv"><span>Platform</span><code>{info.platform}</code></div>
          <div className="kv"><span>Model</span><code>{info.model}</code></div>
          <div className="kv"><span>OS</span><code>{info.operatingSystem} {info.osVersion}</code></div>
          <div className="kv"><span>Manufacturer</span><code>{info.manufacturer}</code></div>
          <div className="kv"><span>Virtual</span><code>{info.isVirtual ? 'yes (emulator)' : 'no (real device)'}</code></div>
          <div className="kv"><span>Web view</span><code>{info.webViewVersion}</code></div>
        </section>
      )}

      {battery && (
        <section className="section">
          <h3>Battery</h3>
          <div className="kv"><span>Level</span><code>{Math.round((battery.batteryLevel || 0) * 100)}%</code></div>
          <div className="kv"><span>Charging</span><code>{battery.isCharging ? 'yes' : 'no'}</code></div>
        </section>
      )}

      <button className="btn" style={{ margin: '12px 20px 24px', width: 'calc(100% - 40px)' }} onClick={() => refresh()}>
        Refresh
      </button>
    </div>
  );
}
