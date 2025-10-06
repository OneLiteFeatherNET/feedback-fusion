<template>
  <v-avatar color="secondary">
    {{ username[0] }}
  </v-avatar>
</template>

<script setup lang="ts">
import { defineProps } from "#imports";

defineProps({
  username: String,
});
</script>
