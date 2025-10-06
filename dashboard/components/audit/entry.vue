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
  </div>
</template>

<script setup lang="ts">
import { defineProps, ref, onMounted } from "#imports";
import { ProtoAuditVersion } from "~/composables/feedback-fusion-v1/audit";

const props = defineProps({
  entry: ProtoAuditVersion,
  isRecursive: {
    type: Boolean,
    default: false,
  },
});

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
    data.value = props.entry;
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
</script>
