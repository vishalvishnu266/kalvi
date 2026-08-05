// -----------------------------------------------------------------------------
// <ui-fragment target="…" action="replace|append|prepend|remove|update">…
//
// A self-applying envelope. When the shell inserts a <ui-fragment> into
// the DOM (typically as a child of the initial <body> or as the parsed
// result of an SSE event), it upgrades and applies itself to the target
// island, then removes itself from the DOM.
//
// Kept as a plain HTMLElement (no Lit dep) so it can run before extras/
// lazy chunks have loaded — this is part of the framework runtime.
// -----------------------------------------------------------------------------

import { applyFragment } from '../framework/shell.js';

class UiFragment extends HTMLElement {
  connectedCallback() {
    // Skip fragments that were parsed into a <template>.content — they
    // aren't actually in the live document, and the shell's applyAll()
    // path will handle them explicitly.
    if (!this.isConnected) return;

    // A fragment inside another fragment (nested applies during SSE
    // rehydration) is a no-op — outer apply already moved the children.
    if (this.parentElement && this.parentElement.tagName === 'UI-FRAGMENT') return;

    applyFragment(this);
    // Remove the envelope itself once its payload has landed.
    this.remove();
  }
}

if (!customElements.get('ui-fragment')) {
  customElements.define('ui-fragment', UiFragment);
}
