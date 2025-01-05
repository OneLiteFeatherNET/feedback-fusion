<template>
  <div>
    <h2>{{ $t("dashboard.targets") }}</h2>

    <v-list>
      <v-list-item
        v-for="target in targets?.targets"
        :key="target.id"
        :title="target.name"
        :subtitle="target.description"
        link
        :href="localePath(`/target/${target.id}`)"
      />
    </v-list>

    <v-pagination
      v-if="targets && targets.total > targets.pageSize"
      v-model="targets.pageToken"
    />
  </div>
</template>

<script setup lang="ts">
import { useNuxtApp, ref, onMounted, useLocalePath, watch } from "#imports";
import { useRpcOptions } from "~/composables/grpc";

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();

const targets = ref(undefined);

watch(
  () => targets.value?.pageToken,
  async (pageToken: number) => {
    targets.value = await $feedbackFusion
      .getTargets({ pageToken, pageSize: 10 }, useRpcOptions())
      .then((value) => value.response);
  },
);

onMounted(async () => {
  targets.value = await $feedbackFusion
    .getTargets({ pageToken: 1, pageSize: 10 }, useRpcOptions())
    .then((value) => value.response);
});
</script>
