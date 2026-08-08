import { useOta } from '../hooks/useOta';

export default function UpdateOverlay() {
  const { isApplying, statusMessage } = useOta();
  // Only show the overlay during the *apply/reload* phase, NOT during the
  // silent 15-second polls.
  if (!isApplying) return null;
  const message = statusMessage || 'Updating…';
  return (
    <div className="overlay">
      <div className="card">
        <div className="spinner" />
        <div className="msg">{message}</div>
        <div className="sub">Do not close the app</div>
      </div>
    </div>
  );
}
