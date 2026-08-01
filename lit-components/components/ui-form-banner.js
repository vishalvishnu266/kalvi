import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-form-banner tone="danger" title="Please fix 2 errors" dismissible>
 *   <ul slot="errors">
 *     <li><a data-field="email" href="#">Guardian email is invalid</a></li>
 *   </ul>
 * </ui-form-banner>
 *
 * The form-level error / info / success banner. Used INSIDE a form (via
 * <ui-form>'s `banner` slot) or standalone above any block of fields.
 *
 * Clicking an errors-summary link scrolls to and focuses the field with
 * that `name` attribute — critical for keyboard/screen-reader users.
 */
class UIFormBanner extends LitBaseElement {
  static properties = {
    tone:          { type: String,  reflect: true },   // info|success|warning|danger|brand|neutral
    title:         { type: String,  reflect: true },
    message:       { type: String,  reflect: true },
    dismissible:   { type: Boolean, reflect: true },
    // When true, the component renders <slot name="section"> instead of the
    // single title/message/errors payload. Sections have their OWN tones
    // (via data-tone on the slotted <div>) so danger + warning can coexist.
    multiSection:  { type: Boolean, reflect: true, attribute: 'multi-section' },
  };

  static styles = css`
    :host { display: block; box-sizing: border-box; width: 100%; margin-bottom: var(--space-3); }
    .box {
      display: flex; gap: var(--space-3); align-items: flex-start;
      padding: var(--space-3) var(--space-4);
      border: 1px solid var(--border, var(--color-border));
      background: var(--bg, var(--color-surface));
      color: var(--fg, var(--color-text));
      border-radius: var(--radius-md);
    }
    .body { flex: 1; min-width: 0; }
    .title { font-weight: var(--fw-semibold); margin: 0 0 4px; font-size: var(--fs-sm); }
    .msg   { margin: 0; font-size: var(--fs-sm); color: var(--fg-muted, var(--color-text-muted)); }
    ::slotted(ul) {
      margin: 8px 0 0; padding: 0 0 0 20px; font-size: var(--fs-sm);
    }
    ::slotted(ul) li { margin: 2px 0; }

    /* ── Multi-section mode ──
       When multi-section is set, the OUTER .box uses a neutral surface,
       and each slotted <div slot="section" data-tone="..."> becomes its
       own coloured tinted block stacked vertically. Sections are targeted
       via ::slotted() with attribute selectors so we don't need any Shadow
       DOM inside them — they stay in the light DOM for a11y. */
    :host([multi-section]) .box { padding: var(--space-3); gap: 0; }
    :host([multi-section]) .body > .sections { display: flex; flex-direction: column; gap: var(--space-2); }
    ::slotted(div[slot="section"]) {
      padding: var(--space-3) var(--space-4);
      border: 1px solid var(--sec-border, var(--color-border));
      background:  var(--sec-bg,     var(--color-surface));
      color:       var(--sec-fg,     var(--color-text));
      border-radius: var(--radius-md);
      font-size: var(--fs-sm);
    }
    ::slotted(div[slot="section"][data-tone="danger"])  { --sec-bg: var(--color-danger-soft);
                                                          --sec-border: var(--color-danger);
                                                          --sec-fg: var(--color-danger-strong); }
    ::slotted(div[slot="section"][data-tone="warning"]) { --sec-bg: var(--color-warning-soft);
                                                          --sec-border: var(--color-warning);
                                                          --sec-fg: var(--color-warning-strong); }
    ::slotted(div[slot="section"][data-tone="success"]) { --sec-bg: var(--color-success-soft);
                                                          --sec-border: var(--color-success);
                                                          --sec-fg: var(--color-success-strong); }
    ::slotted(div[slot="section"][data-tone="info"])    { --sec-bg: var(--color-info-soft);
                                                          --sec-border: var(--color-info);
                                                          --sec-fg: var(--color-info-strong); }

    button.close {
      background: transparent; border: 0; cursor: pointer; padding: 4px 8px;
      color: var(--fg-muted, var(--color-text-muted)); font-size: 18px; line-height: 1;
      border-radius: var(--radius-sm);
    }
    button.close:hover { background: var(--color-surface-hover, rgba(0,0,0,.05)); }

    /* Per-tone tokens — reuse the design system's semantic colours. */
    :host([tone="danger"])  .box { --bg: var(--color-danger-soft,  #fee2e2);
                                    --border: var(--color-danger, #dc2626);
                                    --fg: var(--color-danger-strong, #991b1b);
                                    --fg-muted: var(--color-danger, #991b1b); }
    :host([tone="warning"]) .box { --bg: var(--color-warning-soft, #fef3c7);
                                    --border: var(--color-warning, #f59e0b);
                                    --fg: var(--color-warning-strong, #78350f);
                                    --fg-muted: var(--color-warning, #78350f); }
    :host([tone="success"]) .box { --bg: var(--color-success-soft, #dcfce7);
                                    --border: var(--color-success, #16a34a);
                                    --fg: var(--color-success-strong, #14532d);
                                    --fg-muted: var(--color-success, #14532d); }
    :host([tone="info"])    .box { --bg: var(--color-info-soft, #dbeafe);
                                    --border: var(--color-info, #2563eb);
                                    --fg: var(--color-info-strong, #1e3a8a);
                                    --fg-muted: var(--color-info, #1e3a8a); }
  `;

  constructor() {
    super();
    this.tone = 'info';
    this.title = '';
    this.message = '';
    this.dismissible = false;
  }

  firstUpdated() {
    // Intercept clicks on error-summary <a data-field="..."> to scroll+focus.
    this.addEventListener('click', (e) => {
      const a = e.target.closest('a[data-field]');
      if (!a) return;
      e.preventDefault();
      const name = a.getAttribute('data-field');
      // Look in the OWNER document (light DOM) since form fields live outside this shadow root.
      const root = this.getRootNode();
      const el = root.querySelector
        ? root.querySelector(`[name="${name}"]`) || document.querySelector(`[name="${name}"]`)
        : document.querySelector(`[name="${name}"]`);
      if (el) {
        el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        // Try to focus the inner control if the wrapper doesn't take focus.
        if (typeof el.focus === 'function') el.focus();
        else if (el.shadowRoot) el.shadowRoot.querySelector('input,textarea,select')?.focus();
      }
    });
  }

  #dismiss = () => { this.remove(); };

  render() {
    // Danger anywhere → assertive; otherwise polite. For multi-section we
    // also let the outer tone drive this (Rust already sets tone="danger"
    // when any section is a danger, so this works out).
    const isDanger = this.tone === 'danger';
    return html`
      <div class="box" role=${isDanger ? 'alert' : 'status'} aria-live=${isDanger ? 'assertive' : 'polite'}>
        <div class="body">
          ${this.multiSection
            ? html`
                <div class="sections">
                  <slot name="section"></slot>
                </div>
              `
            : html`
                ${this.title   ? html`<p class="title">${this.title}</p>`   : nothing}
                ${this.message ? html`<p class="msg">${this.message}</p>`   : nothing}
                <slot name="errors"></slot>
              `
          }
        </div>
        ${this.dismissible
          ? html`<button class="close" @click=${this.#dismiss} aria-label="Dismiss">×</button>`
          : nothing}
      </div>
    `;
  }
}
customElements.define('ui-form-banner', UIFormBanner);
