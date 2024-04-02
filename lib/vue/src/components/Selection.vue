<template>
  <div class="feedback-fusion__field__selection">
    <input @click="expanded = !expanded" type="text" readonly />

    <div :style="style" class="feedback-fusion__field__selection-list">
      <div @click="onClick(value)" v-for="(value, i) in props.values" :key="i" :style="itemStyle(value)">
        {{ value }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FeedbackFusionState, SelectionOptions } from '@onelitefeathernet/feedback-fusion-core';
import { computed, inject, ref } from 'vue';

interface SelectionFieldOptions extends SelectionOptions {
  theme?: string;
  value: any;
}

const { config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme])
const props = defineProps<SelectionFieldOptions>();
const emit = defineEmits(["update"]);

const expanded = ref(false);

const style = computed(() => {
  if (!expanded.value) {
    return {
      padding: "0",
      height: "0",
      border: "none"
    }
  }
})

const itemStyle = computed(() => (value: string) => {
  if (data.value.includes(value)) {
    return {
      background: `${theme.value.primary}19`,
    }
  }
})

const focus = computed(() => `${theme.value.subtitle}19`)

const data = computed({
  get() {
    return props.value || [];
  },
  set(value: any) {
    emit("update", value);
  }
})

function onClick(value: string) {
  if (data.value.includes(value)) {
    data.value = data.value.filter((item: string) => item !== value);
  } else {
    data.value = data.value.concat([value])
  }
}
</script>

<style lang="scss" scoped>
.feedback-fusion__field__selection {
  .feedback-fusion__field__selection-list {
    width: 100%;
    max-height: 300px;
    overflow: scroll;

    padding: 10px;
    border: 1px solid v-bind("theme.subtitle");
    border-top: none;
    border-radius: 0 0 10px 10px;

    transition: 0.2s ease-out;

    >div {
      padding: 10px;
      border-bottom: 1px solid v-bind("theme.subtitle");

      &:hover {
        cursor: pointer;
        background: v-bind("focus")
      }
    }
  }
}
</style>
