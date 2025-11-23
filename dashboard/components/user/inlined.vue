<template>
  <div>
    <UserAvatar :username="username" />

    <span class="ml-4">
      {{ username }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from "#imports";

defineProps({
  username: String,
});
</script>
