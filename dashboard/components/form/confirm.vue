<template>
  <v-dialog v-model="dialog">
    <template #activator="{ props }">
      <slot :props="props" />
    </template>

    <v-card class="mx-auto" style="max-width: 500px">
      <v-card-title>
        {{ $t("form.confirm") }}
      </v-card-title>

      <v-card-text>
        {{ props.message }}
      </v-card-text>

      <v-card-actions>
        <v-btn @click="dialog = false" text>
          {{ $t("form.cancel") }}
        </v-btn>

        <v-spacer />

        <v-btn @click="execute" text color="error">
          {{ $t("form.confirm") }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { defineProps, ref } from "#imports";

const props = defineProps({
  action: Function,
  message: String,
});

const dialog = ref(false);

const execute = () => {
  dialog.value = false;

  props.action();
};
</script>
