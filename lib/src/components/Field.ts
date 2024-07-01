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

@customElement("feedback-fusion-field")
@localized()
export class FeedbackFusionPrompt extends LitElement {
  constructor() {
    super();
    updateWhenLocaleChanges(this);
  }

  static styles = css`
    .feedback-fusion__field {
       margin-top: 25px;
       margin-bottom: 15px;
    }

    .feedback-fusion__field .feedback-fusion__field-title {
       color: v-bind("theme.inactive");
       font-size: 14px;
       font-weight: bold;
    }

    .feedback-fusion__field .feedback-fusion__field-description {
       color: v-bind("theme.subtitle");
       font-size: 11px;
    }

    .feedback-fusion__field .feedback-fusion__field-error {
       color: v-bind("theme.error");
       font-size: 11px;
       display: none;
    }

    .feedback-fusion__field:focus-within .feedback-fusion__field-title {
       color: v-bind("theme.primary");
    }
  `

  @property({ type: String })
  title?: string;

  @property({ type: String })
  theme: string = "dark";

  @property({ type: String })
  fieldType?: FieldType;

  // TODO: create an interface for the options
  @property({ type: Object });
  options?: any;

  @property({ attribute: false })
  value?: any;

  onChange(event: Event) {
    // @ts-ignore
    this.value = event.target.value
  }

  fieldTypeString() {
    return Object.keys(FieldType).find(key => FieldType[key] === this.fieldType)!.toLowerCase();
  }

  render() {
    return html`
      <div class="feedback-fusion__field">
        <div class="feedback-fusion__field-title">
          ${this.title}
        </div>

        <${unsafeStatic(`feedback-fusion-field-${this.fieldTypeString()}`)} .options=${this.options[this.fieldTypeString()]} .theme=${this.theme} @onChange=${this.onChange} />
      </div>
    `
  }
}
