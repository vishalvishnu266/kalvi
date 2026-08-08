# DailyGig – React TS variant (TaskFinderReact)

A **mobile-only React 18 + TypeScript + Capacitor** app — the React sibling
of `../task-finder` (which uses Vue 3). Both share the same functionality:
GPS, camera, SQLite, device/network info, deep links, and OTA hot updates
served from a small Axum server.

The point of this project is to be an **apples-to-apples comparison** with
the Vue version so you can judge DX, bundle size, runtime performance,
and hot-update reliability.

---

## Feature parity

| Feature                    | Vue app       | React app (this)        |
| -------------------------- | ------------- | ----------------------- |
| Framework                  | Vue 3 + Vite  | React 18 + Vite         |
| Router                     | `vue-router`  | `react-router-dom` v6   |
| Language                   | `<script setup lang="ts">` | `.tsx` |
| Local state                | `ref`         | `useState`              |
| Global state               | module `ref`  | **Zustand** (`useOtaStore`) |
| Server state / caching     | ad-hoc `fetch` | **TanStack Query** (`QueryClientProvider`) |
| GPS / Camera / SQLite      | composables   | hooks in `src/hooks/`   |
| Notifications              | ⏳ (stub)     | ✅ **`useNotifications`** + Settings button |
| OTA client                 | `@capgo/capacitor-updater` | same                 |
| OTA server                 | Axum (port 3000) | Axum (port **3001**) |
| App id                     | `com.yourcompany.taskfinder` | `com.yourcompany.taskfinderreact` |

### Why Zustand + TanStack Query

Long-term maintainability was the driving reason:

- **Zustand** replaces the ad-hoc module-level `ref` pattern the Vue app
  uses in `useOta.ts`. It gives us:
  - Proper React re-renders via selectors (no `useSyncExternalStore`
    boilerplate)
  - A single import to reach the store from anywhere (tests included)
  - Trivial DevTools integration if we add it later
  - Zero context providers — no perf hit from cascading re-renders
- **TanStack Query** already powers the OTA flow (see below) and is
  ready for the "HTTP client with auth interceptor" milestone. When we
  start hitting the backend for tasks / rides / users we get:
  - Automatic dedupe, caching, and background refresh
  - Retry + exponential backoff on flaky mobile networks
  - Cache invalidation semantics that map cleanly to mutations
  - `useMutation` for optimistic updates on task acceptance flows

The shared client lives in `src/lib/queryClient.ts` with mobile-friendly
defaults (no window-focus refetch, longer stale time, 2 retries).

### OTA flow, now query-driven

`src/hooks/useOta.ts` was rebuilt on top of TanStack Query. The layers:

| Concern                          | Owner                                          |
| -------------------------------- | ---------------------------------------------- |
| Poll `/api/check-update` every 15s | `useCheckUpdateQuery()` — `useQuery` with `refetchInterval` |
| Download → set → reload native   | `useApplyUpdateMutation()` — `useMutation`     |
| App-wide orchestration           | `useAutoUpdater()` — mounted once in `<App />` |
| Foreground re-check              | `CapApp.appStateChange` → `queryClient.invalidateQueries` |
| Cross-component UI state         | Zustand (`isApplying`, `statusMessage`)        |
| Manual "Check for updates"       | `useOta().checkForUpdate()` → `invalidateQueries` + `refetch` |

Why this split is nicer than the hand-rolled `setInterval` + module refs
we had before:

- **Dedupe for free** — if two components mount `useCheckUpdateQuery()`
  they share the same in-flight request and cached result.
- **Automatic pause when hidden** — no wasted requests when the WebView
  is backgrounded (`refetchIntervalInBackground: false`).
- **Retry with backoff** — no need to babysit failed polls on flaky
  Wi-Fi.
- **Testability** — swap the `queryFn` in tests without stubbing
  globals; the mutation can be asserted independently of the poll.
- **Observability** — one line to add `@tanstack/react-query-devtools`
  and you can watch every check/apply from a floating panel.

Running the two OTA servers on different ports means you can build and
compare both apps against a live update endpoint simultaneously.

---

## Layout

```
task-finder-react/
├── index.html
├── vite.config.ts
├── tsconfig.json
├── capacitor.config.json
├── package.json
├── public/
│   └── favicon.svg
├── scripts/
│   └── build-bundle.mjs        # builds dist/, zips it, updates latest.json
├── server/                     # Axum OTA server (port 3001)
│   ├── Cargo.toml
│   └── src/main.rs
└── src/
    ├── main.tsx                # createRoot + <HashRouter>
    ├── App.tsx                 # <Routes> + <TabBar> + <UpdateOverlay>
    ├── style.css               # all styles (was scoped-per-SFC in Vue)
    ├── env.d.ts                # __APP_VERSION__, __OTA_HOST__, __OTA_PORT__
    ├── components/
    │   ├── PageHeader.tsx
    │   ├── TabBar.tsx
    │   └── UpdateOverlay.tsx
    ├── lib/
    │   └── queryClient.ts      # TanStack Query singleton (mobile-tuned)
    ├── hooks/                  # 1:1 port of Vue composables (plus a couple extras)
    │   ├── useCamera.ts
    │   ├── useDeepLinks.ts
    │   ├── useDevice.ts
    │   ├── useLocation.ts
    │   ├── useNative.ts
    │   ├── useNotifications.ts # native LocalNotifications + web fallback
    │   ├── useOta.ts           # useQuery + useMutation + Zustand UI slice
    │   ├── useSqlite.ts
    │   └── useStorage.ts
    └── pages/
        ├── DevicePage.tsx
        ├── LocationPage.tsx
        ├── RideDetailPage.tsx
        ├── SandboxPage.tsx
        ├── SettingsPage.tsx
        └── TaskDetailPage.tsx
```

---

## Quick start

```zsh
cd task-finder-react
npm install
npm run dev             # http://localhost:5174   (Vue app is on 5173)
```

### Physical phone / Android emulator

```zsh
npm run android         # build → cap sync android → cap run android
```

### OTA workflow

```zsh
# terminal 1 — start the OTA server (port 3001, so it can co-exist with the Vue 3000)
cd task-finder-react/server
cargo run

# terminal 2 — build a new bundle
cd task-finder-react
OTA_HOST=192.168.0.4 npm run bundle:ota
```

The app polls `http://<OTA_HOST>:3001/api/check-update?version=…` every
15 seconds and swaps the bundle in place using
`@capgo/capacitor-updater`.

---

## Comparing the two apps

| Dimension            | Where to look                                                                    |
| -------------------- | -------------------------------------------------------------------------------- |
| Bundle size          | Run `npm run build` in each folder, compare `dist/assets/*.js` sizes             |
| Cold-start time      | `chrome://inspect` timeline against the Capacitor WebView                        |
| Reactive DX          | `src/hooks/useOta.ts` vs `../task-finder/src/composables/useOta.ts`              |
| Template ergonomics  | `.vue` `<template>` vs `.tsx` — see `SandboxPage.tsx` for the busiest page       |
| Router                | `HashRouter` + `<Routes>` vs `createRouter` + lazy dynamic imports              |
| Global state         | Vue: module-level `ref`, React: module-level object + `useSyncExternalStore`     |

The Vue app's per-SFC scoped CSS was merged into a single
`src/style.css` here — React has no built-in equivalent, and adding CSS
Modules or a runtime like styled-components would skew the comparison.
Keeping vanilla CSS makes the head-to-head fairer.

---

## System notifications (with sound)

The **Settings** tab has a real "Send test notification 🔔" button:

- **On device** — schedules a `@capacitor/local-notifications` fire ~1 s
  in the future with `sound: 'default'`, so Android and iOS play their
  standard alert tone. This is a genuine OS notification: it shows up
  even if the app is backgrounded.
- **On web** (during `npm run dev`) — uses the browser Notification API
  for the popup and synthesizes a short two-note chime via WebAudio, so
  you get audible feedback without shipping any binary asset.

### Android permission (Android 13+)

`@capacitor/local-notifications` requires runtime consent on Android 13
and above. Capacitor auto-merges the required manifest entries when you
run `npx cap sync android`, but if you have manual edits, make sure the
following is present in `android/app/src/main/AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.POST_NOTIFICATIONS"/>
```

### iOS

No extra setup needed for the default alert sound. If you want a custom
sound file, drop `myalert.caf` into the Xcode project and pass
`sound: 'myalert.caf'` in `useNotifications.ts`.

---

## Roadmap items now unlocked by the new libs

- **HTTP client with auth interceptor** — wrap `fetch` in a small
  `src/lib/api.ts` module and expose typed `useQuery` / `useMutation`
  hooks per resource (`useTasks`, `useRide(id)`, `useAcceptTask()`).
- **Optimistic UI** — TanStack Query's `onMutate` + `queryClient.setQueryData`
  pattern makes accept-task-then-navigate feel instant.
- **Session store** — Zustand slice for the logged-in user, tokens,
  active shift, etc. Persisted with the existing `useStorage` hook.
