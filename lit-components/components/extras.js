// Deferred bundle — common but not first-paint.
//
// Imported by index.js AFTER the critical `core.js` bundle has resolved.
// These modules are cheap enough to bundle together and land in one
// network request, so we don't fragment them into per-tag chunks.
//
// If a component here starts causing measurable layout shift, promote it
// to `core.js`. If it becomes rarely-used, demote it to `heavy.js`.

import './ui-input.js';
import './ui-select.js';
import './ui-checkbox.js';
import './ui-radio.js';
import './ui-switch.js';
import './ui-form.js';
import './ui-list-item.js';
import './ui-tab-bar.js';
import './ui-segmented.js';
import './ui-tooltip.js';
import './ui-avatar-group.js';
import './ui-empty-state.js';
import './ui-pagination.js';
import './ui-modal.js';
import './ui-datepicker.js';
import './ui-daterange.js';
