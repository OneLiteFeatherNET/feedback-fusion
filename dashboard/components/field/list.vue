<template>
  <div>
    <h3>{{ $t("field.list") }}</h3>

    <v-list>
      <v-list-item
        v-for="field in fields?.fields"
        :key="field.id"
        :title="field.title"
        :subtitle="$t(`field.type.${field.fieldType}`)"
        link
        :to="
          localePath(
            `/target/${props.target}/prompt/${props.prompt}/field/${field.id}`,
          )
        "
      />
    </v-list>

    <v-pagination
      v-if="fields && fields.total > fields.pageSize"
      v-model="fields.pageToken"
      :length="Math.ceil(fields.total / fields.pageSize)"
    />

    <FormEdit
      v-if="authorization.hasPermission('Field', 'Write')"
      v-model="creation"
      :fields="creationFields"
      :action="create"
      :title="$t('field.create')"
    >
      <template #default="{ props }">
        <v-btn class="mt-4" block color="success" v-bind="props">
          {{ $t("field.create") }}
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
  prompt: String,
});

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();
const { t } = useI18n();
const authorization = useAuthorizationStore();

const fields = ref(undefined);
const creation = ref({});

const creationFields = ref([
  {
    name: "title",
    label: t("field.title"),
    type: "text",
    required: true,
  },
  {
    name: "description",
    label: t("field.description"),
    type: "textarea",
  },
  {
    name: "fieldType",
    label: t("field.fieldType"),
    required: true,
    type: "select",
    items: [
      {
        title: t("field.type.0"),
        value: 0,
      },
      {
        title: t("field.type.1"),
        value: 1,
      },
      {
        title: t("field.type.2"),
        value: 2,
      },
      {
        title: t("field.type.3"),
        value: 3,
      },
      {
        title: t("field.type.4"),
        value: 4,
      },
      {
        title: t("field.type.5"),
        value: 5,
      },
    ],
  },
]);

const fetchPage = async (pageToken: number) => {
  fields.value = await $feedbackFusion
    .getFields(
      { pageToken, pageSize: 10, prompt: props.prompt },
      useRpcOptions(),
    )
    .then((value) => value.response);
};

watch(
  () => fields.value?.pageToken,
  async (pageToken: number) => {
    await fetchPage(pageToken);
  },
);

onMounted(async () => {
  await fetchPage(1);
});

const create = async () => {
  await $feedbackFusion
    .createField({ ...creation.value, prompt: props.prompt }, useRpcOptions())
    .then((value) => value.response);

  await fetchPage(1);
  creation.value = {};
};
</script>
