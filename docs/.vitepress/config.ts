import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "FeedbackFusion",
  description: "A VitePress Site",
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
            text: "Frontend Library", link: "/docs/frontend-library", items: [
              { text: "Prompt", link: "/docs/frontend-library/prompt" },
              { text: "Theming", link: "/docs/frontend-library/theming" }
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
  }
})
