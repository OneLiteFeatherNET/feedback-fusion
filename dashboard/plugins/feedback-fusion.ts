import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";
import {
  FeedbackFusionV1Client,
  PublicFeedbackFusionV1Client,
} from "~/composables/feedback-fusion-v1/service.client";

export default defineNuxtPlugin(async (app) => {
  return {
    provide: {
      feedbackFusion: new FeedbackFusionV1Client(
        new GrpcWebFetchTransport({
          baseUrl: app.$config.public.feedbackFusionEndpoint as string,
        }),
      ),
      publicFeedbackFusion: new PublicFeedbackFusionV1Client(
        new GrpcWebFetchTransport({
          baseUrl: app.$config.public.feedbackFusionEndpoint as string,
        }),
      ),
    },
  };
});
