import { LitBaseElement, html, css } from '../base.js';

/**
 * <ui-center
 *   max-w="72ch"             → maximum inline width. Any CSS length. Default 72ch (readable text).
 *   padded                   → boolean; add horizontal gutter (var(--space-4))
 *   intrinsic>               → boolean; also center children on the cross axis (flex column)
 *   <article>...</article>
 * </ui-center>
 *
 * Centered narrow column. Use for article/text bodies, empty states,
 * login forms, marketing hero content — anything that shouldn't run edge
 * to edge on wide screens.
 *
 * By default, only the container is centered on the horizontal axis
 * (via `margin-inline: auto`). Add `intrinsic` to also flex-column
 * center each child.
 *
 * Pure layout — emits no events.
 */
class UICenter extends LitBaseElement {
  static properties = {
    'max-w':   { type: String,  reflect: true, attribute: 'max-w' },
    maxW:      { type: String,  attribute: 'max-w', reflect: true },
    padded:    { type: Boolean, reflect: true },
    intrinsic: { type: Boolean, reflect: true },
  };

  static styles = css`
    :host {
      display: block;
      box-sizing: content-box;
      margin-inline: auto;
      max-width: 72ch;
      min-width: 0;
    }

    :host([padded]) {
      padding-inline: var(--space-4);
    }

    :host([intrinsic]) {
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  `;

  constructor() {
    super();
    this.maxW = '72ch';
    this.padded = false;
    this.intrinsic = false;
  }

  updated(changed) {
    if (changed.has('maxW')) {
      this.style.maxWidth = String(this.maxW || '72ch');
    }
  }

  render() { return html`<slot></slot>`; }
}

customElements.define('ui-center', UICenter);
