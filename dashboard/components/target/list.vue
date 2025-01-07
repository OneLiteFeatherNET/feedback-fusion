<template>
  <div>
    <h2>{{ $t("dashboard.targets") }}</h2>

    <v-list class="mt-4">
      <v-list-item
        v-for="target in targets?.targets"
        :key="target.id"
        :title="target.name"
        :subtitle="target.description"
        link
        :to="localePath(`/target/${target.id}`)"
      />
    </v-list>

    <v-pagination
      v-if="targets && targets.total > targets.pageSize"
      v-model="targets.pageToken"
      :length="Math.ceil(targets.total / targets.pageSize)"
    />

    <FormEdit
      v-if="authorization.hasPermission('Target', 'Write')"
      v-model="creation"
      :fields="creationFields"
      :action="create"
      :title="$t('target.create')"
    >
      <template #default="{ props }">
        <v-btn class="mt-4" block color="success" v-bind="props">
          {{ $t("target.create") }}
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

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();
const { t } = useI18n();
const authorization = useAuthorizationStore();

const targets = ref(undefined);
const creation = ref({});

const creationFields = ref([
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

const fetchPage = async (pageToken: number) => {
  targets.value = await $feedbackFusion
    .getTargets({ pageToken, pageSize: 10 }, useRpcOptions())
    .then((value) => value.response);
};

watch(
  () => targets.value?.pageToken,
  async (pageToken: number) => {
    await fetchPage(pageToken);
  },
);

onMounted(async () => {
  await fetchPage(1);
});

const create = async () => {
  await $feedbackFusion
    .createTarget(creation.value, useRpcOptions())
    .then((value) => value.response);

  await fetchPage(1);
  creation.value = {};
};
</script>
