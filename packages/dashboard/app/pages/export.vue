<template>
  <v-card :loading="!exported">
    <v-card-title>
      {{ $t("export.title") }}
    </v-card-title>

    <v-card-text v-if="exported">
      <Shiki lang="hcl" :code="exported" :highlightOptions="highlightOptions" />
    </v-card-text>

    <v-card-actions v-if="exported">
      <v-spacer />
      <v-btn color="success" variant="text" @click="copy">
        {{ $t("form.copy") }}
      </v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import {
  useNuxtApp,
  useRoute,
  onMounted,
  ref,
  useRouter,
  computed,
} from "#imports";
import { useRpcOptions } from "~/composables/grpc";
import { useAuthorizationStore } from "~/composables/authorization";
import { useTheme } from "vuetify";
import clipboard from "clipboardy";

definePageMeta({
  auth: true,
});

const authorization = useAuthorizationStore();
const { $feedbackFusion } = useNuxtApp();
const route = useRoute();
const router = useRouter();
const theme = useTheme();

const exported = ref(undefined);
const highlightOptions = computed(() => ({
  theme: `github-${theme.global.name.value}`,
}));

onMounted(async () => {
  if (!authorization.hasPermission("Export", "Read") || !route.query.targets) {
    return router.push("/");
  }

  exported.value = await $feedbackFusion
    .exportData({ targets: route.query.targets }, await useRpcOptions())
    .then((value) => value.response.export);
});

const copy = async () => {
  await clipboard.write(exported.value);
};
</script>
