import { BaseElement, attr } from './base.js';

/**
 * <ui-button variant="primary|secondary|ghost|danger" size="sm|md|lg" icon="🔍" full>
 *   Save
 * </ui-button>
 *
 * Intent-first: the tag name shows *what* is rendered in Rust templates.
 */
class UIButton extends BaseElement {
  static styles = `
    :host { display: inline-block; }
    :host([full]) { display: block; }
    :host([full]) button { width: 100%; }

    button {
      display: inline-flex; align-items: center; justify-content: center;
      gap: var(--space-2);
      font-family: inherit;
      font-weight: var(--fw-medium);
      cursor: pointer;
      border: 1px solid transparent;
      border-radius: var(--radius-md);
      padding: 0 var(--space-4);
      height: 40px;
      transition: background var(--dur-fast) var(--ease),
                  transform var(--dur-fast) var(--ease),
                  border-color var(--dur-fast) var(--ease);
      background: var(--color-primary);
      color: var(--color-primary-contrast);
      font-size: var(--fs-sm);
    }
    button:hover  { background: var(--color-primary-hover); }
    button:active { transform: translateY(1px); }

    :host([variant="secondary"]) button {
      background: var(--color-surface);
      color: var(--color-text);
      border-color: var(--color-border-strong);
    }
    :host([variant="secondary"]) button:hover { background: var(--color-surface-alt); }

    :host([variant="ghost"]) button {
      background: transparent;
      color: var(--color-text);
    }
    :host([variant="ghost"]) button:hover { background: var(--color-primary-soft); color: var(--color-primary); }

    :host([variant="danger"]) button {
      background: var(--color-danger);
      color: #fff;
    }

    :host([size="sm"]) button { height: 32px; padding: 0 var(--space-3); font-size: var(--fs-xs); }
    :host([size="lg"]) button { height: 48px; padding: 0 var(--space-5); font-size: var(--fs-md); }

    .icon { font-size: 1.05em; line-height: 1; display: inline-flex; }
  `;
  render() {
    const icon = attr(this, 'icon');
    // icon can be an SVG name (e.g. "plus") or an emoji/text — if it matches
    // a bare word, render <ui-icon>; else render the raw character(s).
    const isSvg = icon && /^[a-zA-Z]+[a-zA-Z0-9]*$/.test(icon);
    const iconHtml = icon
      ? (isSvg ? `<ui-icon name="${icon}" size="16"></ui-icon>` : `<span class="icon">${icon}</span>`)
      : '';
    return `<button part="btn">${iconHtml}<slot></slot></button>`;
  }
}
customElements.define('ui-button', UIButton);
