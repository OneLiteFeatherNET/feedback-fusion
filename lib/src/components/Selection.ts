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
import { classMap } from "lit/directives/class-map.js";

@customElement("feedback-fusion-field-selection")
export class FeedbackFusionFieldSelection extends LitElement {
  static styles = css`
    .feedback-fusion__field-input-container {
      outline: none;
      border: 1px solid rgb(var(--feedback-fusion-inactive));
      border-radius: 4px;
      width: calc(100% - 32px);
      padding: 16px;
      color: rgb(var(--feedback-fusion-text));
      font-size: 16px;
      line-height: 24px;
      transition: 0.2s ease-out all;
      display: flex;
      flex-direction: row;
    }

    .feedback-fusion__field-input-container:focus-within {
      border-color: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field-input-container .feedback-fusion__field__selection-chips {
      padding: 0 10px;
    }

    .feedback-fusion__field-input-container .feedback-fusion__field__selection-chips div {
      background: rgb(var(--feedback-fusion-primary));
      padding: 0 12px;
      border-radius: 16px;
      display: inline-block;
      margin-left: 5px;
    }

    .feedback-fusion__field-input-container input {
      flex-grow: 1;
      background: transparent;
      outline: none;
      border: none;
    }

    .feedback-fusion__field__selection-list {
      width: calc(100% - 20px);
      max-height: 300px;
      overflow: scroll;
      padding: 10px;
      border: 1px solid rgb(var(--feedback-fusion-subtitle));
      border-top: none;
      border-radius: 0 0 10px 10px;
      transition: 0.2s ease-out;
    }

    .feedback-fusion__field__selection-list > div {
      padding: 10px;
      border-bottom: 1px solid rgb(var(--feedback-fusion-subtitle));
    }

    .feedback-fusion__field__selection-list > div:hover {
      cursor: pointer;
      background: rgba(var(--feedback-fusion-subtitle), 0.1);
    }

    .feedback-fusion__field__selection-list-hidden {
      padding: 0;
      height: 0;
      border: none;
    }

    .feedback-fusion__field__selection-list-selected {
      background: rgba(var(--feedback-fusion-primary), 0.1);
    }
  `

  @property({ type: Object })
  options?: any;

  // internal
  @property({ attribute: false })
  expanded: boolean = false;

  @property({ attribute: false })
  search: string = "";

  @property({ type: Array, attribute: false })
  value: string[] = [];

  get inputValue() {
    return this.value || [] as string[];
  }

  set inputValue(value: string[]) {
    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  private toggleExpanded() {
    this.expanded = !this.expanded;
  }

  private onSearch(event: Event) {
    // @ts-ignore
    this.search = event.target.value
  }

  private insertValue(value: string) {
    if (this.options.multiple)
      this.inputValue = this.inputValue.concat([value])
    else
      this.inputValue = [value]
  }

  private onKeyUp(event: KeyboardEvent) {
    if (event.key === "Enter" && this.options?.combobox && !!this.search) {
      this.insertValue(this.search);

      if (!this.options.values.includes(this.search))
        this.options.values = this.options.values.concat([this.search]);

      this.search = "";
    }
  }

  private onClick(value: string) {
    return () => {
      if (this.inputValue.includes(value))
        this.inputValue = this.inputValue.filter((item: string) => item !== value);
      else
        this.insertValue(value);

    };
  }

  render() {
    return html`
      <div class="feedback-fusion__field-input-container" @click=${this.toggleExpanded}>
        <div class="feedback-fusion__field__selection-chips">
          ${this.inputValue.map(value => html`
            <div>
              ${value}
            </div>
          `)}
        </div>

        <input @keyup=${this.onKeyUp} type="text" ?readonly=${!this.options.combobox} .value=${this.search} @input=${this.onSearch} />
      </div>

      <div class=${classMap({ "feedback-fusion__field__selection-list": true, "feedback-fusion__field__selection-list-hidden": !this.expanded })}>
        ${this.options.values.filter(value => this.options.combobox ? value.startsWith(this.search) : true).map(value => html`
          <div class=${classMap({ "feedback-fusion__field__selection-list-selected": this.inputValue.includes(value) })} @click=${this.onClick(value)}>
            ${value}
          </div>
        `)}
      </div>
      `;
  }
}
