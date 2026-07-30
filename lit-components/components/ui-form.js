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
  'ui-radio-group', 'ui-datepicker', 'ui-daterange', 'ui-combobox',
];

class UIForm extends LitBaseElement {
  static properties = {
    action: { type: String, reflect: true },
    method: { type: String, reflect: true },   // "get" | "post"
    novalidate: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: block; }
    form { display: flex; flex-direction: column; gap: var(--space-4); }
    .actions {
      display: flex; gap: var(--space-2); justify-content: flex-end;
      padding-top: var(--space-2); border-top: 1px solid var(--color-border);
      margin-top: var(--space-2);
    }
    :host([inline]) form { flex-direction: row; align-items: end; flex-wrap: wrap; }
  `;

  constructor() {
    super();
    this.action = '';
    this.method = 'post';
    this.novalidate = false;
  }

  connectedCallback() {
    super.connectedCallback();
    // Intercept clicks on any [type="submit"] / [type="reset"] child.
    this.addEventListener('click', this.#onClick);
    this.addEventListener('keydown', this.#onKey);
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
    this.emit('ui-submit', { values, valid: true, action: this.action, method: this.method });
    return true;
  }

  render() {
    return html`
      <form novalidate>
        <slot></slot>
        <div class="actions"><slot name="actions"></slot></div>
      </form>`;
  }
}
customElements.define('ui-form', UIForm);
