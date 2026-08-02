import { BaseElement, attr } from './base.js';

/**
 * <ui-input label="Full name" placeholder="e.g. Aarav" name="fullName" required></ui-input>
 * <ui-input type="password" label="Password"></ui-input>
 */
class UIInput extends BaseElement {
  static styles = `
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
    input, textarea, select {
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
      transition: border-color var(--dur-fast) var(--ease), box-shadow var(--dur-fast) var(--ease);
    }
    textarea { padding: var(--space-3) var(--space-4); height: auto; min-height: 88px; resize: vertical; }
    input:focus, textarea:focus, select:focus {
      border-color: var(--color-primary);
      box-shadow: 0 0 0 3px var(--color-primary-soft);
    }
    .hint { font-size: var(--fs-xs); color: var(--color-text-subtle); margin-top: 4px; }
    :host([invalid]) input, :host([invalid]) textarea { border-color: var(--color-danger); }
    :host([invalid]) .hint { color: var(--color-danger); }
  `;
  render() {
    const label = attr(this, 'label');
    const type = attr(this, 'type', 'text');
    const placeholder = attr(this, 'placeholder');
    const value = attr(this, 'value');
    const name = attr(this, 'name');
    const hint = attr(this, 'hint');
    const required = this.hasAttribute('required') ? 'required' : '';
    let field;
    if (type === 'textarea') {
      field = `<textarea name="${name}" placeholder="${placeholder}" ${required}>${value}</textarea>`;
    } else if (type === 'select') {
      field = `<select name="${name}" ${required}><slot></slot></select>`;
    } else {
      const valAttr = value ? ` value="${value.replace(/"/g,'&quot;')}"` : '';
      field = `<input type="${type}" name="${name}" placeholder="${placeholder}"${valAttr} ${required}>`;
    }
    return `
      ${label ? `<label>${label}${required ? ' *' : ''}</label>` : ''}
      <div class="wrap">${field}</div>
      ${hint ? `<div class="hint">${hint}</div>` : ''}`;
  }
}
customElements.define('ui-input', UIInput);
