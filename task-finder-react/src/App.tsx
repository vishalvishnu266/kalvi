import { useEffect } from 'react';
import { Routes, Route, Navigate, useNavigate } from 'react-router-dom';
import TabBar from './components/TabBar';
import UpdateOverlay from './components/UpdateOverlay';
import { initNative } from './hooks/useNative';
import { initDeepLinks } from './hooks/useDeepLinks';
import { useAutoUpdater } from './hooks/useOta';

import SandboxPage from './pages/SandboxPage';
import LocationPage from './pages/LocationPage';
import DevicePage from './pages/DevicePage';
import SettingsPage from './pages/SettingsPage';
import TaskDetailPage from './pages/TaskDetailPage';
import RideDetailPage from './pages/RideDetailPage';

export default function App() {
  const navigate = useNavigate();

  // App-wide OTA poller — TanStack Query drives the 15s poll and the
  // auto-apply-on-new-version side effect. Runs regardless of which tab is
  // active so a user stuck on the Sandbox tab still receives hot updates.
  useAutoUpdater(15_000);

  useEffect(() => {
    initNative();
    initDeepLinks(navigate);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div className="app-shell">
      <main className="app-main">
        <Routes>
          <Route path="/" element={<Navigate to="/sandbox" replace />} />
          <Route path="/sandbox" element={<SandboxPage />} />
          <Route path="/location" element={<LocationPage />} />
          <Route path="/device" element={<DevicePage />} />
          <Route path="/settings" element={<SettingsPage />} />
          <Route path="/task/:id" element={<TaskDetailPage />} />
          <Route path="/ride/:id" element={<RideDetailPage />} />
          <Route path="*" element={<Navigate to="/sandbox" replace />} />
        </Routes>
      </main>
      <TabBar />
      <UpdateOverlay />
    </div>
  );
}
