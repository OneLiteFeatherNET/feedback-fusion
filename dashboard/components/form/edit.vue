<template>
  <v-dialog v-model="dialog">
    <template #activator="{ props }">
      <slot :props="props" />
    </template>

    <v-card class="mx-auto" style="max-width: 500px; width: 95vw">
      <v-card-title>
        {{ $t("form.edit") }}
      </v-card-title>

      <v-card-subtitle>
        {{ props.subtitle }}
      </v-card-subtitle>

      <v-card-text>
        <template v-for="field in props.fields" :key="field.name">
          <v-text-field
            v-if="field.type === 'text'"
            color="primary"
            outlined
            :label="field.label"
            v-model="data[field.name]"
            :rules="field.required ? [required($t)] : []"
          />

          <v-textarea
            v-if="field.type === 'textarea'"
            :label="field.label"
            outlined
            color="primary"
            v-model="data[field.name]"
            :rules="field.required ? [required($t)] : []"
          />
        </template>
      </v-card-text>

      <v-card-actions>
        <v-btn @click="dialog = false" text>
          {{ $t("form.cancel") }}
        </v-btn>

        <v-spacer />

        <v-btn
          @click="execute"
          text
          color="error"
          :disabled="
            disabled(
              $t,
              ...fields
                .filter((field) => field.required)
                .map((field) => data[field.name]),
            )
          "
        >
          {{ $t("form.save") }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { defineProps, ref, computed } from "#imports";
import { disabled } from "~/composables/form";

const props = defineProps({
  action: Function,
  subtitle: String,
  modelValue: Object,
  fields: Array,
});
const emit = defineEmits(["update:modelValue"]);

const data = computed({
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});
const dialog = ref(false);

const execute = () => {
  console.log(
    disabled(
      props.fields
        .filter((field) => field.required)
        .map((field) => data.value[field.name]),
    ),
  );
  dialog.value = false;

  props.action();
};
</script>
