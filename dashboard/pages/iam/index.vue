<template>
  <div>
    <h2>{{ $t("iam.title") }}</h2>

    <v-list v-if="authorizations && authorizations.total > 1" class="mt-4">
      <v-list-item
        v-for="target in targets?.targets"
        :key="target.id"
        :title="target.name"
        :subtitle="target.description"
        link
        :to="localePath(`/target/${target.id}`)"
      >
        <template #append>
          <v-checkbox
            class="d-flex justify-center align-center align-items-center"
            color="primary"
            v-model="selected"
            :value="target.id"
            label=""
            @click.stop="() => {}"
          />
        </template>
      </v-list-item>
    </v-list>

    <v-pagination
      v-if="authorizations && authorizations.total > authorizations.pageSize"
      v-model="authorizations.pageToken"
      :length="Math.ceil(authorizations.total / authorizations.pageSize)"
    />

    <FormEdit
      v-if="authorization.hasPermission('Authorize', 'Write')"
      v-model="creation"
      :fields="creationFields"
      :action="create"
      :title="$t('iam.create')"
    >
      <template #default="{ props }">
        <v-btn class="mt-4" block color="success" v-bind="props">
          {{ $t("iam.create") }}
        </v-btn>
      </template>
    </FormEdit>

    <v-btn
      v-if="
        authorization.hasPermission('Authorize', 'Read') && selected.length > 0
      "
      class="float-right mt-4"
      variant="text"
      color="success"
      @click="
        router.push({
          path: localePath('/iam/export'),
          query: { ids: selected },
        })
      "
    >
      {{ $t("dashboard.export") }}
    </v-btn>
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
  useRouter,
} from "#imports";
import { useRpcOptions } from "~/composables/grpc";
import { useAuthorizationStore } from "~/composables/authorization";

const { $feedbackFusion } = useNuxtApp();
const localePath = useLocalePath();
const { t } = useI18n();
const authorization = useAuthorizationStore();
const router = useRouter();

const authorizations = ref(undefined);
const creation = ref({});
const selected = ref([]);

const creationFields = ref([
  {
    name: "resource_kind",
    label: t("iam.authorization.resource_kind.label"),
    type: "select",
    required: true,
    items: Array.from({ length: 6 }, (_, i) => ({
      title: t(`iam.authorization.resource_kind.${i}`),
      value: i,
    })),
  },
  {
    name: "authorization_data.type",
    label: t("iam.authorization.authorization_data.type.label"),
    type: "select",
    required: true,
    items: Array.from({ length: 3 }, (_, i) => ({
      title: t(`iam.authorization.authorization_data.type.${i}`),
      value: i,
    })),
  },
  {
    name: "authorization_data.grant",
    label: t("iam.authorization.authorization_data.grant.label"),
    type: "select",
    required: true,
    items: Array.from({ length: 4 }, (_, i) => ({
      title: t(`iam.authorization.authorization_data.grant.${i}`),
      value: i,
    })),
    multiple: true
  },
]);

const fetchPage = async (pageToken: number) => {
  authorizations.value = await $feedbackFusion
    .getResourceAuthorizations({ pageToken, pageSize: 10 }, useRpcOptions())
    .then((value) => value.response);
};

watch(
  () => authorizations.value?.pageToken,
  async (pageToken: number) => {
    await fetchPage(pageToken);
  },
);

onMounted(async () => {
  await fetchPage(1);
});

const create = async () => {
  await $feedbackFusion
    .createResourceAuthorization(creation.value, useRpcOptions())
    .then((value) => value.response);

  await fetchPage(1);
  creation.value = {};
};
</script>
