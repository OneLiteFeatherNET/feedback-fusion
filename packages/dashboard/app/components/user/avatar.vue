<template>
  <v-avatar color="secondary">
    {{ username[0] }}
  </v-avatar>
</template>

<script setup lang="ts">
defineProps({
  username: String,
});
</script>
