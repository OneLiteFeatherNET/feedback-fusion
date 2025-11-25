<template>
  <InstanceCard
    endpoint="Target"
    :breadcrumbs="breadcrumbs"
    :editFields="editFields"
    :fetch="fetch"
    :delete="deleteTarget"
    :edit="edit"
    :deleteMessage="$t('target.delete')"
  >
    <template #title="{ instance }">
      {{ instance?.name }}
    </template>

    <template #subtitle="{ instance }">
      {{ instance?.description }}
    </template>

    <template #default="{ instance }">
      <PromptList v-if="instance" :target="instance.id" />
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

const route = useRoute();
const router = useRouter();
const { $feedbackFusion } = useNuxtApp();
const { t } = useI18n();
const localePath = useLocalePath();

const breadcrumbs = ref([
  {
    title: "target",
    to: localePath("/"),
  },
  {
    title: route.params.target,
    to: localePath(`/target/${route.params.target}`),
  },
]);

const editFields = ref([
  {
    name: "name",
    label: t("target.name"),
    type: "text",
    required: true,
  },
  {
    name: "description",
    label: t("target.description"),
    type: "textarea",
  },
]);

const fetch = async () => {
  return await $feedbackFusion
    .getTarget({ id: route.params.target }, useRpcOptions())
    .then((value) => value.response);
};

const deleteTarget = (id) => async () => {
  await $feedbackFusion
    .deleteTarget({ id }, useRpcOptions())
    .then(() => router.push("/"));
};

const edit = (target) => async () => {
  return await $feedbackFusion
    .updateTarget(target, useRpcOptions())
    .then((value) => value.response);
};
</script>
