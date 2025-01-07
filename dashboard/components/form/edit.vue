<template>
  <v-dialog v-model="dialog">
    <template #activator="{ props }">
      <slot :props="props" />
    </template>

    <v-card class="mx-auto" style="max-width: 500px; width: 95vw">
      <v-card-title>
        {{ !!title ? title : $t("form.edit") }}
      </v-card-title>

      <v-card-subtitle>
        {{ props.subtitle }}
      </v-card-subtitle>

      <v-card-text>
        <template v-for="field in props.fields" :key="field.name">
          <v-text-field
            v-if="field.type === 'text' && (!field.if || field.if())"
            color="primary"
            outlined
            :label="field.label"
            v-model="data[field.name]"
            :rules="field.required ? [required($t)] : []"
          />

          <v-number-input
            v-if="field.type === 'number' && (!field.if || field.if())"
            color="primary"
            outlined
            :label="field.label"
            v-model="data[field.name]"
            :rules="field.required ? [required($t)] : []"
            :min="field.min"
            :max="field.max"
          />

          <v-textarea
            v-if="field.type === 'textarea' && (!field.if || field.if())"
            :label="field.label"
            outlined
            color="primary"
            v-model="data[field.name]"
            :rules="field.required ? [required($t)] : []"
          />

          <v-select
            v-if="field.type === 'select' && (!field.if || field.if())"
            :label="field.label"
            outlined
            chips
            color="primary"
            v-model="data[field.name]"
            :items="field.items"
            :rules="field.required ? [required($t)] : []"
            :multiple="field.multiple"
          />

          <v-combobox
            v-if="field.type === 'combobox' && (!field.if || field.if())"
            :label="field.label"
            outlined
            chips
            color="primary"
            v-model="data[field.name]"
            :items="field.items"
            :rules="field.required ? [required($t)] : []"
            :multiple="field.multiple"
          />

          <v-switch
            v-if="field.type === 'switch' && (!field.if || field.if())"
            color="primary"
            outlined
            :label="field.label"
            v-model="data[field.name]"
          />
        </template>
      </v-card-text>

      <v-card-actions>
        <v-btn @click="dialog = false" text color="error">
          {{ $t("form.cancel") }}
        </v-btn>

        <v-spacer />

        <v-btn
          @click="execute"
          text
          color="success"
          :disabled="
            disabled(
              $t,
              ...fields
                .filter((field) => field.required && (!field.if || field.if()))
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
  title: String,
});
const emit = defineEmits(["update:modelValue"]);

const data = computed({
  get: () => props.modelValue,
  set: (value) => emit("update:modelValue", value),
});
const dialog = ref(false);

const execute = () => {
  dialog.value = false;

  props.action();
};
</script>
