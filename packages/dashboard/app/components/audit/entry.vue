<template>
  <div>
    <v-list>
      <v-list-item
        v-for="(field, index) in listableFields"
        :key="index"
        :title="'' + data[field]"
        :subtitle="field"
      />
    </v-list>

    <v-expansion-panels>
      <v-expansion-panel
        color="secondary"
        v-for="(key, index) in toRecurse"
        :key="index"
        :title="key"
      >
        <v-expansion-panel-text>
          <AuditEntry :entry="data[key]" is-recursive />
        </v-expansion-panel-text>
      </v-expansion-panel>
    </v-expansion-panels>

    <FormConfirm
      :message="t('audit.confirmRollback')"
      :action="rollback"
      v-if="!isRecursive && authorization.hasPermission(endpoint, 'Write')"
    >
      <template #default="{ props }">
        <v-btn
          color="warning"
          variant="text"
          v-bind="props"
          class="mt-12 float-right"
        >
          {{ $t("audit.rollback") }}
        </v-btn>
      </template>
    </FormConfirm>
  </div>
</template>

<script setup lang="ts">
import {
  ref,
  onMounted,
  useI18n,
  useNuxtApp,
} from "#imports";
import { ProtoAuditVersion } from "~/composables/feedback-fusion-v1/audit";
import { useRpcOptions } from "~/composables/grpc";
import { useAuthorizationStore } from "~/composables/authorization";

const props = defineProps({
  entry: ProtoAuditVersion,
  isRecursive: {
    type: Boolean,
    default: false,
  },
  endpoint: String,
});
const emit = defineEmits(["rollback"]);

const { t } = useI18n();
const { $feedbackFusion } = useNuxtApp();
const authorization = useAuthorizationStore();

const listableFields = ref([]);
const toRecurse = ref([]);
const data = ref(undefined);

onMounted(() => {
  if (!props.isRecursive) {
    // thats the type
    const type = props.entry.data.inner.oneofKind;
    // load the data
    data.value = props.entry.data.inner[type];
  } else {
    // as we could be stuck in an enum variant (which we dont want to fully expand) we have to check wether we have only 1 value
    // and wether this value does contain variant fixing
    let current = props.entry;
    const keys = Object.keys(current);
    if (keys.length === 1) {
      // check wether the subobject is a variant
      const capsulated = current[keys[0]];
      const oneofKind = capsulated.oneofKind;

      if (oneofKind !== undefined) {
        current = capsulated[oneofKind];
      }
    }

    data.value = current;
  }

  // collect all the keys from the data
  const keys = Object.keys(data.value);

  // now we will determine the listable fields, these are all fields which have a atomic datatype
  listableFields.value = keys.filter(
    (key) =>
      typeof data.value[key] !== "object" || Array.isArray(data.value[key]),
  );

  // everything that is not in listable fields will have a recursion
  toRecurse.value = keys.filter(
    (key) =>
      !listableFields.value.includes(key) &&
      // we ignore these
      !["createdAt", "updatedAt"].includes(key),
  );
});

const rollback = async () => {
  await $feedbackFusion
    .rollbackResource(props.entry, await useRpcOptions())
    .then(() => emit("rollback"));
};
</script>
