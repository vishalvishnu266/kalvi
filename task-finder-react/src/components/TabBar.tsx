import { useLocation, useNavigate } from 'react-router-dom';
import { hapticTap } from '../hooks/useNative';

type Tab = { name: string; label: string; path: string; icon: string };

const tabs: Tab[] = [
  {
    name: 'sandbox', label: 'Sandbox', path: '/sandbox',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="16" rx="2"/><path d="M3 9h18"/><circle cx="7" cy="6.5" r="0.6" fill="currentColor"/><circle cx="9.5" cy="6.5" r="0.6" fill="currentColor"/></svg>',
  },
  {
    name: 'location', label: 'Location', path: '/location',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s-7-7.58-7-12a7 7 0 1 1 14 0c0 4.42-7 12-7 12z"/><circle cx="12" cy="10" r="2.5"/></svg>',
  },
  {
    name: 'device', label: 'Device', path: '/device',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="6" y="2" width="12" height="20" rx="2"/><path d="M11 19h2"/></svg>',
  },
  {
    name: 'settings', label: 'Settings', path: '/settings',
    icon: '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3H9a1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8V9a1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1z"/></svg>',
  },
];

/**
 * Determines which tab is "active". Detail routes map back to sandbox
 * (mirrors the Vue app's route.meta.tab convention).
 */
function currentTab(path: string): string {
  if (path.startsWith('/location')) return 'location';
  if (path.startsWith('/device')) return 'device';
  if (path.startsWith('/settings')) return 'settings';
  return 'sandbox';
}

export default function TabBar() {
  const location = useLocation();
  const navigate = useNavigate();
  const current = currentTab(location.pathname);

  function go(tab: Tab) {
    hapticTap();
    if (location.pathname !== tab.path) navigate(tab.path);
  }

  return (
    <nav className="tab-bar" role="tablist" aria-label="Main navigation">
      {tabs.map((tab) => (
        <button
          key={tab.name}
          className={`tab-btn ${current === tab.name ? 'active' : ''}`}
          role="tab"
          aria-selected={current === tab.name}
          aria-label={tab.label}
          onClick={() => go(tab)}
        >
          <span className="tab-icon" dangerouslySetInnerHTML={{ __html: tab.icon }} />
          <span className="tab-label">{tab.label}</span>
        </button>
      ))}
    </nav>
  );
}
