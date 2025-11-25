<template>
  <div>
    <v-breadcrumbs :items="breadcrumbs" />

    <v-card :loading="!instance">
      <v-card-title>
        <slot name="title" :instance="instance" />
      </v-card-title>

      <v-card-subtitle>
        <slot name="subtitle" :instance="instance" />
      </v-card-subtitle>

      <v-card-text class="mt-4">
        <slot :instance="instance" />
      </v-card-text>

      <v-card-actions v-if="instance">
        <FormEdit
          v-if="
            editFields &&
            editFields.length > 0 &&
            authorization.hasPermission(endpoint, 'Write')
          "
          v-model="editInstance"
          :fields="editFields"
          :action="save"
          :subtitle="instance.id"
        >
          <template #default="{ props }">
            <v-btn color="primary" text v-bind="props">
              {{ $t("form.edit") }}
            </v-btn>
          </template>
        </FormEdit>

        <v-btn color="warning" text :to="localePath(`${route.path}/audit`)">
          {{ $t("form.audit") }}
        </v-btn>

        <v-spacer />

        <FormConfirm
          v-if="
            !!deleteMessage && authorization.hasPermission(endpoint, 'Write')
          "
          :message="deleteMessage"
          :action="props.delete(instance.id)"
        >
          <template #default="{ props }">
            <v-btn v-bind="props" color="error" text>
              {{ $t("form.delete") }}
            </v-btn>
          </template>
        </FormConfirm>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import {
  onMounted,
  ref,
  defineProps,
  useRouter,
  useLocalePath,
  useRoute,
} from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";

const props = defineProps({
  breadcrumbs: Array,
  editFields: Array,
  fetch: Function,
  delete: Function,
  edit: Function,
  endpoint: String,
  deleteMessage: String,
});

const authorization = useAuthorizationStore();
const router = useRouter();
const localePath = useLocalePath();
const route = useRoute();

const instance = ref(undefined);
const editInstance = ref(undefined);

onMounted(async () => {
  await authorization.fetch();

  if (!authorization.hasPermission(props.endpoint, "Read")) {
    return router.push("/");
  }

  instance.value = await props.fetch().then((value) => value);

  editInstance.value = JSON.parse(JSON.stringify(instance.value));
});

const save = async () => {
  await props
    .edit(editInstance.value)()
    .then((value) => (instance.value = value));
};
</script>
