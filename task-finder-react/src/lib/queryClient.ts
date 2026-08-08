import { QueryClient } from '@tanstack/react-query';

/**
 * Shared TanStack Query client.
 *
 * Kept in its own module so both the app and future test harnesses can
 * import the same singleton. Tuned for a mobile app:
 *  - `refetchOnWindowFocus: false` — Capacitor emits focus events for every
 *    tab switch, which would hammer the OTA/API server unnecessarily.
 *  - `retry: 2` — one retry is often not enough on flaky mobile networks.
 *  - Long stale time — most of the app's data is user-owned and changes
 *    only through user actions we already invalidate on.
 */
export const queryClient = new QueryClient({
    defaultOptions: {
        queries: {
            staleTime: 60_000,
            gcTime: 5 * 60_000,
            retry: 2,
            refetchOnWindowFocus: false,
            refetchOnReconnect: true,
        },
        mutations: {
            retry: 0,
        },
    },
});
