<template>
  <div>
    <v-breadcrumbs :items="breadcrumbs" />

    <v-card :loading="!prompt">
      <v-card-title>
        {{ prompt?.title }}
      </v-card-title>

      <v-card-subtitle>
        {{ prompt?.description }}
      </v-card-subtitle>

      <v-card-text class="mt-4"> </v-card-text>

      <v-card-actions v-if="prompt">
        <FormEdit
          v-if="authorization.hasPermission('Prompt', 'Write')"
          v-model="editPrompt"
          :fields="editFields"
          :action="save"
          :subtitle="prompt.id"
        >
          <template #default="{ props }">
            <v-btn color="primary" text v-bind="props">
              {{ $t("form.edit") }}
            </v-btn>
          </template>
        </FormEdit>

        <v-spacer />

        <FormConfirm
          v-if="authorization.hasPermission('Prompt', 'Write')"
          :message="$t('prompt.delete', { name: prompt.name })"
          :action="deletePrompt(prompt.id)"
        >
          <template #default="{ props }">
            <v-btn v-bind="props" color="error" text>
              {{ $t("form.delete") }}
            </v-btn>
          </template>
        </FormConfirm>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import {
  useRouter,
  onMounted,
  ref,
  useNuxtApp,
  useRoute,
  useI18n,
  useLocalePath,
} from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";
import { useRpcOptions } from "~/composables/grpc";

const localePath = useLocalePath();
const authorization = useAuthorizationStore();
const router = useRouter();
const route = useRoute();
const { $feedbackFusion, $publicFeedbackFusion } = useNuxtApp();
const { t } = useI18n();

const breadcrumbs = ref([
  {
    title: "target",
    to: localePath("/"),
  },
  {
    title: route.params.target,
    to: localePath(`/target/${route.params.target}`),
  },
  {
    title: "prompt",
  },
  {
    title: route.params.prompt,
    to: localePath(
      `/target/${route.params.target}/prompt/${route.params.prompt}`,
    ),
    disabled: false,
  },
]);
const prompt = ref(undefined);
const editPrompt = ref(undefined);

const editFields = ref([
  {
    name: "title",
    label: t("prompt.title"),
    type: "text",
    required: true,
  },
  {
    name: "description",
    label: t("prompt.description"),
    type: "textarea",
    required: true,
  },
]);

onMounted(async () => {
  await authorization.fetch();

  if (!authorization.hasPermission("Prompt", "Read")) {
    return router.push("/");
  }

  prompt.value = await $publicFeedbackFusion
    .getPrompt({ id: route.params.prompt }, useRpcOptions())
    .then((value) => value.response);

  editPrompt.value = JSON.parse(JSON.stringify(prompt.value));
});

const deletePrompt = (id: number) => async () => {
  await $feedbackFusion.deletePrompt({ id }, useRpcOptions());
};

const save = async () => {
  await $feedbackFusion
    .updatePrompt(editPrompt.value, useRpcOptions())
    .then((value) => (prompt.value = value.response));
};
</script>
