/*
 * Copyright (c) 2024 OneLiteFeatherNET
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

import { css, html, LitElement } from "lit";
import { msg, updateWhenLocaleChanges } from "@lit/localize";
import { customElement, property } from "lit/decorators.js";
import { localized } from "@lit/localize";

@customElement("feedback-fusion-field-number")
@localized()
export class FeedbackFusionFieldNumber extends LitElement {
  constructor() {
    super();
    updateWhenLocaleChanges(this);
  }

  static styles = css`
    input {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      border-radius: 4px;
      background: rgba(var(--feedback-fusion-inactive), 0.1);
      width: calc(100% - 32px);
      padding: 16px;
      color: rgb(var(--feedback-fusion-text));
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
    }

    input:focus {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    input:invalid {
      border-color: rgb(var(--feedback-fusion-error));
    }

    input:invalid ~ .feedback-fusion__field-error {
      display: block;
    }

    .feedback-fusion__field-error {
      color: rgb(var(--feedback-fusion-error));
      font-size: 11px;
      display: none;
    }
  `

  @property({ type: Object })
  options?: any;

  @property({ type: Number, attribute: false })
  value: number = 0;

  onChange(event: Event) {
    // @ts-ignore
    this.inputValue = event.target.value;
  }

  get inputValue() {
    return this.value;
  }

  set inputValue(value: number) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  render() {
    return html`
      <input @change=${this.onChange} value=${this.inputValue} type="number" placeholder=${this.options!.placeholder} min=${this.options.min} max=${this.options.max} />

      <div class="feedback-fusion__field-error">
        ${isNaN(this.inputValue) || !this.inputValue ? `
          ${msg("Value is not a number")}
        ` : `
          ${msg(html`Value must lie within ${this.options.min} and ${this.options.max}`)}
        `}
      </div>
    `;
  }
}
