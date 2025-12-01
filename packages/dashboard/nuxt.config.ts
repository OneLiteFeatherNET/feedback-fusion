// https://nuxt.com/docs/api/configuration/nuxt-config

export default defineNuxtConfig({
  experimental: { appManifest: false },
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
    "@pinia/nuxt",
    "@nuxtjs/color-mode",
    "nuxt-shiki",
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
  runtimeConfig: {
    public: {
      feedbackFusionEndpoint: "NUXT_PUBLIC_FEEDBACK_FUSION_ENDPOINT",
    },
    clientId: "",
    clientSecret: "",
    oidcDiscovery: "",
    scope: "openid profile",
  },
  vuetify: {
    vuetifyOptions: {
      labComponents: true,
    },
  },
  shiki: {
    bundledLangs: ["html", "hcl"],
    bundledThemes: ["github-dark", "github-light"],
  },
});
