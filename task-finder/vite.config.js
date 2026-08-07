import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const __dirname = dirname(fileURLToPath(import.meta.url))
const pkg = JSON.parse(readFileSync(resolve(__dirname, 'package.json'), 'utf-8'))

// Auto-bump the patch version on every `vite build` so every build produces
// a NEW bundle version that the OTA server can serve.
// We combine package.json version + a UTC timestamp so it is monotonically
// increasing and human-readable, e.g. "0.0.0+20260807130612".
function buildVersion() {
    const now = new Date()
    const pad = (n) => String(n).padStart(2, '0')
    const stamp = `${now.getUTCFullYear()}${pad(now.getUTCMonth() + 1)}${pad(now.getUTCDate())}` +
                  `${pad(now.getUTCHours())}${pad(now.getUTCMinutes())}${pad(now.getUTCSeconds())}`
    return `${pkg.version}+${stamp}`
}

const APP_VERSION = process.env.APP_VERSION || buildVersion()

export default defineConfig({
    plugins: [vue()],
    define: {
        __APP_VERSION__: JSON.stringify(APP_VERSION),
    },
})

export { APP_VERSION }
