import { BaseElement, html, css } from './base.js';

class UITable extends BaseElement {
  static properties = {
    columns: { type: Array },
    data: { type: Array },
    emptyMessage: { type: String, attribute: 'empty-message' },
  };

  constructor() {
    super();
    this.columns = [];
    this.data = [];
    this.emptyMessage = 'No data available';
  }

  static styles = css`
    :host { display: block; width: 100%; overflow-x: auto; }
    table {
      width: 100%;
      border-collapse: collapse;
      font-size: var(--fs-sm);
    }
    th {
      text-align: left;
      padding: var(--space-3) var(--space-4);
      background: var(--color-surface-alt);
      color: var(--color-text-muted);
      font-weight: var(--fw-semibold);
      border-bottom: 1px solid var(--color-border-strong);
    }
    td {
      padding: var(--space-3) var(--space-4);
      border-bottom: 1px solid var(--color-border);
      color: var(--color-text);
    }
    tr:last-child td { border-bottom: none; }
    .empty {
      padding: var(--space-8);
      text-align: center;
      color: var(--color-text-subtle);
      font-style: italic;
    }
  `;

  render() {
    if (this.data.length === 0) {
      return html`<div class="empty">${this.emptyMessage}</div>`;
    }

    return html`
      <table>
        <thead>
          <tr>
            ${this.columns.map(col => html`
              <th style="text-align: ${col.align || 'left'}">${col.header}</th>
            `)}
          </tr>
        </thead>
        <tbody>
          ${this.data.map(row => html`
            <tr>
              ${this.columns.map(col => html`
                <td style="text-align: ${col.align || 'left'}">${row[col.key]}</td>
              `)}
            </tr>
          `)}
        </tbody>
      </table>
    `;
  }
}

customElements.define('ui-table', UITable);
