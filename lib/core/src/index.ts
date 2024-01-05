import {
  FeedbackPromptResponse,
  SubmitFeedbackPromptResponseRequest,
} from "../../../bindings";

export class FeedbackFusionClient {
  public baseURL: string;

  public constructor(baseURL: string, target: String) {
    this.baseURL = `${baseURL}/v1/target/${target}/`;
  }

  /**
   * Submit a new response to the prompt
   * @param prompt {string} the target prompt
   * @param responses {SubmitFeedbackPromptResponseRequest} the field responses
   * @returns {Promise<FeedbackPromptResponse>}
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
}

export * from "../../../bindings";
