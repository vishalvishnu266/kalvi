# UI Preview

Static, dependency-free copies of every server-rendered template in
`../../templates/`, rewritten to use ONLY the semantic component classes
defined in `../../styles/app.css`.

## Goal

Give designers / developers a place to iterate on the visual language
without running the Rust app, and to prove the semantic class vocabulary
(`.btn`, `.card`, `.input`, `.nav-link`, `.menu-tile`, …) is expressive
enough to render every screen.

## How to preview

Open `index.html` in your browser (double-click or drag it into a tab).
Every screen loads its CSS from `assets/app.build.css` via a relative
path, so no dev server is required.

If you want to serve the folder locally instead (nicer for hot reload
via a browser extension):

```powershell
# from repo root
npx --yes serve docs/ui-preview -l 5500
# then visit http://localhost:5500
```

## Files

```
docs/ui-preview/
├── index.html              ← landing index: links to every screen
├── landing.html            ← marketing page
├── login.html              ← staff login
├── dashboard.html          ← authenticated app shell + launcher grid
├── admin/
│   ├── tenants.html        ← admin control plane
│   └── new_tenant.html
├── portal/
│   ├── index.html
│   ├── home.html
│   ├── login.html
│   └── register.html
├── assets/
│   ├── app.build.css       ← copy of ../../static/app.build.css
│   ├── app.js              ← copy of ../../static/app.js
│   └── boot.js             ← shared theme + icon boot script
└── README.md
```

Any Askama-specific syntax (`{% %}`, `{% if let %}`) has been replaced
with hard-coded sample data so pages open as plain HTML.

## Keeping in sync

Whenever you change `styles/app.css` or run `npm run build:css` at the
repo root, refresh this folder's copy of the CSS:

```powershell
Copy-Item static/app.build.css docs/ui-preview/assets/app.build.css -Force
```

## Design principles enforced here

1. **No raw utility strings > 3 tokens.** If you find yourself typing
   `w-full pl-9 pr-3 py-2.5 rounded-lg …`, add a semantic class to
   `styles/app.css` and use that here.
2. **No inline `<style>`.** Everything is composed from classes.
3. **Zero JS frameworks.** Only Lucide (icons) and the tiny `boot.js`.
