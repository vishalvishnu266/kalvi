import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-input label="Full name" placeholder="e.g. Aarav" name="fullName" required></ui-input>
 * <ui-input type="password" label="Password"></ui-input>
 * <ui-input type="textarea" label="Notes" hint="Max 500 chars"></ui-input>
 *
 * DSL surface:
 *   - label       : string
 *   - type        : "text" (default) | "password" | "email" | "number" | "textarea"
 *   - name        : form field name
 *   - value       : current value
 *   - placeholder : placeholder text
 *   - hint        : helper / error text below the field
 *   - required    : boolean
 *   - invalid     : boolean, apply the error styling
 *
 * Emits:
 *   - "ui-input"  on every keystroke  ({ value, name })
 *   - "ui-change" on blur / commit    ({ value, name })
 */
class UIInput extends LitBaseElement {
  static properties = {
    label:       { type: String, reflect: true },
    type:        { type: String, reflect: true },
    name:        { type: String, reflect: true },
    value:       { type: String },
    placeholder: { type: String, reflect: true },
    hint:        { type: String, reflect: true },
    required:    { type: Boolean, reflect: true },
    invalid:     { type: Boolean, reflect: true },
  };

  static styles = css`
    :host { display: block; box-sizing: border-box; width: 100%; }
    *, *::before, *::after { box-sizing: border-box; }
    label {
      display: block;
      font-size: var(--fs-xs);
      color: var(--color-text-muted);
      margin-bottom: 6px;
      font-weight: var(--fw-medium);
      letter-spacing: .01em;
    }
    .wrap { position: relative; display: block; width: 100%; }
    input, textarea {
      display: block;
      width: 100%;
      max-width: 100%;
      min-width: 0;
      font-family: inherit;
      font-size: var(--fs-sm);
      color: var(--color-text);
      background: var(--color-surface);
      border: 1px solid var(--color-border-strong);
      border-radius: var(--radius-md);
      padding: 0 var(--space-4);
      height: 40px;
      outline: none;
      transition: border-color var(--dur-fast) var(--ease),
                  box-shadow var(--dur-fast) var(--ease);
    }
    textarea {
      padding: var(--space-3) var(--space-4);
      height: auto; min-height: 88px; resize: vertical;
    }
    input:focus, textarea:focus {
      border-color: var(--color-primary);
      box-shadow: 0 0 0 3px var(--color-primary-soft);
    }
    .hint {
      font-size: var(--fs-xs);
      color: var(--color-text-subtle);
      margin-top: 4px;
    }
    :host([invalid]) input, :host([invalid]) textarea { border-color: var(--color-danger); }
    :host([invalid]) .hint { color: var(--color-danger); }
  `;

  constructor() {
    super();
    this.label = '';
    this.type = 'text';
    this.name = '';
    this.value = '';
    this.placeholder = '';
    this.hint = '';
    this.required = false;
    this.invalid = false;
  }

  #onInput(e) {
    this.value = e.target.value;
    this.emit('ui-input', { value: this.value, name: this.name });
  }
  #onChange(e) {
    this.value = e.target.value;
    this.emit('ui-change', { value: this.value, name: this.name });
  }

  #renderField() {
    if (this.type === 'textarea') {
      return html`
        <textarea
          name=${this.name}
          placeholder=${this.placeholder}
          ?required=${this.required}
          .value=${this.value}
          @input=${this.#onInput}
          @change=${this.#onChange}
        ></textarea>`;
    }
    return html`
      <input
        type=${this.type}
        name=${this.name}
        placeholder=${this.placeholder}
        ?required=${this.required}
        .value=${this.value}
        @input=${this.#onInput}
        @change=${this.#onChange}
      >`;
  }

  render() {
    return html`
      ${this.label
        ? html`<label>${this.label}${this.required ? ' *' : ''}</label>`
        : nothing}
      <div class="wrap">${this.#renderField()}</div>
      ${this.hint ? html`<div class="hint">${this.hint}</div>` : nothing}
    `;
  }
}
customElements.define('ui-input', UIInput);
