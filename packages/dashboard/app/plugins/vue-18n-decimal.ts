// workaround https://github.com/vuetifyjs/nuxt-module/issues/324
import { computed } from "vue";
import { defineNuxtPlugin } from "#imports";
import type { useI18n } from "vue-i18n";

function inferDecimalSeparator(n: ReturnType<typeof useI18n>["n"]) {
  return n(0.1).includes(",") ? "," : ".";
}

export default defineNuxtPlugin({
  name: "app:vuetify-i18n-decimal",
  // make sure vue-i18n is ready
  // @ts-expect-error: plugin name not typed
  dependsOn: ["i18n:plugin"],
  setup(nuxtApp) {
    const i18n: ReturnType<typeof useI18n> = nuxtApp.$i18n;
    nuxtApp.hook("vuetify:configuration", ({ vuetifyOptions }) => {
      // if module hasn't configured locale yet, nothing to do
      if (!vuetifyOptions.locale || !vuetifyOptions.locale.adapter) return;

      // override / add decimalSeparator on the existing adapter
      vuetifyOptions.locale.adapter.decimalSeparator = computed(() =>
        inferDecimalSeparator(i18n.n),
      );
    });
  },
});
