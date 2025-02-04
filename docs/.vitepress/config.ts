import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "FeedbackFusion",
  description: "A VitePress Site",
  base: "/feedback-fusion/",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Docs", link: "/docs" }
    ],

    sidebar: [
      {
        text: "Docs",
        items: [
          {
            text: "Configuration", items: [
              { text: "Server", link: "/docs/configuration/server" },
              { text: "Dashboard", link: "/docs/configuration/dashboard" }
            ]
          },
          {
            text: "Deployment", items: [
              { text: "Helm", link: "/docs/deployment/helm" },
              { text: "Docker", link: "/docs/deployment/docker" }
            ]
          },
          {
            text: "Observability", items: [
              { text: "Logging", link: "/docs/observability/logging" },
              { text: "Tracing", link: "/docs/observability/tracing" }
            ]
          },
          { text: "Caching", link: "/docs/caching" },
          {
            text: "Frontend Library", link: "/docs/frontend-library", items: [
              { text: "Prompt", link: "/docs/frontend-library/prompt" },
              { text: "Theming", link: "/docs/frontend-library/theming" }
            ]
          },
          {
            text: "Reference", items: [
              {
                text: "API", link: "/docs/reference/api"
              }
            ]
          }
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/onelitefeathernet/feedback-fusion' }
    ]
  },
  vue: {
    template: {
      compilerOptions: {
        isCustomElement: (name) => ["feedback-fusion-prompt"].includes(name.toLowerCase())
      }
    }
  },
  vite: {
    build: {
      target: "esnext"
    }
  }
})
