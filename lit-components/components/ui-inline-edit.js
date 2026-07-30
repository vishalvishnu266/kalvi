import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Click-to-edit text field. Perfect for inline editing inside tables/cards.
 *
 * <ui-inline-edit value="Aarav Kumar" name="fullName"></ui-inline-edit>
 * <ui-inline-edit value="12" name="roll" type="number"></ui-inline-edit>
 * <ui-inline-edit value="Notes" name="notes" type="textarea" multiline></ui-inline-edit>
 *
 * DSL surface:
 *   - value       : current committed value
 *   - name        : optional form field name
 *   - placeholder : shown when value is empty
 *   - type        : "text" (default) | "number" | "textarea"
 *   - required    : boolean; empty commit reverts to previous value
 *   - readonly    : boolean; renders as static text only
 *
 * Emits `ui-change` with { value, previous, name } on commit (Enter / blur).
 * Escape cancels and reverts.
 */
class UIInlineEdit extends LitBaseElement {
  static properties = {
    value:       { type: String,  reflect: true },
    name:        { type: String,  reflect: true },
    placeholder: { type: String,  reflect: true },
    type:        { type: String,  reflect: true },
    required:    { type: Boolean, reflect: true },
    readonly:    { type: Boolean, reflect: true },
    editing:     { type: Boolean, reflect: true, state: true },
  };

  static styles = css`
    :host { display: inline-block; min-width: 60px; max-width: 100%; }
    .view {
      display: inline-flex; align-items: center; gap: 6px;
      padding: 4px 8px; border-radius: 6px;
      cursor: text; color: var(--color-text);
      font-size: var(--fs-sm);
      border: 1px solid transparent;
      transition:
        background var(--dur-fast) var(--ease),
        border-color var(--dur-fast) var(--ease);
      max-width: 100%;
    }
    :host(:not([readonly])) .view:hover {
      background: color-mix(in srgb, var(--color-text) 5%, transparent);
      border-color: var(--color-border);
    }
    :host([readonly]) .view { cursor: default; }
    .view .txt {
      overflow: hidden; text-overflow: ellipsis; white-space: nowrap;
      max-width: 240px;
    }
    .view .placeholder { color: var(--color-text-subtle); font-style: italic; }
    .view ui-icon { color: var(--color-text-subtle); opacity: 0; transition: opacity var(--dur-fast) var(--ease); }
    :host(:not([readonly])) .view:hover ui-icon { opacity: 1; }

    input, textarea {
      font: inherit; font-size: var(--fs-sm);
      color: var(--color-text);
      padding: 4px 8px;
      background: var(--color-surface);
      border: 1px solid var(--color-primary);
      border-radius: 6px;
      outline: none;
      box-shadow: 0 0 0 3px var(--color-primary-soft);
      width: 100%; min-width: 120px;
      box-sizing: border-box;
    }
    textarea { min-height: 60px; resize: vertical; }
  `;

  constructor() {
    super();
    this.value = '';
    this.name = '';
    this.placeholder = 'Empty';
    this.type = 'text';
    this.required = false;
    this.readonly = false;
    this.editing = false;
    this._prev = '';
  }

  #enterEdit = () => {
    if (this.readonly) return;
    this._prev = this.value;
    this.editing = true;
    this.updateComplete.then(() => {
      const el = this.$('input, textarea');
      el?.focus();
      if (el?.setSelectionRange) el.setSelectionRange(el.value.length, el.value.length);
    });
  };
  #commit = () => {
    const raw = this.$('input, textarea')?.value ?? this.value;
    let next = raw.trim();
    if (this.required && !next) next = this._prev; // revert if empty
    this.value = next;
    this.editing = false;
    if (next !== this._prev) {
      this.emit('ui-change', { value: next, previous: this._prev, name: this.name });
    }
  };
  #cancel = () => {
    this.value = this._prev;
    this.editing = false;
  };
  #onKey = (e) => {
    if (e.key === 'Escape') { e.preventDefault(); this.#cancel(); }
    else if (e.key === 'Enter' && this.type !== 'textarea') { e.preventDefault(); this.#commit(); }
    else if (e.key === 'Enter' && (e.ctrlKey || e.metaKey)) { e.preventDefault(); this.#commit(); }
  };

  render() {
    if (!this.editing) {
      const hasValue = this.value && this.value.length > 0;
      return html`
        <span class="view" tabindex=${this.readonly ? nothing : '0'}
              @click=${this.#enterEdit}
              @keydown=${(e) => { if (!this.readonly && (e.key === 'Enter' || e.key === ' ')) { e.preventDefault(); this.#enterEdit(); } }}>
          <span class=${hasValue ? 'txt' : 'txt placeholder'}>${hasValue ? this.value : this.placeholder}</span>
          ${this.readonly ? nothing : html`<ui-icon name="edit" size="12"></ui-icon>`}
        </span>`;
    }
    if (this.type === 'textarea') {
      return html`<textarea
        .value=${this.value}
        @keydown=${this.#onKey}
        @blur=${this.#commit}></textarea>`;
    }
    return html`<input
      type=${this.type}
      .value=${this.value}
      placeholder=${this.placeholder}
      @keydown=${this.#onKey}
      @blur=${this.#commit}>`;
  }
}
customElements.define('ui-inline-edit', UIInlineEdit);
