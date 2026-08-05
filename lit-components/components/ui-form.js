import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-form action="/students" method="post">
 *   <ui-input name="fullName" label="Full name" required></ui-input>
 *   <ui-input name="email" type="email" label="Email" required></ui-input>
 *   <ui-checkbox name="agree" required>I agree to the terms</ui-checkbox>
 *
 *   <div slot="actions">
 *     <ui-button type="submit">Save</ui-button>
 *     <ui-button variant="secondary" type="reset">Reset</ui-button>
 *   </div>
 * </ui-form>
 *
 * Serialises every named child ui-input / ui-select / ui-checkbox / ui-radio-group /
 * ui-switch / ui-datepicker / ui-daterange into a plain { name: value } object
 * and emits a bubbling `ui-submit` event with { values, valid }.
 *
 * If any field is marked [required] but empty (or [invalid]), submission is
 * blocked and each offending field gets [invalid] applied; a `ui-invalid`
 * event is emitted with { fields: [{ name, reason }] }.
 *
 * The DSL surface is HTML-only, so a Rust template can compose forms the
 * same way.
 */
const FIELD_TAGS = [
  'ui-input', 'ui-select', 'ui-checkbox', 'ui-switch',
  'ui-radio-group', 'ui-datepicker', 'ui-daterange',
];

class UIForm extends LitBaseElement {
  static properties = {
    action: { type: String, reflect: true },
    method: { type: String, reflect: true },   // "get" | "post"
    novalidate: { type: Boolean, reflect: true },
    /**
     * Pin the actions row (Save / Cancel / Delete etc.) so it stays
     * visible while the user scrolls a long form. Values:
     *   ""        - not sticky (default)
     *   "bottom"  - pinned to the bottom of the viewport
     *   "top"     - pinned to the top of the viewport (renders the
     *               actions row above the fields as well)
     *
     * Uses CSS position:sticky so it costs nothing until the row would
     * otherwise scroll off-screen - no scroll listeners, no layout
     * thrash. When pinned it grows a shadow so it visually detaches
     * from the fields underneath.
     */
    sticky: { type: String, reflect: true },
    /**
     * Optional CSS length (e.g. "56px", "var(--app-topbar-h)") used as
     * top when sticky="top" or bottom when sticky="bottom". Defaults to
     * 0px. Set this to your app-shell topbar height so the sticky bar
     * sits just below it instead of overlapping.
     */
    stickyOffset: { type: String, reflect: true, attribute: 'sticky-offset' },
  };

  static styles = css`
    :host { display: block; }
    form { display: flex; flex-direction: column; gap: var(--space-4); }
    /* The banner slot sits above the fields with its own gap so the
       form-level error / success surface reads as a distinct band, not
       "another field". */
    ::slotted([slot="banner"]) { display: block; margin-bottom: var(--space-2); }
    .actions {
      display: flex; gap: var(--space-2); justify-content: flex-end;
      padding-top: var(--space-2); border-top: 1px solid var(--color-border);
      margin-top: var(--space-2);
    }
    :host([inline]) form { flex-direction: row; align-items: end; flex-wrap: wrap; }

    /* ------------------------------------------------------------------
       STICKY ACTION BAR
       ------------------------------------------------------------------
       position:sticky pins an element relative to its nearest SCROLLING
       ancestor, but only while its own containing block is on screen.
       For the actions row to actually stick we need:
         (a) the row's containing block (here: the <form>) to be TALLER
             than the row itself, otherwise there is no scroll range to
             "stick" through, and
         (b) no ancestor with overflow:hidden|auto|scroll between the
             row and the scroll container — such an ancestor becomes the
             new scroll container and clips the sticky.
       Requirement (a) is naturally satisfied because <form> contains
       both the fields slot AND the actions row. Requirement (b) is the
       responsibility of the page layout — we emit a dev-console warning
       when we detect a trap ancestor (see connectedCallback).
       ------------------------------------------------------------------ */
    :host([sticky="bottom"]) form {
      /* Give the sticky row a positioning context that spans the whole
         form height (fields + actions). Without this the row's box
         collapses to its own height and sticky has nowhere to travel. */
      min-height: 100%;
    }
    :host([sticky="bottom"]) .actions {
      position: sticky;
      bottom: calc(var(--sticky-offset, 0px) + env(safe-area-inset-bottom, 0px));
      z-index: 10;
      background: var(--color-surface);
      padding: var(--space-3) var(--space-4);
      margin: var(--space-2) calc(var(--space-4) * -1) 0;
      border-top: 1px solid var(--color-border);
      box-shadow: 0 -4px 12px -6px rgba(0,0,0,.15);
      border-radius: 0 0 var(--radius-md) var(--radius-md);
    }
    :host([sticky="top"]) .actions {
      position: sticky;
      top: calc(var(--sticky-offset, 0px) + env(safe-area-inset-top, 0px));
      z-index: 10;
      background: var(--color-surface);
      padding: var(--space-3) var(--space-4);
      margin: 0 calc(var(--space-4) * -1) var(--space-2);
      border-top: 0;
      border-bottom: 1px solid var(--color-border);
      box-shadow: 0 4px 12px -6px rgba(0,0,0,.15);
      border-radius: var(--radius-md) var(--radius-md) 0 0;
      /* Reorder inside the flex column so the actions render BEFORE
         the fields visually. */
      order: -1;
    }
    :host([sticky]) .actions { border-top-color: transparent; }

    /* Mobile polish: on narrow viewports the sticky bar stretches
       edge-to-edge and the primary button grows to be thumb-friendly. */
    @media (max-width: 640px) {
      :host([sticky]) .actions {
        justify-content: space-between;
        gap: var(--space-3);
      }
      :host([sticky]) .actions ::slotted(ui-button) {
        flex: 1;
      }
    }
  `;

  constructor() {
    super();
    this.action = '';
    this.method = 'post';
    this.novalidate = false;
    this.sticky = '';
    this.stickyOffset = '';
  }

  /**
   * Enum-like whitelist for the `sticky` attribute. Anything not in
   * this set is ignored (and dev-warned) so a typo like sticky="botton"
   * doesn't silently break the layout — the row simply renders
   * un-pinned as if the attribute were absent.
   *
   * The Rust `StickyActions` enum guarantees only these values at
   * compile time; this guard just defends the raw-HTML path.
   */
  static STICKY_VALUES = Object.freeze(['top', 'bottom']);

  updated(changed) {
    if (changed.has('stickyOffset')) {
      if (this.stickyOffset) {
        this.style.setProperty('--sticky-offset', this.stickyOffset);
      } else {
        this.style.removeProperty('--sticky-offset');
      }
    }
    if (changed.has('sticky')) {
      const v = (this.sticky || '').toLowerCase();
      if (v && !UIForm.STICKY_VALUES.includes(v)) {
        // eslint-disable-next-line no-console
        console.warn(
          `[ui-form] Ignoring unknown sticky="${this.sticky}". ` +
          `Expected one of: ${UIForm.STICKY_VALUES.join(', ')}.`
        );
        // Remove the attribute so CSS selectors don't half-match.
        this.removeAttribute('sticky');
        this.sticky = '';
      } else if (v !== this.sticky) {
        // Normalise casing so CSS selectors always match.
        this.sticky = v;
      }
    }
  }

  connectedCallback() {
    super.connectedCallback();
    // Intercept clicks on any [type="submit"] / [type="reset"] child.
    this.addEventListener('click', this.#onClick);
    this.addEventListener('keydown', this.#onKey);
    // Detect scroll/overflow ancestors that would trap the sticky row.
    // Deferred so the element is actually laid out first.
    if (this.sticky) {
      requestAnimationFrame(() => this.#auditStickyAncestors());
    }
  }

  /**
   * `position: sticky` pins against the nearest scrolling ancestor. If
   * some intermediate ancestor has `overflow: hidden|auto|scroll` it
   * becomes the sticky's scroll container and the row will "stick"
   * inside that (usually short) box instead of the viewport — which
   * looks exactly like "not sticking".
   *
   * This walker crosses shadow-root boundaries and warns the developer
   * with the exact offending element so they can add
   * `overflow: visible` (or restructure the layout) to fix it.
   */
  #auditStickyAncestors() {
    const traps = ['auto', 'scroll', 'hidden', 'clip'];
    let node = this.parentNode;
    let hopped = false;
    while (node) {
      if (node.nodeType === Node.ELEMENT_NODE) {
        const cs = getComputedStyle(node);
        // The <html> and <body> are the natural scroll containers —
        // sticking against them is exactly what we want.
        const isRoot = node === document.documentElement || node === document.body;
        if (!isRoot) {
          const yTrap = traps.includes(cs.overflowY);
          const xTrap = traps.includes(cs.overflowX);
          if (yTrap || xTrap) {
            // eslint-disable-next-line no-console
            console.warn(
              `[ui-form] sticky="${this.sticky}" may not work because ` +
              `an ancestor has overflow:${yTrap ? cs.overflowY : cs.overflowX}. ` +
              `Set that element's overflow to "visible" or move <ui-form> ` +
              `outside it. Offender:`, node
            );
            return;
          }
        }
      }
      // Cross shadow-root boundaries.
      const parent = node.parentNode || node.host;
      if (!parent && !hopped && node.getRootNode && node.getRootNode() !== document) {
        hopped = true;
        node = node.getRootNode().host || null;
      } else {
        node = parent;
      }
    }
  }

  #onClick = (e) => {
    const btn = e.target.closest('[type="submit"]');
    if (btn && this.contains(btn)) { e.preventDefault(); this.submit(); return; }
    const rst = e.target.closest('[type="reset"]');
    if (rst && this.contains(rst)) { e.preventDefault(); this.reset(); }
  };
  #onKey = (e) => {
    if (e.key === 'Enter' && e.target.matches('ui-input') && !e.target.matches('[type="textarea"]')) {
      e.preventDefault();
      this.submit();
    }
  };

  fields() {
    return [...this.querySelectorAll(FIELD_TAGS.join(','))].filter(f => f.getAttribute('name'));
  }

  values() {
    const out = {};
    for (const f of this.fields()) {
      const name = f.getAttribute('name');
      if (!name) continue;
      const tag = f.tagName.toLowerCase();
      if (tag === 'ui-checkbox' || tag === 'ui-switch') {
        out[name] = !!f.checked;
      } else if (tag === 'ui-daterange') {
        out[name] = { from: f.from || '', to: f.to || '' };
      } else {
        out[name] = f.value ?? '';
      }
    }
    return out;
  }

  validate() {
    const bad = [];
    for (const f of this.fields()) {
      const name = f.getAttribute('name');
      const required = f.hasAttribute('required');
      const tag = f.tagName.toLowerCase();
      let empty = false;
      if (tag === 'ui-checkbox' || tag === 'ui-switch') empty = !f.checked;
      else if (tag === 'ui-daterange') empty = !(f.from && f.to);
      else empty = !((f.value ?? '').toString().trim());

      f.removeAttribute('invalid');
      if (required && empty) {
        bad.push({ name, reason: 'required' });
        f.setAttribute('invalid', '');
      }
    }
    return bad;
  }

  reset() {
    for (const f of this.fields()) {
      const tag = f.tagName.toLowerCase();
      if (tag === 'ui-checkbox' || tag === 'ui-switch') f.checked = false;
      else if (tag === 'ui-daterange') { f.from = ''; f.to = ''; }
      else f.value = '';
      f.removeAttribute('invalid');
    }
    this.emit('ui-reset');
  }

  submit() {
    const bad = this.novalidate ? [] : this.validate();
    const values = this.values();
    if (bad.length) {
      this.emit('ui-invalid', { fields: bad, values });
      const first = this.fields().find(f => f.hasAttribute('invalid'));
      first?.focus?.();
      return false;
    }
    // Fire the observable event first so consumer code can react / log.
    // (LitBaseElement.emit dispatches a non-cancelable CustomEvent; the
    // real submission below is still performed unconditionally when an
    // `action` is set — that's the documented DSL contract.)
    this.emit('ui-submit', { values, valid: true, action: this.action, method: this.method });

    // Perform an actual HTTP submission if an `action` was provided. We
    // synthesise a hidden native <form> so the browser handles the request
    // the same way a plain HTML form would; the shell's document-level
    // submit interceptor then swaps in whatever the server returns.
    if (this.action) {
      const nativeForm = document.createElement('form');
      nativeForm.action = this.action;
      nativeForm.method = (this.method || 'post').toLowerCase();
      nativeForm.style.display = 'none';

      const appendField = (name, val) => {
        const el = document.createElement('input');
        el.type = 'hidden'; el.name = name; el.value = val;
        nativeForm.appendChild(el);
      };
      for (const [name, val] of Object.entries(values)) {
        if (val === null || val === undefined) continue;
        if (typeof val === 'boolean') {
          // HTML convention: unchecked checkboxes send nothing.
          if (val) appendField(name, 'on');
        } else if (typeof val === 'object') {
          // e.g. ui-daterange → { from, to } → name_from / name_to
          for (const [k, v] of Object.entries(val)) {
            if (v !== null && v !== undefined && v !== '') appendField(`${name}_${k}`, v);
          }
        } else {
          appendField(name, String(val));
        }
      }
      document.body.appendChild(nativeForm);
      nativeForm.submit();
    }
    return true;
  }

  render() {
    return html`
      <form novalidate>
        <slot name="banner"></slot>
        <slot></slot>
        <div class="actions"><slot name="actions"></slot></div>
      </form>`;
  }
}
customElements.define('ui-form', UIForm);
