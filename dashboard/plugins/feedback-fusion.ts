import { GrpcWebFetchTransport } from "@protobuf-ts/grpcweb-transport";

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
