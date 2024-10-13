// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },
  modules: ["vuetify-nuxt-module", "@nuxtjs/i18n", "nuxt-oidc-auth"],
  oidc: {
    defaultProvider: "oidc",
    middleware: {
      globalMiddlewareEnabled: true,
    },
    providers: {
      oidc: {
        pkce: false,
        validateAccessToken: true,
        scope: ["openid", process.env.FEEDBACK_FUSION_API_SCOPE!],
        tokenRequestType: "form-urlencoded",
        authorizationUrl:
          process.env.FEEDBACK_FUSION_OIDC_PROVIDER_AUTHORIZATION_URL,
        tokenUrl: process.env.FEEDBACK_FUSION_OIDC_PROVIDER_TOKEN_URL,
        // @ts-expect-error idk why
        openIdConfiguration:
          process.env.FEEDBACK_FUSION_OIDC_PROVIDER_DISCOVERY_URL,
        redirectUri: process.env.FEEDBACK_FUSION_OIDC_REDIRECT_URL!,
        clientId: process.env.FEEDBACK_FUSION_OIDC_CLIENT_ID,
        clientSecret: process.env.FEEDBACK_FUSION_OIDC_CLIENT_SECRET,
      },
    },
  },
});
