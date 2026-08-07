import { ref } from 'vue';
import { Capacitor } from '@capacitor/core';
import { App as CapApp } from '@capacitor/app';
import { CapacitorUpdater } from '@capgo/capacitor-updater';

// Injected at build time via vite.config.js `define`
declare const __APP_VERSION__: string;

export function useOta() {
    const statusMessage = ref('Idle');
    const isUpdating = ref(false);
    let pollTimer: any = null;
    let appStateListener: any = null;

    // Tell native we booted successfully (prevents rollback)
    CapacitorUpdater.notifyAppReady();

    const getApiUrl = () => {
        if (Capacitor.getPlatform() === 'android') {
            // 10.0.2.2 = host machine as seen from Android Emulator
            return 'http://10.0.2.2:3000';
        }
        return 'http://localhost:3000';
    };

    async function getCurrentVersion(): Promise<string> {
        try {
            const current = await CapacitorUpdater.current();
            // If an OTA bundle is active use its version, otherwise use the baked-in build version
            const v = current?.bundle?.version;
            if (v && v !== 'builtin') return v;
        } catch (_) { /* ignore */ }
        return __APP_VERSION__;
    }

    async function checkForUpdate(silent = false) {
        if (isUpdating.value) return;
        isUpdating.value = true;
        if (!silent) statusMessage.value = 'Checking version...';

        try {
            const currentVersion = await getCurrentVersion();

            const response = await fetch(
                `${getApiUrl()}/api/check-update?version=${encodeURIComponent(currentVersion)}`
            );
            if (!response.ok) throw new Error('Failed to reach update server');

            const data = await response.json() as {
                update_available: boolean;
                version: string;
                url?: string;
            };

            if (!data.update_available || !data.url) {
                statusMessage.value = `Up to date (v${currentVersion})`;
                return;
            }

            statusMessage.value = `Downloading v${data.version}...`;
            const bundle = await CapacitorUpdater.download({
                url: data.url,
                version: data.version,
            });

            statusMessage.value = 'Applying update...';
            await CapacitorUpdater.set({ id: bundle.id });

            // Hot reloads the webview WITHOUT the user closing the app
            statusMessage.value = 'Reloading app...';
            await CapacitorUpdater.reload();

        } catch (err: any) {
            console.error('[OTA]', err);
            statusMessage.value = `Error: ${err?.message || 'Update failed'}`;
        } finally {
            isUpdating.value = false;
        }
    }

    /**
     * Poll the server for a new bundle at a fixed interval AND when the app
     * comes back to the foreground. This lets us push updates while the user
     * is still using the app.
     */
    function startAutoUpdate(intervalMs = 15000) {
        stopAutoUpdate();
        // Kick one off immediately
        checkForUpdate(true).catch(() => { /* noop */ });
        pollTimer = setInterval(() => {
            checkForUpdate(true).catch(() => { /* noop */ });
        }, intervalMs);

        // Also re-check when app resumes
        try {
            CapApp.addListener('appStateChange', (state: { isActive: boolean }) => {
                if (state.isActive) checkForUpdate(true).catch(() => { /* noop */ });
            }).then((h) => { appStateListener = h; });
        } catch (_) { /* @capacitor/app may not be installed on web */ }
    }

    function stopAutoUpdate() {
        if (pollTimer) {
            clearInterval(pollTimer);
            pollTimer = null;
        }
        if (appStateListener?.remove) {
            appStateListener.remove();
            appStateListener = null;
        }
    }

    return { checkForUpdate, statusMessage, isUpdating, startAutoUpdate, stopAutoUpdate };
}
