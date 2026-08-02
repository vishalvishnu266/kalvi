import { BaseElement } from './base.js';

/**
 * <ui-table>
 *   <table>
 *     <thead><tr><th>Name</th>...</tr></thead>
 *     <tbody><tr>...</tr></tbody>
 *   </table>
 * </ui-table>
 *
 * On mobile, rows become vertical stacks (like a native app card list).
 */
class UITable extends BaseElement {
  static styles = `
    :host { display: block; width: 100%; max-width: 100%; min-width: 0; box-sizing: border-box; }
    /* Table can be wider than viewport → scroll INSIDE the card only, not the whole page. */
    .scroll {
      overflow-x: auto;
      overflow-y: hidden;
      -webkit-overflow-scrolling: touch;
      max-width: 100%;
    }
    ::slotted(table) {
      width: 100%;
      min-width: 720px;               /* keeps columns readable, scrolls inside */
      border-collapse: separate;
      border-spacing: 0;
      font-size: var(--fs-sm);
    }
  `;
  render() { return `<div class="scroll"><slot></slot></div>`; }
}
customElements.define('ui-table', UITable);
