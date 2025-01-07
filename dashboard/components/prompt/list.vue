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
        :to="localePath(`/target/${props.target}/prompt/${prompt.id}`)"
      />
    </v-list>

    <v-pagination
      v-if="prompts && prompts.total > prompts.pageSize"
      v-model="prompts.pageToken"
      :length="Math.ceil(prompts.total / prompts.pageSize)"
    />

    <FormEdit
      v-if="authorization.hasPermission('Prompt', 'Write')"
      v-model="creation"
      :fields="creationFields"
      :action="create"
      :title="$t('prompt.create')"
    >
      <template #default="{ props }">
        <v-btn class="mt-4" block color="success" v-bind="props">
          {{ $t("prompt.create") }}
        </v-btn>
      </template>
    </FormEdit>
  </div>
</template>

<script setup lang="ts">
import {
  useNuxtApp,
  ref,
  onMounted,
  useLocalePath,
  watch,
  useI18n,
} from "#imports";
import { useRpcOptions } from "~/composables/grpc";
import { useAuthorizationStore } from "~/composables/authorization";

const props = defineProps({
  target: String,
});

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();
const { t } = useI18n();
const authorization = useAuthorizationStore();

const prompts = ref(undefined);
const creation = ref({});

const creationFields = ref([
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

const fetchPage = async (pageToken: number) => {
  prompts.value = await $feedbackFusion
    .getPrompts(
      { pageToken, pageSize: 10, target: props.target },
      useRpcOptions(),
    )
    .then((value) => value.response);
};

watch(
  () => prompts.value?.pageToken,
  async (pageToken: number) => {
    await fetchPage(pageToken);
  },
);

onMounted(async () => {
  await fetchPage(1);
});

const create = async () => {
  await $feedbackFusion
    .createPrompt({ ...creation.value, target: props.target }, useRpcOptions())
    .then((value) => value.response);

  await fetchPage(1);
  creation.value = {};
};
</script>
