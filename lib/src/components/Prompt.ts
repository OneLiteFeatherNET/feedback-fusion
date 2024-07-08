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
import { Field, Prompt } from "../feedback-fusion-v1.js";
import { localized } from "@lit/localize";
import { PublicFeedbackFusionV1Client } from "../feedback-fusion-v1.client.js";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import "./Field.js";
import { setLocale } from "../locales.js";

@customElement("feedback-fusion-prompt")
@localized()
export class FeedbackFusionPrompt extends LitElement {
  constructor() {
    super();
    updateWhenLocaleChanges(this);
  }

  static styles = css`
    :host {
      --feedback-fusion-text: 255, 255, 245; /* #FFFFF5 */
      --feedback-fusion-subtitle: 117, 117, 117; /* #757575 */
      --feedback-fusion-sheet: 33, 33, 33; /* #212121 */
      --feedback-fusion-primary: 52, 152, 219; /* #3498db */
      --feedback-fusion-inactive: 117, 117, 117; /* #757575 */
      --feedback-fusion-success: 76, 175, 80; /* #4caf50 */
      --feedback-fusion-error: 211, 61, 61; /* #d33d3d */
    }

    .feedback-fusion__prompt {
       color: rgb(var(--feedback-fusion-text));
       width: 100%;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container {
       margin: auto;
       background-color: rgb(var(--feedback-fusion-sheet));
       padding: 16px;
       overflow: hidden;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-header .feedback-fusion__prompt-header-title {
       font-weight: bold;
       font-size: 20px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-header .feedback-fusion__prompt-header-subtitle {
       color: rgb(var(--feedback-fusion-subtitle));
       font-size: 14px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-fields {
       padding: 10px 0;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions {
       margin-top: 10px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button {
       text-transform: uppercase;
       font-weight: bold;
       letter-spacing: 2px;
       font-size: 13px;
       color: rgb(var(--feedback-fusion-primary));
       position: relative;
       padding: 10px 15px;
       background: rgb(var(--feedback-fusion-sheet));
       border: none;
       cursor: pointer;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button:after {
       content: "";
       position: absolute;
       left: 0;
       right: 0;
       top: 0;
       bottom: 0;
       background: rgb(var(--feedback-fusion-primary));
       opacity: 0;
       transition: 0.1s ease-out all;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions button:hover:after {
       opacity: 0.1;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions .feedback-fusion__prompt-actions-submit,
    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-actions .feedback-fusion__prompt-actions-close {
       float: right;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status {
       margin-top: 20px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status div {
       width: 100%;
       padding: 15px;
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status .feedback-fusion__prompt-status-success {
       background: rgb(var(--feedback-fusion-success));
    }

    .feedback-fusion__prompt .feedback-fusion__prompt-container .feedback-fusion__prompt-status .feedback-fusion__prompt-status-error {
       background: rgb(var(--feedback-fusion-error));
    }
  `;

  @property({ type: Boolean })
  autoClose: boolean = false;

  @property({ type: String })
  baseUrl!: string;

  @property({ type: Number })
  closeAfter: number = 1000;

  @property({ type: String })
  locale: string = "en";

  @property({ type: String })
  promptId!: string;

  // internal

  @property({ attribute: false })
  clientProvider!: PublicFeedbackFusionV1Client;

  @property({ attribute: false })
  currentFieldPage = 1;

  @property({ attribute: false })
  data: { [key: string]: any } = {};

  @property({ attribute: false })
  error: boolean = false;

  @property({ attribute: false })
  fields: Field[] = [];

  @property({ attribute: false })
  finished: boolean = false;

  @property({ attribute: false })
  open: boolean = true;

  @property({ attribute: false })
  prompt: Prompt | undefined;

  @property({ attribute: false })
  totalFieldPages = 1;

  async connectedCallback() {
    super.connectedCallback();
    setLocale(this.locale);
    this.clientProvider = new PublicFeedbackFusionV1Client(new GrpcWebFetchTransport({ baseUrl: this.baseUrl }));

    await this._fetchPrompt().catch(console.error);
    await this._fetchFields().catch(console.error);
  }

  private async _fetchPrompt() {
    await this.clientProvider.getPrompt({ id: this.promptId })
      .then((data: any) => this.prompt = data.response)
  }

  private async _fetchFields() {
    await this.clientProvider.getActiveFields({ prompt: this.promptId, pageSize: 10, pageToken: this.currentFieldPage })
      .then((data: any) => {
        this.totalFieldPages = Math.ceil(data.response.total / 10);
        this.fields = data.response.fields;
      })
  }

  private async _submitResponse() {
    const body = {};
    Object.keys(this.data).forEach((key: string) => body[key] = { data: this.data[key] });

    await this.clientProvider.createResponses({ data: body, prompt: this.prompt!.id })
      .then(() => {
        this.data = {};
        this.finished = true;

        if (this.autoClose)
          setTimeout(() => this.open = false, this.closeAfter || 5000);
      })
      .catch(() => this.error = true);
  }

  private onUpdate(field: string) {
    return (event: CustomEvent) => {
      // we have to do it this way so lit can detect the change
      let target = {};
      target[field] = event.detail.value;
      this.data = { ...this.data, ...target };
    }
  }

  render() {
    return html`
      ${this.prompt?.active && this.open ? html`
        <div class="feedback-fusion__prompt">
          <div class="feedback-fusion__prompt-container">
            <div class="feedback-fusion__prompt-header">
              <div class="feedbac-fusion__prompt-header-title">
                <slot name="title">
                  ${this.prompt?.title || msg("Loading...")}
                </slot>
              </div>

              <div class="feedback-fusion__prompt-header-subtitle">
                <slot name="subtitle">
                  ${msg(html`Page ${this.currentFieldPage} of ${this.totalFieldPages}`)}
                </slot>
              </div>
            </div>

            ${this.finished ? html`
              <div class="feedback-fusion__prompt-status">
                <slot name="success">
                  <div class="feedback-fusion__prompt-status-success">
                    ${msg("Thank you for participating in our survey!")}
                  </div>
                </slot>
              </div>
            ` : ''}

            ${this.error ? html`
              <div class="feedback-fusion__prompt-status">
                <slot name="error">
                  <div class="feedback-fusion__prompt-status-error">
                    ${msg("An error occurred while processing your request.")}
                  </div>
                </slot>
              </div>
            ` : ''}

            ${!this.finished ? html`
              <div class="feedback-fusion__prompt-fields">
                ${this.fields.map(field => html`
                  <slot name="field">
                    <feedback-fusion-field .fieldId=${field.id} .value=${this.data[field.id]} @update=${this.onUpdate(field.id)} .fieldTitle=${field.title} .options=${field.options!.options} .fieldType=${field.fieldType} />
                  </slot>
                `)}
              </div>
            ` : ''}

            <div class="feedback-fusion__prompt-actions">
              ${!this.finished ? html`
              <button @click="${this._submitResponse}" class="feedback-fusion__prompt-actions-submit">
                ${msg("Submit")}
              </button>
              ` : html`
              <button @click="${() => this.open = false}" class="feedback-fusion__prompt-actions-close">
                ${msg("Close")}
              </button>
              `}
            </div>
          </div>
        </div>
      ` : ''}
    `;
  }
}
