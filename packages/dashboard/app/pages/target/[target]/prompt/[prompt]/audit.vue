<template>
  <AuditHistory
    :kind="ProtoResourceKind.PROMPT"
    endpoint="Prompt"
    :breadcrumbs="breadcrumbs"
    :id="route.params.prompt"
  >
    <template #subtitle="{ id }"> Prompt {{ id }} </template>
  </AuditHistory>
</template>

<script setup lang="ts">
import { ref, useRoute, useLocalePath } from "#imports";
import { ProtoResourceKind } from "~/composables/common/resource";

definePageMeta({
  auth: true,
});

const route = useRoute();
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
  {
    title: "prompt",
  },
  {
    title: route.params.prompt,
    to: localePath(
      `/target/${route.params.target}/prompt/${route.params.prompt}`,
    ),
  },
  {
    title: "audit",
  },
]);
</script>
