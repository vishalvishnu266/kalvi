// One-off bundler that produces `components/vendor/lit-all.min.js` from
// the installed `lit` npm package. Re-run whenever you bump Lit.
//
//   node scripts/vendor-lit.mjs
//
// The bundle is a plain ES module — no bare specifiers, no import maps,
// no top-level await. Any modern browser can `import` it directly.

import { build } from 'esbuild';
import { writeFileSync, mkdirSync } from 'node:fs';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outFile   = resolve(__dirname, '../components/vendor/lit-all.min.js');

mkdirSync(dirname(outFile), { recursive: true });

// Re-export exactly what `LitBaseElement` uses. Add more names here if
// you start using additional Lit exports (repeat, when, styleMap, …).
const entry = `
export { LitElement, html, css, nothing } from 'lit';
`;

const tmpEntry = resolve(__dirname, '_entry.mjs');
writeFileSync(tmpEntry, entry, 'utf8');

await build({
  entryPoints: [tmpEntry],
  outfile:     outFile,
  bundle:      true,
  format:      'esm',
  minify:      true,
  sourcemap:   false,
  target:      ['es2020'],
  logLevel:    'info',
  absWorkingDir: resolve(__dirname, '..'),   // so bare-specifiers resolve against lit-components/node_modules
});

// Clean up the entry stub.
import { unlinkSync } from 'node:fs';
unlinkSync(tmpEntry);

console.log(`\nOK - Wrote ${outFile}`);
