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
import { CheckboxStyle } from "../feedback-fusion-v1";

@customElement("feedback-fusion-field-checkbox")
export class FeedbackFusionFieldCheckbox extends LitElement {
  static styles = css`
    label input {
      height: 0;
      width: 0;
      display: none;
      visibility: hidden;
    }

    label span {
      position: relative;
      width: 60px;
      height: 35px;
      display: inline-block;
      border-radius: 16px;
      background: rgb(var(--feedback-fusion-inactive));
      cursor: pointer;
    }

    label span:after {
      position: absolute;
      top: 5px;
      bottom: 5px;
      left: 5px;
      width: 25px;
      content: "";
      border-radius: 50%;
      background: white;
      transition: 0.15s ease-out;
    }

    label input:checked + span {
      background: rgb(var(--feedback-fusion-primary));
    }

    label input:checked + span:after {
      left: 30px;
    }
  `

  @property({ type: Object })
  options?: any;

  @property({ type: Boolean, attribute: false })
  value: boolean = false;

  onChange(event: Event) {
    // @ts-ignore
    this.inputValue = event.target.value;
  }

  get inputValue() {
    return this.value;
  }

  set inputValue(value: boolean) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  render() {
    return html`
      ${this.options.style === CheckboxStyle.NORMAL ? html`
        <input type="checkbox" value=${this.value} @change=${this.onChange} />
      `: html `
        <label>
          <input type="checkbox" />
          <span />
        </label>
      `}
    `;
  }
}
