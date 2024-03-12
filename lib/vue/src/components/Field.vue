<template>
  <div class="feedback-fusion__field">
    <div v-if="props.type === 'text'" class="feedback-fusion__field__text">
      <input v-model="data" type="text">
    </div>
  </div>
</template>

<script setup lang="ts">
import { FeedbackPromptField } from '@onelitefeathernet/feedback-fusion-core';
import { computed } from 'vue';

interface FieldProps extends FeedbackPromptField {
  value: any;
}

const props = defineProps<FieldProps>();
const emit = defineEmits(["update"]);

const data = computed({
  get() {
    return props.value;
  },
  set(value: any) {
    emit("update", value);
  }
})
</script>

<style lang="scss" scoped>
.feedback-fusion__field {}
</style>
