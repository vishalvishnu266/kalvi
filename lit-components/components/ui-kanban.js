import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * Simple drag-and-drop kanban board.
 *
 * <ui-kanban>
 *   <ui-kanban-column title="To do">
 *     <ui-kanban-card id="c1">Register new admission</ui-kanban-card>
 *     <ui-kanban-card id="c2">Prepare exam schedule</ui-kanban-card>
 *   </ui-kanban-column>
 *   <ui-kanban-column title="In progress"></ui-kanban-column>
 *   <ui-kanban-column title="Done"></ui-kanban-column>
 * </ui-kanban>
 *
 * Emits `ui-move` on the board with:
 *   { card, from, to, index } — card id, source column title, target column title.
 */
class UIKanban extends LitBaseElement {
  static styles = css`
    :host {
      display: grid;
      grid-auto-flow: column;
      grid-auto-columns: minmax(240px, 1fr);
      gap: var(--space-4);
      overflow-x: auto;
      padding-bottom: 8px;
    }
    ::slotted(ui-kanban-column) { min-width: 0; }
  `;

  connectedCallback() {
    super.connectedCallback();
    this.addEventListener('ui-kanban-drop', (e) => this.emit('ui-move', e.detail));
  }

  render() { return html`<slot></slot>`; }
}
customElements.define('ui-kanban', UIKanban);

class UIKanbanColumn extends LitBaseElement {
  static properties = {
    title: { type: String, reflect: true },
    _over: { state: true },
  };

  static styles = css`
    :host {
      display: flex; flex-direction: column;
      background: var(--color-surface-alt);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-lg);
      padding: 10px;
      min-height: 120px;
      transition: background var(--dur-fast) var(--ease), border-color var(--dur-fast) var(--ease);
    }
    :host([data-over]) {
      background: var(--color-primary-soft);
      border-color: var(--color-primary);
    }
    header {
      display: flex; align-items: center; gap: 8px;
      margin-bottom: 10px;
    }
    header .title {
      flex: 1;
      font-size: var(--fs-sm); font-weight: var(--fw-semibold);
      color: var(--color-text);
    }
    header .count {
      font-size: 11px;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      color: var(--color-text-muted);
      padding: 1px 8px; border-radius: 999px;
      font-weight: var(--fw-medium);
      font-variant-numeric: tabular-nums;
    }
    .cards { display: flex; flex-direction: column; gap: 8px; flex: 1; }
  `;

  constructor() { super(); this.title = ''; this._over = false; }

  #onDragOver = (e) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'move';
    if (!this._over) { this._over = true; this.setAttribute('data-over',''); }
  };
  #onDragLeave = () => { this._over = false; this.removeAttribute('data-over'); };
  #onDrop = (e) => {
    e.preventDefault();
    this.#onDragLeave();
    const id = e.dataTransfer.getData('text/plain');
    const card = document.getElementById(id);
    if (!card || card.tagName !== 'UI-KANBAN-CARD') return;
    const from = card.parentElement;
    const list = this.renderRoot.querySelector('.cards');
    list.appendChild(card);
    this.emit('ui-kanban-drop', {
      card: id,
      from: from?.getAttribute('title') || '',
      to: this.title,
      index: [...list.children].indexOf(card),
    });
    this.requestUpdate();
    from?.requestUpdate?.();
  };

  #cardCount() { return this.querySelectorAll(':scope > ui-kanban-card').length; }

  render() {
    return html`
      <header>
        <span class="title">${this.title}</span>
        <span class="count">${this.#cardCount()}</span>
      </header>
      <div class="cards"
           @dragover=${this.#onDragOver}
           @dragleave=${this.#onDragLeave}
           @drop=${this.#onDrop}>
        <slot></slot>
      </div>
    `;
  }
}
customElements.define('ui-kanban-column', UIKanbanColumn);

class UIKanbanCard extends LitBaseElement {
  static styles = css`
    :host {
      display: block;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-md);
      padding: 10px 12px;
      font-size: var(--fs-sm); color: var(--color-text);
      cursor: grab;
      box-shadow: var(--shadow-xs);
      transition: box-shadow var(--dur-fast) var(--ease),
                  transform var(--dur-fast) var(--ease);
    }
    :host(:hover) { box-shadow: var(--shadow-sm); border-color: var(--color-text-subtle); }
    :host(:active) { cursor: grabbing; }
    :host([dragging]) { opacity: 0.35; transform: rotate(2deg); }
  `;

  connectedCallback() {
    super.connectedCallback();
    if (!this.id) this.id = 'k-' + Math.random().toString(36).slice(2, 8);
    this.setAttribute('draggable', 'true');
    this.addEventListener('dragstart', (e) => {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', this.id);
      this.setAttribute('dragging', '');
    });
    this.addEventListener('dragend', () => this.removeAttribute('dragging'));
  }

  render() { return html`<slot></slot>`; }
}
customElements.define('ui-kanban-card', UIKanbanCard);
