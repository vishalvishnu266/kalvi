import { LitBaseElement, html, css, nothing } from './base.js';

/**
 * <ui-table>
 *   <table>
 *     <thead><tr><th>Name</th>…</tr></thead>
 *     <tbody><tr>…</tr></tbody>
 *   </table>
 * </ui-table>
 *
 * Just wraps a slotted <table> in a horizontally scrollable container.
 */
class UITable extends LitBaseElement {
  static styles = css`
    :host {
      display: block; width: 100%; max-width: 100%;
      min-width: 0; box-sizing: border-box;
    }
    .scroll {
      overflow-x: auto;
      overflow-y: hidden;
      -webkit-overflow-scrolling: touch;
      max-width: 100%;
    }
    ::slotted(table) {
      width: 100%;
      min-width: 720px;
      border-collapse: separate;
      border-spacing: 0;
      font-size: var(--fs-sm);
    }
  `;

  render() {
    return html`<div class="scroll"><slot></slot></div>`;
  }
}
customElements.define('ui-table', UITable);
