import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-file-upload
 *   label="Bulk import students"
 *   accept=".csv,text/csv"
 *   multiple
 *   max-size="5242880">
 * </ui-file-upload>
 *
 * DSL surface:
 *   - label     : field label
 *   - accept    : MIME / extension filter (passed to <input type="file">)
 *   - multiple  : boolean
 *   - max-size  : bytes (client-side check)
 *   - name      : form field name
 *   - hint      : helper text
 *
 * Emits `ui-change` with { files: File[] } — files also live at .files.
 */
class UIFileUpload extends LitBaseElement {
  static properties = {
    label:     { type: String,  reflect: true },
    accept:    { type: String,  reflect: true },
    multiple:  { type: Boolean, reflect: true },
    'max-size':{ type: Number,  reflect: true, attribute: 'max-size' },
    name:      { type: String,  reflect: true },
    hint:      { type: String,  reflect: true },
    dragOver:  { state: true },
  };

  static styles = css`
    :host { display: block; }
    .label {
      display: block; font-size: var(--fs-xs); color: var(--color-text-muted);
      margin-bottom: 6px; font-weight: var(--fw-medium);
    }
    .drop {
      position: relative;
      display: flex; flex-direction: column; align-items: center; justify-content: center;
      gap: 8px;
      padding: 24px;
      background: var(--color-surface);
      border: 2px dashed var(--color-border-strong);
      border-radius: var(--radius-lg);
      color: var(--color-text-muted);
      text-align: center;
      cursor: pointer;
      transition: border-color var(--dur-fast) var(--ease),
                  background var(--dur-fast) var(--ease);
    }
    .drop:hover, :host([dragOver]) .drop {
      border-color: var(--color-primary);
      background: var(--color-primary-soft);
      color: var(--color-primary);
    }
    .drop input { display: none; }
    .drop .title { font-weight: var(--fw-semibold); color: var(--color-text); }
    .hint { font-size: var(--fs-xs); color: var(--color-text-subtle); margin-top: 6px; }

    ul {
      list-style: none; padding: 0; margin: 12px 0 0;
      display: flex; flex-direction: column; gap: 6px;
    }
    li {
      display: flex; align-items: center; gap: 10px;
      padding: 8px 12px;
      background: var(--color-surface-alt);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      font-size: var(--fs-sm); color: var(--color-text);
    }
    li .size { color: var(--color-text-muted); font-size: var(--fs-xs); font-variant-numeric: tabular-nums; }
    li .remove {
      margin-left: auto;
      appearance: none; border: 0; background: transparent; cursor: pointer;
      color: var(--color-text-muted); width: 22px; height: 22px;
      border-radius: 4px; display: grid; place-items: center;
    }
    li .remove:hover { background: color-mix(in srgb, var(--color-text) 8%, transparent); color: var(--color-danger); }
    li.err { border-color: var(--color-danger); color: var(--color-danger); }
  `;

  constructor() {
    super();
    this.label = '';
    this.accept = '';
    this.multiple = false;
    this['max-size'] = 0;
    this.name = '';
    this.hint = '';
    this.dragOver = false;
    this.files = [];
  }

  #openPicker = () => this.$('input')?.click();

  #onDragOver = (e) => { e.preventDefault(); this.dragOver = true; this.setAttribute('dragOver',''); };
  #onDragLeave = () => { this.dragOver = false; this.removeAttribute('dragOver'); };
  #onDrop = (e) => {
    e.preventDefault();
    this.#onDragLeave();
    this.#accept([...e.dataTransfer.files]);
  };
  #onChange = (e) => this.#accept([...e.target.files]);
  #accept(files) {
    if (!this.multiple) files = files.slice(0, 1);
    const max = this['max-size'] || 0;
    files.forEach(f => { if (max && f.size > max) f.__oversize = true; });
    this.files = files;
    this.emit('ui-change', { files });
  }
  #remove(i) {
    this.files.splice(i, 1);
    this.files = [...this.files];
    this.emit('ui-change', { files: this.files });
  }

  #fmtSize(n) {
    const u = ['B','KB','MB','GB']; let i = 0;
    while (n >= 1024 && i < u.length - 1) { n /= 1024; i++; }
    return `${n.toFixed(n < 10 ? 1 : 0)} ${u[i]}`;
  }

  render() {
    return html`
      ${this.label ? html`<span class="label">${this.label}</span>` : nothing}
      <div class="drop"
           @click=${this.#openPicker}
           @dragover=${this.#onDragOver}
           @dragleave=${this.#onDragLeave}
           @drop=${this.#onDrop}>
        <ui-icon name="upload" size="24"></ui-icon>
        <div class="title">Drop files here or click to browse</div>
        ${this.hint ? html`<div class="hint">${this.hint}</div>` : nothing}
        <input type="file"
               name=${this.name}
               accept=${this.accept}
               ?multiple=${this.multiple}
               @change=${this.#onChange}>
      </div>
      ${this.files.length
        ? html`<ul>
            ${this.files.map((f, i) => html`
              <li class=${f.__oversize ? 'err' : ''}>
                <ui-icon name="clipboard" size="14"></ui-icon>
                <span>${f.name}</span>
                <span class="size">${this.#fmtSize(f.size)}${f.__oversize ? ' — too large' : ''}</span>
                <button class="remove" title="Remove" @click=${() => this.#remove(i)}>
                  <ui-icon name="x" size="14"></ui-icon>
                </button>
              </li>`)}
          </ul>`
        : nothing}
    `;
  }
}
customElements.define('ui-file-upload', UIFileUpload);
