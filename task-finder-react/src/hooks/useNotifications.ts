import { useCallback, useState } from 'react';
import { Capacitor } from '@capacitor/core';

/**
 * Triggers a real system notification.
 *
 * On device (iOS / Android via Capacitor):
 *   - Uses @capacitor/local-notifications
 *   - Android plays the default notification sound (`sound: 'default'`)
 *   - iOS requires a bundled .caf/.wav in the app resources — omitted so
 *     we don't need extra native assets; the system alert tone is used.
 *
 * On web (during `npm run dev`):
 *   - Uses the browser Notification API for the popup
 *   - Uses WebAudio to synthesize a short "chime" so you get audible
 *     feedback without shipping any binary asset
 */
export function useNotifications() {
    const [permission, setPermission] = useState<string>('unknown');
    const [error, setError] = useState<string | null>(null);
    const [busy, setBusy] = useState(false);
    const isNative = Capacitor.isNativePlatform();

    // ---- Permissions ----------------------------------------------
    const checkPermission = useCallback(async () => {
        try {
            if (isNative) {
                const { LocalNotifications } = await import('@capacitor/local-notifications');
                const p = await LocalNotifications.checkPermissions();
                setPermission(p.display);
                return p.display;
            }
            if (typeof Notification !== 'undefined') {
                setPermission(Notification.permission);
                return Notification.permission;
            }
            setPermission('unsupported');
            return 'unsupported';
        } catch (e: any) {
            setError(e?.message || 'Permission check failed');
            return 'unknown';
        }
    }, [isNative]);

    const requestPermission = useCallback(async () => {
        try {
            if (isNative) {
                const { LocalNotifications } = await import('@capacitor/local-notifications');
                const p = await LocalNotifications.requestPermissions();
                setPermission(p.display);
                return p.display;
            }
            if (typeof Notification !== 'undefined') {
                const result = await Notification.requestPermission();
                setPermission(result);
                return result;
            }
            setPermission('unsupported');
            return 'unsupported';
        } catch (e: any) {
            setError(e?.message || 'Permission request failed');
            return 'denied';
        }
    }, [isNative]);

    // ---- Web audio chime (fallback for dev/browser) ---------------
    async function playWebChime() {
        try {
            const Ctor = (window as any).AudioContext || (window as any).webkitAudioContext;
            if (!Ctor) return;
            const ctx = new Ctor();
            const now = ctx.currentTime;
            const tones = [660, 990]; // two-note "ding"
            tones.forEach((freq, i) => {
                const osc = ctx.createOscillator();
                const gain = ctx.createGain();
                osc.type = 'sine';
                osc.frequency.value = freq;
                osc.connect(gain);
                gain.connect(ctx.destination);
                const start = now + i * 0.18;
                gain.gain.setValueAtTime(0.0001, start);
                gain.gain.exponentialRampToValueAtTime(0.35, start + 0.02);
                gain.gain.exponentialRampToValueAtTime(0.0001, start + 0.28);
                osc.start(start);
                osc.stop(start + 0.32);
            });
            // Close after playback so we don't leak audio contexts
            setTimeout(() => ctx.close().catch(() => {}), 800);
        } catch (e) {
            console.warn('[notifications] web chime failed', e);
        }
    }

    // ---- Main entrypoint ------------------------------------------
    const notify = useCallback(async (title = 'DailyGig', body = 'Ping! 🔔') => {
        setError(null);
        setBusy(true);
        try {
            // Make sure we have permission first
            let perm = permission;
            if (perm !== 'granted') perm = await requestPermission();
            if (perm !== 'granted') {
                setError(`Notification permission not granted (${perm})`);
                return false;
            }

            if (isNative) {
                const { LocalNotifications } = await import('@capacitor/local-notifications');
                await LocalNotifications.schedule({
                    notifications: [
                        {
                            // Random id keeps repeat presses from overwriting the previous one
                            id: Math.floor(Math.random() * 2_147_483_647),
                            title,
                            body,
                            // Fire ~1s from now so the OS treats it as a real notification
                            schedule: { at: new Date(Date.now() + 1000) },
                            // Android: play the default notification sound.
                            // iOS: providing 'default' also plays the default alert.
                            sound: 'default',
                            smallIcon: 'ic_stat_icon_config_sample',
                        },
                    ],
                });
                return true;
            }

            // Web fallback: popup + WebAudio chime
            try {
                new Notification(title, { body });
            } catch (e) {
                console.warn('[notifications] web Notification() failed', e);
            }
            await playWebChime();
            return true;
        } catch (e: any) {
            setError(e?.message || 'notify failed');
            return false;
        } finally {
            setBusy(false);
        }
    }, [isNative, permission, requestPermission]);

    return {
        permission, error, busy, isNative,
        checkPermission, requestPermission, notify,
    };
}
