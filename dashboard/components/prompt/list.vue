<template>
  <div>
    <h3>{{ $t("prompt.list") }}</h3>

    <v-list>
      <v-list-item
        v-for="prompt in prompts?.prompts"
        :key="prompt.id"
        :title="prompt.title"
        :subtitle="prompt.description"
        link
        :href="localePath(`/target/${props.target}/prompt/${prompt.id}`)"
      />
    </v-list>

    <v-pagination
      v-if="prompts && prompts.total > prompts.pageSize"
      v-model="prompts.pageToken"
    />
  </div>
</template>

<script setup lang="ts">
import { useNuxtApp, ref, onMounted, useLocalePath, watch } from "#imports";
import { useRpcOptions } from "~/composables/grpc";

const props = defineProps({
  target: String,
});

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();

const prompts = ref(undefined);

watch(
  () => prompts.value?.pageToken,
  async (pageToken: number) => {
    prompts.value = await $feedbackFusion
      .getPrompts(
        { pageToken, pageSize: 10, target: props.target },
        useRpcOptions(),
      )
      .then((value) => value.response);
  },
);

onMounted(async () => {
  prompts.value = await $feedbackFusion
    .getPrompts(
      { pageToken: 1, pageSize: 10, target: props.target },
      useRpcOptions(),
    )
    .then((value) => value.response);
});
</script>
