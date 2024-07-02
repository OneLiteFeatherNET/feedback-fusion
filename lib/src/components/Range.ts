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
import { customElement, property, query } from "lit/decorators.js";
import { styleMap } from "lit/directives/style-map.js";

@customElement("feedback-fusion-field-range")
export class FeedbackFusionFieldRange extends LitElement {
  static styles = css`
    .feedback-fusion__field__range {
      padding: 10px;
      height: 30px;
      width: calc(100% - 20px);
      position: relative;
    }

    .feedback-fusion__field__range-background {
      top: 13.5px;
      left: 0;
      right: 0;
      height: 3px;
      position: absolute;
      background: rgb(var(--feedback-fusion-inactive));
    }

    .feedback-fusion__field__range-background div {
      position: absolute;
      top: 0;
      bottom: 0;
      background: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field__range-marker {
      position: absolute;
      left: 0;
      top: 5px;
      height: 20px;
      width: 20px;
      border-radius: 50%;
      background: rgb(var(--feedback-fusion-inactive));
    }

    .feedback-fusion__field__range-marker div {
      position: absolute;
      top: -20px;
      left: 50%;
      transform: translateX(-50%);
      display: none;
    }

    .feedback-fusion__field__range:hover .feedback-fusion__field__range-marker {
      background: rgb(var(--feedback-fusion-primary));
    }

    .feedback-fusion__field__range:hover .feedback-fusion__field__range-marker div {
      display: block;
    }
  `

  @property({ type: Object })
  options?: any;

  @property({ type: String, attribute: false })
  value?: { start: number, end: number };

  @query(".feedback-fusion__field__range-background")
  background?: HTMLDivElement;

  @property({ attribute: false })
  left: number = 0;

  @property({ attribute: false })
  right: number = 0;

  @property({ attribute: false })
  dragLeft: boolean = false;

  @property({ attribute: false })
  dragRight: boolean = false;

  onChange(event: Event) {
    // @ts-ignore
    this.inputValue = event.target.value;
  }

  protected firstUpdated() {
    this.right = this.background!.clientWidth
  }

  get inputValue() {
    return this.value || { start: this.options.min, end: this.options.max };
  }

  set inputValue(value: { start: number, end: number }) {
    this.left = (value.start - this.options.min) / (this.options.max - this.options.min) * this.background!.clientWidth || 0;
    this.right = (value.end - this.options.min) / (this.options.max - this.options.min) * this.background!.clientWidth || 0;

    this.dispatchEvent(new CustomEvent("update", { detail: { value } }))
  }

  private onClick(event: MouseEvent) {
    const x = event.layerX;
    const value = Math.round((this.options.max - this.options.min) * (x / this.background!.clientWidth || 1) + this.options.min);

    if (x < this.left) {
      this.inputValue = { start: value, end: this.inputValue.end };
    }

    if (x > this.right) {
      this.inputValue = { start: this.inputValue.start, end: value };
    }
  }

  private onMouseMove(event: MouseEvent) {
    // @ts-ignore
    if (event.target.className !== "feedback-fusion__field__range") return;

    const x = event.layerX;
    const value = Math.round((this.options.max - this.options.min) * (x / this.background!.clientWidth || 1) + this.options.min);

    if (x < this.right && this.dragLeft && value < this.inputValue.end) {
      this.inputValue = { start: value, end: this.inputValue?.end };
    }

    if (x > this.left && this.dragRight && value > this.inputValue.start) {
      this.inputValue = { start: this.inputValue?.start, end: value };
    }
  }

  render() {
    return html`
      <div @mouseup=${() => { this.dragRight = false; this.dragLeft = false; }} @mouseleave=${() => { this.dragRight = false; this.dragLeft = false; }} @mousemove=${this.onMouseMove} class="feedback-fusion__field__range">
        <div class="feedback-fusion__field__range-background" @click=${this.onClick}>
          <div style=${styleMap({ left: `${this.left}px`, right: `${this.background ? this.background!.clientWidth - this.right : 0}px` })} /></div>
        </div>

        <div @mousedown=${(event: MouseEvent) => { event.preventDefault(); this.dragLeft = true }} style=${styleMap({ left: `calc(${this.left}px - 10px)` })} class="feedback-fusion__field__range-marker marker-left">
          <div>
            ${this.inputValue.start}
          </div>
        </div>

        <div  @mousedown=${(event: MouseEvent) => { event.preventDefault(); this.dragRight = true }} style=${styleMap({ left: `calc(${this.right}px - 10px)` })} class="feedback-fusion__field__range-marker marker-right">
          <div>
            ${this.inputValue.end}
          </div>
        </div
      </div>
    `;
  }
}
