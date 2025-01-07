<template>
  <InstanceCard
    endpoint="Prompt"
    :breadcrumbs="breadcrumbs"
    :editFields="editFields"
    :fetch="fetch"
    :delete="deletePrompt"
    :edit="edit"
    :deleteMessage="$t('prompt.delete')"
  >
    <template #title="{ instance }">
      {{ instance?.title }}
    </template>

    <template #subtitle="{ instance }">
      {{ instance?.description }}
    </template>

    <template #default="{ instance }">
      <FieldList
        v-if="instance"
        :target="route.params.target"
        :prompt="instance.id"
      />
    </template>
  </InstanceCard>
</template>

<script setup lang="ts">
import {
  ref,
  useNuxtApp,
  useRoute,
  useI18n,
  useLocalePath,
  useRouter,
} from "#imports";
import { useRpcOptions } from "~/composables/grpc";

const localePath = useLocalePath();
const route = useRoute();
const router = useRouter();
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
  },
]);

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
  {
    name: "active",
    label: t("prompt.active"),
    type: "switch",
  },
]);

const fetch = async () => {
  return await $publicFeedbackFusion
    .getPrompt({ id: route.params.prompt }, useRpcOptions())
    .then((value) => value.response);
};

const deletePrompt = (id) => async () => {
  await $feedbackFusion
    .deletePrompt({ id }, useRpcOptions())
    .then(() => router.push(localePath(`/target/${route.params.target}`)));
};

const edit = (prompt) => async () => {
  return await $feedbackFusion
    .updatePrompt(prompt, useRpcOptions())
    .then((value) => value.response);
};
</script>
