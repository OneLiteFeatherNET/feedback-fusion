<template>
  <div class="feedback-fusion__field">
    <div class="feedback-fusion__field-title">
      {{ props.title }}
    </div>

    <input v-if="props.type === 'text'" v-model="data" type="text" :placeholder="props.options.placeholder">

    <div class="feedback-fusion__field-description">
      {{ props.options.description }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { FeedbackFusionState, FeedbackPromptField } from '@onelitefeathernet/feedback-fusion-core';
import { computed, inject } from 'vue';

interface FieldProps extends FeedbackPromptField {
  value: any;
  theme?: string;
}

const { config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme])

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
.feedback-fusion__field {
  margin-top: 25px;
  margin-bottom: 15px;

  input {
    outline: none;
    border: 1px solid v-bind("theme.inactive");
    border-radius: 4px;

    padding: 16px;

    color: v-bind("theme.text");
    font-size: 16px;
    line-height: 24px;

    transition: 0.2s ease-out all;

    &:focus {
      border: 1px solid v-bind("theme.primary");
    }
  }

  .feedback-fusion__field-title {
    color: v-bind("theme.inactive");
    font-size: 14px;
    font-weight: bold;
  }

  .feedback-fusion__field-description {
    color: v-bind("theme.subtitle");
    font-size: 11px;
  }

  &:focus-within {
    .feedback-fusion__field-title {
      color: v-bind("theme.primary")
    }
  }

}
</style>
