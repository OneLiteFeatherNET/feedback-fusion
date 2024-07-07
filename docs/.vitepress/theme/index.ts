import DefaultTheme from "vitepress/theme";

export default {
  ...DefaultTheme,
  async enhanceApp() {
    if (!import.meta.env.SSR) {
      const handler = await import("../../mock");
      const { setupWorker } = await import("msw/browser");
      setupWorker(...handler.default).start({ serviceWorker: { url: "/feedback-fusion/mockServiceWorker.js" }, onUnhandledRequest: "bypass" }).catch(console.log);
    };
  },
};
