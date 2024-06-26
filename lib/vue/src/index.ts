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
  FeedbackFusionConfigurationOptions,
  patchConfig,
  PublicFeedbackFusionV1Client,
} from "@onelitefeathernet/feedback-fusion-core";
import i18next from "i18next";
import { App } from "vue";
import Prompt from "./components/Prompt.vue";
import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

export const FeedbackFusion = {
  install(Vue: App, config: FeedbackFusionConfigurationOptions) {
    const patchedConfig = patchConfig(config);

    i18next.init({
      lng: config.defaultLocale,
      resources: config.locales as any,
    });

    Vue.provide("feedbackFusionState", {
      config: patchedConfig,
      client: new PublicFeedbackFusionV1Client(
        new GrpcWebFetchTransport({
          baseUrl: config.endpoint,
        }),
      ),
    });

    Vue.component("FeedbackFusionPrompt", Prompt);
  },
};
