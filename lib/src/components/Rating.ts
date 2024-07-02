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

@customElement("feedback-fusion-field-rating")
export class FeedbackFusionFieldRating extends LitElement {
  constructor() {
    super();
  }

  static styles = css`
    .feedback-fusion__field__rating-point {
      display: inline-block;
    }

    .feedback-fusion__field__rating-point input {
      height: 0;
      width: 0;
      visibility: hidden;
      display: none;
    }

    .feedback-fusion__field__rating-point svg {
      cursor: pointer;
      height: 35px;
    }
  `

  @property({ type: String })
  fieldId?: string;

  @property({ type: Object })
  options?: any;

  @property({ type: Number, attribute: false })
  value: number = 0;

  @property({ attribute: false })
  starColor: Array<string> = []

  onChange(event: Event) {
    // @ts-ignore
    this.inputValue = event.target.value;

    // @ts-ignore
    this.starColor = this.starColor.map((_, i) => +event.target.value > i ? "var(--feedback-fusion-primary)" : "var(--feedback-fusion-inactive)")
  }

  get inputValue() {
    return this.value;
  }

  set inputValue(value: number) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  connectedCallback(): void {
    this.starColor = new Array(this.options.max).fill("var(--feedback-fusion-inactive)");

    super.connectedCallback();
  }

  render() {
    return html`
      ${new Array(this.options.max).fill(0).map((_, i: number) => html`
        <div class="feedback-fusion__field__rating-point">
          <input type="radio" name=${this.fieldId} id=${this.fieldId! + i} value=${i + 1} @change=${this.onChange} />
          <label for=${this.fieldId! + i}>
            <svg xmlns="http://www.w3.org/2000/svg" fill=${this.starColor[i]} viewBox="0 0 24 24">
              <title>star-outline</title>
              <path
                d="M12,15.39L8.24,17.66L9.23,13.38L5.91,10.5L10.29,10.13L12,6.09L13.71,10.13L18.09,10.5L14.77,13.38L15.76,17.66M22,9.24L14.81,8.63L12,2L9.19,8.63L2,9.24L7.45,13.97L5.82,21L12,17.27L18.18,21L16.54,13.97L22,9.24Z" />
            </svg>
          </label>
        </div>
      `)}
    `;
  }
}
