import { App as CapApp, URLOpenListenerEvent } from '@capacitor/app';
import type { NavigateFunction } from 'react-router-dom';

/**
 * Deep link handler.
 *
 * Supported schemes:
 *   dailygig://task/<id>            -> /task/<id>
 *   dailygig://ride/<id>            -> /ride/<id>
 *   dailygig://sandbox              -> /sandbox
 *   https://dailygig.app/task/<id>  -> /task/<id>
 *
 * Register once at app bootstrap:
 *     initDeepLinks(navigate)
 */
export async function initDeepLinks(navigate: NavigateFunction) {
    const parse = (raw: string): string | null => {
        try {
            const url = new URL(raw);
            let path: string;
            if (url.protocol.startsWith('http')) {
                path = url.pathname;
            } else {
                path = `/${url.host}${url.pathname}`.replace(/\/+/g, '/');
            }
            if (url.search) path += url.search;
            return path;
        } catch (e) {
            console.warn('[deep-link] parse failed for', raw, e);
            return null;
        }
    };

    const route = (raw: string) => {
        const path = parse(raw);
        if (!path) return;
        console.log('[deep-link] ->', path);
        try {
            navigate(path);
        } catch (err) {
            console.warn('[deep-link] navigate failed', err);
        }
    };

    // 1) Cold start
    try {
        const { url } = (await CapApp.getLaunchUrl()) ?? { url: undefined };
        if (url) route(url);
    } catch {
        /* not available on web */
    }

    // 2) Warm start
    try {
        await CapApp.addListener('appUrlOpen', (event: URLOpenListenerEvent) => {
            if (event.url) route(event.url);
        });
    } catch (e) {
        console.warn('[deep-link] appUrlOpen listener unavailable', e);
    }
}
