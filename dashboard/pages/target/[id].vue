<template>
  <v-card :loading="!target">
    <v-card-title>
      {{ target?.name }}
    </v-card-title>

    <v-card-subtitle>
      {{ target?.description }}
    </v-card-subtitle>

    <v-card-text class="mt-4">
      <PromptList v-if="target" :target="target.id" />
    </v-card-text>

    <v-card-actions v-if="target">
      <FormEdit
        v-if="authorization.hasPermission('Target', 'Write')"
        v-model="editTarget"
        :fields="editFields"
        :action="save"
      >
        <template #default="{ props }">
          <v-btn color="primary" text v-bind="props">
            {{ $t("form.edit") }}
          </v-btn>
        </template>
      </FormEdit>

      <v-spacer />

      <FormConfirm
        v-if="authorization.hasPermission('Target', 'Write')"
        :message="$t('target.delete', { name: target.name })"
        :action="deleteTarget(target.id)"
      >
        <template #default="{ props }">
          <v-btn v-bind="props" color="error" text>
            {{ $t("form.delete") }}
          </v-btn>
        </template>
      </FormConfirm>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import {
  useRouter,
  onMounted,
  ref,
  useNuxtApp,
  useRoute,
  useI18n,
} from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";
import { useRpcOptions } from "~/composables/grpc";

const authorization = useAuthorizationStore();
const router = useRouter();
const route = useRoute();
const { $feedbackFusion } = useNuxtApp();
const { t } = useI18n();

const target = ref(undefined);
const editTarget = ref(undefined);

const editFields = ref([
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

onMounted(async () => {
  await authorization.fetch();

  if (!authorization.hasPermission("Target", "Read")) {
    return router.push("/");
  }

  target.value = await $feedbackFusion
    .getTarget({ id: route.params.id }, useRpcOptions())
    .then((value) => value.response);

  editTarget.value = JSON.parse(JSON.stringify(target.value));
});

const deleteTarget = (id: number) => async () => {
  await $feedbackFusion.deleteTarget({ id }, useRpcOptions());
};

const save = async () => {
  await $feedbackFusion
    .updateTarget(editTarget.value, useRpcOptions())
    .then((value) => (target.value = value.response));
};
</script>
