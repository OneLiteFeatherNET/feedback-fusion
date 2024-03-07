/*
 * Copyright (c) 2023 OneLiteFeatherNET
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

import {
  FeedbackPromptResponse,
  SubmitFeedbackPromptResponseRequest,
  FeedbackPromptField,
  Page,
} from "../../../bindings";

export class FeedbackFusionClient {
  public baseURL: string;

  public constructor(baseURL: string, target: String) {
    this.baseURL = `${baseURL}/v1/target/${target}/`;
  }

  /**
   * Submit a new response to the prompt
   */
  public async submitResponse(
    prompt: String,
    responses: SubmitFeedbackPromptResponseRequest,
  ): Promise<FeedbackPromptResponse> {
    return await fetch(`${this.baseURL}/prompt/${prompt}/response`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(responses),
    })
      .then((response) => response.json() as Promise<FeedbackPromptResponse>)
      .then((response: FeedbackPromptResponse) => response);
      // TODO: handle errors
  }

  /**
  * Fetch a page of fields 
  */
  public async getFields(prompt: string, page = 1, page_size = 20): Promise<Page<FeedbackPromptField>> {
    return await fetch(`${this.baseURL}/prompt/${prompt}/fetch?page=${page}&page_size=${page_size}`)
      .then((response) => response.json() as Promise<Page<FeedbackPromptField>>)
      .then((response: Page<FeedbackPromptField>) => response)
  }
}

export * from "../../../bindings";
