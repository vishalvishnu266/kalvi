import { useEffect } from 'react';
import { create } from 'zustand';
import { useQuery, useMutation } from '@tanstack/react-query';
import { Capacitor } from '@capacitor/core';
import { App as CapApp } from '@capacitor/app';
import { CapacitorUpdater } from '@capgo/capacitor-updater';
import { queryClient } from '../lib/queryClient';

// Injected at build time via vite.config.ts `define`
declare const __APP_VERSION__: string;

// ----------------------------------------------------------------------
// Types & constants
// ----------------------------------------------------------------------

export interface CheckUpdateResponse {
    update_available: boolean;
    version: string;
    url?: string;
}

export const OTA_QUERY_KEY = ['ota', 'check-update'] as const;

// ----------------------------------------------------------------------
// Zustand store — UI-facing state (survives unmounts, shared by
// <UpdateOverlay/> and <SettingsPage/>). We keep it minimal now that
// TanStack Query owns the network state.
// ----------------------------------------------------------------------

interface OtaUiState {
    isApplying: boolean;
    statusMessage: string;
    setApplying: (v: boolean) => void;
    setStatus: (msg: string) => void;
}

export const useOtaStore = create<OtaUiState>((set) => ({
    isApplying: false,
    statusMessage: 'Idle',
    setApplying: (v) => set({ isApplying: v }),
    setStatus: (msg) => set({ statusMessage: msg }),
}));

// Guard against re-applying the same version repeatedly
let lastAppliedVersion: string | null = null;
let appStateListener: any = null;
let bootstrapped = false;

// Tell native we booted successfully (fires once, safely on web too)
try { CapacitorUpdater.notifyAppReady(); } catch { /* web */ }

// ----------------------------------------------------------------------
// Pure helpers
// ----------------------------------------------------------------------

function getApiUrl() {
    const host: string = (globalThis as any).__OTA_HOST__ || '192.168.0.4';
    const port: number = (globalThis as any).__OTA_PORT__ || 3001;
    return `http://${host}:${port}`;
}

async function getCurrentVersion(): Promise<string> {
    try {
        const current = await CapacitorUpdater.current();
        const v = current?.bundle?.version;
        if (v && v !== 'builtin') return v;
    } catch { /* web / not native */ }
    return __APP_VERSION__;
}

/** GET /api/check-update?version=… — the pure fetch bit. */
async function fetchUpdateInfo(): Promise<CheckUpdateResponse> {
    const currentVersion = await getCurrentVersion();
    const res = await fetch(
        `${getApiUrl()}/api/check-update?version=${encodeURIComponent(currentVersion)}`,
        { cache: 'no-store' }
    );
    if (!res.ok) throw new Error('Failed to reach update server');
    return (await res.json()) as CheckUpdateResponse;
}

// ----------------------------------------------------------------------
// Query + Mutation
// ----------------------------------------------------------------------

/**
 * Poll the OTA server for update availability.
 *
 * `refetchInterval` gives us the every-15s cadence for free, and TanStack
 * Query dedupes concurrent calls, retries with backoff, and pauses when
 * the tab is hidden — better than the hand-rolled setInterval we had.
 */
export function useCheckUpdateQuery(intervalMs = 15_000) {
    return useQuery({
        queryKey: OTA_QUERY_KEY,
        queryFn: fetchUpdateInfo,
        refetchInterval: intervalMs,
        // Even if the query is unused for a moment, keep polling — the
        // <UpdateOverlay> or Settings row will mount/unmount but we still
        // want the app-wide auto-updater to run.
        refetchIntervalInBackground: false,
        // Don't retry aggressively when the LAN server is off; it just
        // spams the console.
        retry: 1,
    });
}

/**
 * Download + apply the latest bundle. This is the destructive side
 * effect that the query result triggers on success.
 */
export function useApplyUpdateMutation() {
    const setApplying = useOtaStore((s) => s.setApplying);
    const setStatus = useOtaStore((s) => s.setStatus);

    return useMutation({
        mutationKey: ['ota', 'apply'],
        mutationFn: async (info: CheckUpdateResponse) => {
            if (!info.update_available || !info.url) return { skipped: true as const };
            if (lastAppliedVersion === info.version) return { skipped: true as const };

            setStatus(`Downloading v${info.version}...`);
            const bundle = await CapacitorUpdater.download({
                url: info.url,
                version: info.version,
            });

            setApplying(true);
            setStatus('Applying update...');
            await CapacitorUpdater.set({ id: bundle.id });
            lastAppliedVersion = info.version;

            setStatus('Reloading app...');
            try {
                await CapacitorUpdater.reload();
            } catch (e) {
                console.warn('[OTA] CapacitorUpdater.reload() failed, falling back', e);
                window.location.reload();
            }
            return { skipped: false as const, version: info.version };
        },
        onError: (err: any) => {
            console.error('[OTA] apply failed', err);
            setStatus(`Error: ${err?.message || 'Update failed'}`);
            setApplying(false);
        },
    });
}

// ----------------------------------------------------------------------
// App-level orchestrator hook
// ----------------------------------------------------------------------

/**
 * Mount this once (e.g. in <App />) to:
 *   - start polling the OTA server
 *   - auto-download+apply whenever an update is available
 *   - re-check when the app returns to the foreground
 *
 * Because the query/mutation live in TanStack Query's cache, any other
 * component can subscribe to the same data via `useCheckUpdateQuery()`
 * without triggering an extra network call.
 */
export function useAutoUpdater(intervalMs = 15_000) {
    const query = useCheckUpdateQuery(intervalMs);
    const apply = useApplyUpdateMutation();
    const setStatus = useOtaStore((s) => s.setStatus);
    const isApplying = useOtaStore((s) => s.isApplying);

    // Sync query results into the user-facing status line
    useEffect(() => {
        if (query.isError) {
            setStatus(`Error: ${(query.error as Error)?.message || 'check failed'}`);
            return;
        }
        if (!query.data) return;
        if (!query.data.update_available) {
            setStatus(`Up to date (v${query.data.version})`);
        } else if (lastAppliedVersion === query.data.version) {
            setStatus(`Applied v${query.data.version}, waiting for reload…`);
        } else {
            setStatus(`Update available: v${query.data.version}`);
        }
    }, [query.data, query.isError, query.error, setStatus]);

    // Auto-apply whenever the query surfaces a new update, unless we're
    // already applying one.
    useEffect(() => {
        if (!query.data?.update_available) return;
        if (isApplying || apply.isPending) return;
        if (lastAppliedVersion === query.data.version) return;
        apply.mutate(query.data);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [query.data, isApplying, apply.isPending]);

    // Foreground → force a fresh check (bypasses the polling window)
    useEffect(() => {
        if (bootstrapped) return;
        bootstrapped = true;
        try {
            CapApp.addListener('appStateChange', (s: { isActive: boolean }) => {
                if (s.isActive) queryClient.invalidateQueries({ queryKey: OTA_QUERY_KEY });
            }).then((h) => { appStateListener = h; });
        } catch { /* web */ }
        return () => {
            appStateListener?.remove?.();
            appStateListener = null;
            bootstrapped = false;
        };
    }, []);
}

// ----------------------------------------------------------------------
// Convenience hook for read-only consumers (Settings, Overlay)
// ----------------------------------------------------------------------

export function useOta() {
    const query = useCheckUpdateQuery();
    const apply = useApplyUpdateMutation();
    const statusMessage = useOtaStore((s) => s.statusMessage);
    const isApplying = useOtaStore((s) => s.isApplying);

    /** Force a fresh manual check (invalidates the cache). */
    const checkForUpdate = async () => {
        await queryClient.invalidateQueries({ queryKey: OTA_QUERY_KEY });
        await query.refetch();
    };

    return {
        checkForUpdate,
        statusMessage,
        isUpdating: query.isFetching,
        isApplying: isApplying || apply.isPending,
        data: query.data,
        error: query.error,
        platform: Capacitor.getPlatform(),
    };
}
