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

import { localized, updateWhenLocaleChanges } from "@lit/localize";
import { css, LitElement } from "lit";
import { customElement, property } from "lit/decorators.js";
import { FieldType } from "../feedback-fusion-v1";
import { html, unsafeStatic } from "lit/static-html.js";
import "./Text.js";
import "./Number.js";
import "./Rating.js";
import "./Checkbox.js";
import "./Selection.js";
import "./Range.js";

@customElement("feedback-fusion-field")
@localized()
export class FeedbackFusionField extends LitElement {
  constructor() {
    super();
    updateWhenLocaleChanges(this);
  }

  static styles = css`
    .feedback-fusion__field {
       margin-top: 25px;
       margin-bottom: 15px;
    }

    .feedback-fusion__field > :last-child {
      width: 100%
    }

    .feedback-fusion__field .feedback-fusion__field-title {
       color: rgb(var(--feedback-fusion-inactive));
       font-size: 14px;
       font-weight: bold;
    }

    .feedback-fusion__field .feedback-fusion__field-description {
       color: rgb(var(--feedback-fusion-subtitle));
       font-size: 11px;
    }

    .feedback-fusion__field:focus-within .feedback-fusion__field-title {
       color: rgb(var(--feedback-fusion-primary));
    }
  `

  @property({ type: String })
  fieldId?: string;

  @property({ type: String })
  fieldTitle?: string;

  @property({ type: String })
  fieldType?: FieldType;

  // TODO: create an interface for the options
  @property({ type: Object })
  options?: any;

  @property({ attribute: false })
  value?: any;

  onUpdate(event: CustomEvent) {
    this.fieldValue = event.detail.value
  }

  set fieldValue(value: any) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  get fieldValue() {
    return this.value;
  }

  fieldTypeString() {
    return Object.keys(FieldType).find(key => FieldType[key] === this.fieldType)!.toLowerCase();
  }

  render() {
    return html`
      <div class="feedback-fusion__field">
        <div class="feedback-fusion__field-title">
          ${this.fieldTitle}
        </div>

        <${unsafeStatic(`feedback-fusion-field-${this.fieldTypeString()}`)} .fieldId=${this.fieldId} .value=${this.fieldValue} .options=${this.options[this.fieldTypeString()]} @update=${this.onUpdate} />
      </div>
    `
  }
}
