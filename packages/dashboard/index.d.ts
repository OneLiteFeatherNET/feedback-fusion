declare module "#app" {
  interface NuxtApp {
    $feedbackFusion: FeedbackFusionV1Client;
    $publicFeedbackFusion: PublicFeedbackFusionV1Client;
  }
}

declare module "vue" {
  interface ComponentCustomProperties {
    $feedbackFusion: FeedbackFusionV1Client;
    $publicFeedbackFusion: PublicFeedbackFusionV1Client;
  }
}

export { };
