import { ref } from 'vue';
import { Capacitor } from '@capacitor/core';
import { CapacitorUpdater } from '@capgo/capacitor-updater';

export function useOta() {
    const statusMessage = ref('Idle');
    const isUpdating = ref(false);

    // Inform native layer app JS initialized successfully
    CapacitorUpdater.notifyAppReady();

    const getApiUrl = () => {
        if (Capacitor.getPlatform() === 'android') {
            return 'http://10.0.2.2:3000'; // Host machine IP inside Android Emulator
        }
        return 'http://localhost:3000';
    };

    async function checkForUpdate() {
        isUpdating.value = true;
        statusMessage.value = 'Checking version...';

        try {
            // 1. Get currently running bundle details
            const latestStats = await CapacitorUpdater.getLatest();
            const currentVersion = latestStats?.version || '1.0.0';

            // 2. Fetch from Axum
            const response = await fetch(`${getApiUrl()}/api/check-update?version=${currentVersion}`);
            if (!response.ok) throw new Error('Failed to reach update server');

            const data = await response.json(); // { update_available, version, url }

            if (!data.update_available || !data.url) {
                statusMessage.value = `Up to date (v${currentVersion})`;
                return;
            }

            // 3. Download update bundle .zip
            statusMessage.value = `Downloading v${data.version}...`;
            const bundle = await CapacitorUpdater.download({
                url: data.url,
                version: data.version,
            });

            // 4. Set update as active bundle
            statusMessage.value = 'Applying update...';
            await CapacitorUpdater.set({ id: bundle.id });

            // 5. Hot Reload App
            statusMessage.value = 'Reloading app...';
            await CapacitorUpdater.reload();

        } catch (err: any) {
            console.error(err);
            statusMessage.value = `Error: ${err.message || 'Update failed'}`;
        } finally {
            isUpdating.value = false;
        }
    }

    return { checkForUpdate, statusMessage, isUpdating };
}