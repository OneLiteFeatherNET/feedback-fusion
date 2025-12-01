<template>
  <div>
    <h3>{{ $t("field.list") }}</h3>

    <v-list>
      <template v-if="authorization.hasPermission('Field', 'Write')">
        <FormEdit
          v-for="field in fields?.fields"
          :key="field.id"
          v-model="creation"
          :fields="editFields"
          :action="edit"
        >
          <template #default="{ props }">
            <v-list-item
              v-bind="props"
              @click="editField(field)"
              :title="field.title"
              :subtitle="$t(`field.type.${field.fieldType}`)"
            >
              <template #append>
                <v-icon
                  class="mr-4"
                  icon="mdi-history"
                  color="warning"
                  @click.stop="
                    router.push(
                      localePath(`${route.path}/field/${field.id}/audit`),
                    )
                  "
                />

                <FormConfirm
                  :message="t('field.delete')"
                  :action="deleteField(field.id)"
                >
                  <template #default="{ props }">
                    <v-icon color="error" icon="mdi-delete" v-bind="props" />
                  </template>
                </FormConfirm>
              </template>
            </v-list-item>
          </template>
        </FormEdit>
      </template>

      <v-list-item
        v-else
        v-for="field in fields?.fields"
        :key="field.id"
        :title="field.title"
        :subtitle="$t(`field.type.${field.fieldType}`)"
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
        <v-btn
          @click="creation = {}"
          class="mt-4"
          block
          color="success"
          v-bind="props"
        >
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
  watch,
  useI18n,
  useRoute,
  useLocalePath,
  useRouter,
} from "#imports";
import { useRpcOptions } from "~/composables/grpc";
import { useAuthorizationStore } from "~/composables/authorization";
import { numberToKind } from "~/composables/convert";
import { ProtoFieldType } from "~/composables/feedback-fusion-v1/field";

const props = defineProps({
  target: String,
  prompt: String,
});

const { $feedbackFusion } = useNuxtApp();
const { t } = useI18n();
const authorization = useAuthorizationStore();
const route = useRoute();
const localePath = useLocalePath();
const router = useRouter();

const fields = ref(undefined);
const creation = ref({});

const editFields = ref([
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
    name: "options.lines",
    label: t("field.options.lines"),
    required: true,
    type: "number",
    min: 1,
    max: 255,
    if: () => creation.value.fieldType === ProtoFieldType.TEXT,
  },
  {
    name: "options.placeholder",
    label: t("field.options.placeholder"),
    required: true,
    type: "text",
    if: () =>
      creation.value.fieldType === ProtoFieldType.NUMBER ||
      creation.value.fieldType === ProtoFieldType.TEXT,
  },
  {
    name: "options.min",
    label: t("field.options.min"),
    type: "number",
    required: true,
    if: () =>
      creation.value.fieldType === ProtoFieldType.NUMBER ||
      creation.value.fieldType === ProtoFieldType.RANGE,
  },
  {
    name: "options.max",
    label: t("field.options.max"),
    type: "number",
    if: () =>
      creation.value.fieldType === ProtoFieldType.NUMBER ||
      creation.value.fieldType === ProtoFieldType.RANGE ||
      creation.value.fieldType === ProtoFieldType.RATING,
  },
  {
    name: "options.defaultState",
    label: t("field.options.defaultState"),
    type: "switch",
    if: () => creation.value.fieldType === ProtoFieldType.CHECKBOX,
  },
  {
    name: "options.style",
    label: t("field.options.style"),
    type: "select",
    required: true,
    if: () => creation.value.fieldType === ProtoFieldType.CHECKBOX,
    items: Array.from({ length: 2 }, (_, i) => ({
      title: t(`field.checkboxStyle.${i}`),
      value: i,
    })),
  },
  {
    name: "options.multiple",
    label: t("field.options.multiple"),
    type: "switch",
    if: () => creation.value.fieldType === ProtoFieldType.SELECTION,
  },
  {
    name: "options.combobox",
    label: t("field.options.combobox"),
    type: "switch",
    if: () => creation.value.fieldType === ProtoFieldType.SELECTION,
  },
  {
    name: "options.values",
    label: t("field.options.values"),
    type: "combobox",
    multiple: true,
    items: [],
    if: () => creation.value.fieldType === ProtoFieldType.SELECTION,
  },
]);

const creationFields = ref(
  [
    {
      name: "fieldType",
      label: t("field.fieldType"),
      required: true,
      type: "select",
      items: Array.from({ length: 6 }, (_, i) => ({
        title: t(`field.type.${i}`),
        value: i,
      })),
    },
  ].concat(editFields.value),
);

const fetchPage = async (pageToken: number) => {
  fields.value = await $feedbackFusion
    .getFields(
      { pageToken, pageSize: 10, prompt: props.prompt },
      await useRpcOptions(),
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

const convert = () => {
  const kind = numberToKind(creation.value.fieldType);
  creation.value.options = {
    options: {
      oneofKind: kind,
    },
  };
  creation.value.options.options[kind] = {};
  const keys = Object.keys(creation.value);

  keys
    .filter((key) => key.startsWith("options."))
    .forEach((key) => {
      creation.value.options.options[kind][key.split("options.")[1]] =
        creation.value[key];

      delete creation.value[key];
    });
};

const create = async () => {
  convert();

  await $feedbackFusion.createField(
    {
      ...creation.value,
      prompt: props.prompt,
    },
    await useRpcOptions(),
  );

  await fetchPage(1);
  creation.value = {};
};

const editField = (field: any) => {
  creation.value = JSON.parse(JSON.stringify(field));

  const keys = Object.keys(
    creation.value.options.options[numberToKind(creation.value.fieldType)],
  );
  keys.forEach(
    (key) =>
      (creation.value[`options.${key}`] =
        creation.value.options.options[numberToKind(creation.value.fieldType)][
          key
        ]),
  );
};

const edit = async () => {
  convert();

  await $feedbackFusion.updateField(creation.value, await useRpcOptions());

  await fetchPage(1);
  creation.value = {};
};

const deleteField = (id: string) => async () => {
  await $feedbackFusion.deleteField({ id }, await useRpcOptions());

  await fetchPage(1);
};
</script>
