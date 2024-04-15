<template>
  <div class="feedback-fusion__field__checkbox">
    <input v-if="props.style === 'checkbox'" v-model="data" type="checkbox">

    <label v-else>
      <input type="checkbox">

      <span />
    </label>
  </div>
</template>

<script setup lang="ts">
import { FeedbackFusionState, CheckboxOptions } from '@onelitefeathernet/feedback-fusion-core';
import { computed, inject, ref } from 'vue';

interface CheckboxFieldOptions extends CheckboxOptions {
  theme?: string;
  value: any;
}

const { config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme])
const props = defineProps<CheckboxFieldOptions>();
const emit = defineEmits(["update"]);

const data = computed({
  get() {
    return props.value || props.defaulState;
  },
  set(value: any) {
    emit("update", value);
  }
})
</script>

<style lang="scss" scoped>
.feedback-fusion__field__checkbox {
  label {
    input {
      height: 0;
      width: 0;
      display: none;
      visibility: hidden;
    }

    span {
      position: relative;
      width: 60px;
      height: 35px;
      display: inline-block;
      border-radius: 16px;
      background: v-bind("theme.inactive");

      &:after {
        position: absolute;
        top: 5px;
        bottom: 5px; 
        left: 5px;
        width: 25px;
        content: "";
        border-radius: 50%;
        background: white;
        transition: 0.15s ease-out;
      }
    }

    input:checked + span {
      background: v-bind("theme.primary");

      &:after {
        left: 30px;
      }
    }
  }
}
</style>
