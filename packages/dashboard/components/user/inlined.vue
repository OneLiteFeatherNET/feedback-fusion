<template>
  <div>
    <UserAvatar :username="username" />

    <span class="ml-4">
      {{ username }}
    </span>
  </div>
</template>

<script setup lang="ts">
defineProps({
  username: String,
});
</script>
