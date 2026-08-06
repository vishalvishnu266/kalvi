// Entry file — import once per page.
//
//   <script type="module" src="/lit-components/components/index.js"></script>
//
// After the non-primitive purge, the whole library is one small critical
// bundle (see core.js). No extras tier, no lazy loader, no FOUCE gate —
// primitives + layout upgrade in a single tick and there's nothing to
// defer.

import './core.js';
