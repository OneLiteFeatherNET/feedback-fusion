// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  app: {
    head: {
      charset: "utf-8",
      meta: [{ content: "telephone=no", name: "format-detection" }],
      script: [],
      style: [],
      title: "Feedback-Fusion",
      viewport: "width=device-width, initial-scale=1",
    },
  },
  compatibilityDate: "2024-04-03",
  devtools: { enabled: true },
  modules: [
    "vuetify-nuxt-module",
    "@nuxtjs/i18n",
    "nuxt-oidc-auth",
    "@pinia/nuxt",
    "@nuxtjs/color-mode",
  ],
  i18n: {
    locales: [{ code: "en", language: "en-US", file: "en.json" }],
    lazy: true,
    defaultLocale: "en",
    detectBrowserLanguage: {
      cookieKey: "lang",
      useCookie: true,
      redirectOn: "root",
      alwaysRedirect: true,
    },
  },
  build: {
    transpile: ["@onelitefeathernet/feedback-fusion"],
  },
  oidc: {
    middleware: {
      globalMiddlewareEnabled: true,
      customLoginPage: true,
    },
    providers: {
      oidc: {
        pkce: false,
        validateAccessToken: true,
        scope:
          process.env.NODE_ENV == "development"
            ? ["openid", "profile", "test"]
            : ["openid", "profile"],
        tokenRequestType: "form-urlencoded",
        exposeAccessToken: true,
        exposeIdToken: true,
        authorizationUrl:
          process.env.FEEDBACK_FUSION_OIDC_PROVIDER_AUTHORIZATION_URL,
        tokenUrl: process.env.FEEDBACK_FUSION_OIDC_PROVIDER_TOKEN_URL,
        // @ts-expect-error idk why
        openIdConfiguration:
          process.env.FEEDBACK_FUSION_OIDC_PROVIDER_DISCOVERY_URL,
        redirectUri: process.env.FEEDBACK_FUSION_OIDC_REDIRECT_URL!,
        clientId: process.env.FEEDBACK_FUSION_OIDC_CLIENT_ID,
        clientSecret: process.env.FEEDBACK_FUSION_OIDC_CLIENT_SECRET,
        optionalClaims: ["groups"],
        userNameClaim: "preferred_username",
      },
    },
  },
  runtimeConfig: {
    public: {
      feedbackFusionEndpoint: "NUXT_PUBLIC_FEEDBACK_FUSION_ENDPOINT",
    },
  },
  vuetify: {
    vuetifyOptions: {
      labComponents: true,
    },
  },
});
