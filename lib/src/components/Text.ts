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
import { customElement, property } from "lit/decorators.js";

@customElement("feedback-fusion-field-text")
export class FeedbackFusionFieldText extends LitElement {
  static styles = css`
    input, textarea {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      border-radius: 4px;
      width: calc(100% - 32px);
      padding: 16px;
      color: var(--feedback-fusion-text);
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
    }

    input:focus, textarea:focus {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    input:invalid, textarea:invalid {
      border-color: rgb(var(--feedback-fusion-error));
    }
  `

  @property({ type: Object })
  options?: any;

  @property({ type: String, attribute: false })
  value: string = "";

  onChange(event: Event) {
    // @ts-ignore
    this.inputValue = event.target.value;
  }

  get inputValue() {
    return this.value;
  }

  set inputValue(value: string) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  render() {
    return html`
      ${this.options!.lines === 1
        ? html`
          <input @change=${this.onChange} value=${this.inputValue}  type="text" placeholder=${this.options!.placeholder} />
        `
        : html`
          <textarea @change=${this.onChange} value=${this.inputValue} rows=${this.options!.rows} placeholder=${this.options!.placeholder} />
        `}
    `;
  }
}
