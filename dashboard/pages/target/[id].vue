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
      <v-btn
        v-if="authorization.hasPermission('Target', 'Write')"
        color="primary"
        text
      >
        {{ $t("form.edit") }}
      </v-btn>

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
import { useRouter, onMounted, ref, useNuxtApp, useRoute } from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";
import { useRpcOptions } from "~/composables/grpc";

const authorization = useAuthorizationStore();
const router = useRouter();
const route = useRoute();
const { $feedbackFusion } = useNuxtApp();

const target = ref(undefined);

onMounted(async () => {
  await authorization.fetch();

  if (!authorization.hasPermission("Target", "Read")) {
    return router.push("/");
  }

  target.value = await $feedbackFusion
    .getTarget({ id: route.params.id }, useRpcOptions())
    .then((value) => value.response);
});

const deleteTarget = (id: number) => async () => {
  await $feedbackFusion.deleteTarget({ id }, useRpcOptions());
};
</script>
