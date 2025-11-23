<template>
  <AuditHistory
    :kind="ProtoResourceKind.TARGET"
    endpoint="Target"
    :breadcrumbs="breadcrumbs"
    :id="route.params.target"
  >
    <template #subtitle="{ id }"> Target {{ id }} </template>
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
    title: "audit",
  },
]);
</script>
