<template>
  <AuditHistory
    :kind="ProtoResourceKind.FIELD"
    endpoint="Field"
    :breadcrumbs="breadcrumbs"
    :id="route.params.field"
  >
    <template #subtitle="{ id }"> Field {{ id }} </template>
  </AuditHistory>
</template>

<script setup lang="ts">
import { ref, useRoute, useLocalePath } from "#imports";
import { ProtoResourceKind } from "~/composables/common/resource";

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
    title: "field",
  },
  {
    title: route.params.field,
  },
  {
    title: "audit",
  },
]);
</script>
