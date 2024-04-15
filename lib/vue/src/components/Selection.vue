<template>
  <div class="feedback-fusion__field__selection">
    <div class="feedback-fusion__field-input-container" @click="expanded = !expanded">
      <div class="feedback-fusion__field__selection-chips">
        <div v-for="(value, i) in data" :key="i">
          {{ value }}
        </div>
      </div>

      <input @keyup.enter="onEnter" type="text" :readonly="!props.combobox" v-model="search" />
    </div>

    <div :style="style" class="feedback-fusion__field__selection-list">
      <div @click="onClick(value)" v-for="(value, i) in listValues" :key="i" :style="itemStyle(value)">
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
const search = ref("")

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

const listValues = computed(() => {
  if (props.combobox) {
    return props.values.filter((value: string) => value.startsWith(search.value));
  } else {
    return props.values;
  }
})

function onClick(value: string) {
  if (data.value.includes(value)) {
    data.value = data.value.filter((item: string) => item !== value);
  } else {
    data.value = data.value.concat([value])
  }
}

function onEnter() {
  if (props.combobox && !!search.value) {
    data.value = data.value.concat([search.value]);
    search.value = "";
  }
}
</script>

<style lang="scss" scoped>
.feedback-fusion__field__selection {
  .feedback-fusion__field-input-container {
    display: flex;
    flex-direction: row;

    .feedback-fusion__field__selection-chips {
      padding: 0 10px;

      div {
        background: v-bind("theme.primary");
        padding: 0 12px;
        border-radius: 16px;
        display: inline-block;
        margin-left: 5px;
      }
    }

    input {
      flex-grow: 1;
    }
  }

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
