<template>
  <div @mouseup="onMouseUp" @mouseleave="onMouseUp" @mousemove="onMouseMove" class="feedback-fusion__field__range">
    <div ref="background" class="feedback-fusion__field__range-background" @click="onClick">
      <div :style="{ left: `${left}px`, right: `${background ? background.clientWidth - right : 0}px` }" />
    </div>

    <div @mousedown.prevent="dragLeft = true" class="feedback-fusion__field__range-marker"
      :style="{ left: `calc(${left}px - 10px)` }">
      <div>
        {{ data.start }}
      </div>
    </div>

    <div @mousedown.prevent="dragRight = true" class="feedback-fusion__field__range-marker"
      :style="{ left: `calc(${right}px - 10px)` }">
      <div>
        {{ data.end }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { FeedbackFusionState, RangeOptions } from '@onelitefeathernet/feedback-fusion-core';
import { computed, inject, ref } from 'vue';

interface RangeFieldOptions extends RangeOptions {
  theme?: string;
  value: any;
}

const { config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme])
const props = defineProps<RangeFieldOptions>();
const emit = defineEmits(["update"]);

const dragLeft = ref(false);
const dragRight = ref(false);
const background = ref(null);
const left = computed(() => (data.value.start - props.min) / (props.max - props.min) * background.value?.clientWidth || 0);
const right = computed(() => (data.value.end - props.min) / (props.max - props.min) * background.value?.clientWidth || 0);

const data = computed({
  get() {
    return props.value || { start: props.min, end: props.max };
  },
  set(value: any) {
    emit("update", value);
  }
})

function onClick(event: any) {
  const x = event.layerX;
  const value = Math.round((props.max - props.min) * (x / background.value?.clientWidth || 1) + props.min);

  if (x < left.value) {
    data.value = { start: value, end: data.value?.end };
  }

  if (x > left.value) {
    data.value = { start: data.value?.start, end: value };
  }
}

function onMouseMove(event: any) {
  if (event.target.className !== "feedback-fusion__field__range") return;

  const x = event.layerX;
  const value = Math.round((props.max - props.min) * (x / background.value?.clientWidth || 1) + props.min);

  if (x < right.value && dragLeft.value && value < data.value.end) {
    data.value = { start: value, end: data.value?.end };
  }

  if (x > left.value && dragRight.value && value > data.value.start) {
    data.value = { start: data.value?.start, end: value };
  }
}

function onMouseUp() {
  dragLeft.value = false;
  dragRight.value = false;
}
</script>

<style lang="scss" scoped>
.feedback-fusion__field__range {
  padding: 10px;
  height: 30px;
  width: 100%;
  position: relative;

  .feedback-fusion__field__range-background {
    top: 13.5px;
    left: 0;
    right: 0;
    height: 3px;
    position: absolute;

    background: v-bind("theme.inactive");

    div {
      position: absolute;
      top: 0;
      bottom: 0;
      background: v-bind("theme.primary");
    }
  }

  .feedback-fusion__field__range-marker {
    position: absolute;
    left: 0;
    top: 5px;
    height: 20px;
    width: 20px;
    border-radius: 50%;

    background: v-bind("theme.inactive");

    div {
      position: absolute;
      top: -20px;
      left: 50%;
      transform: translateX(-50%);
      display: none;
    }
  }

  &:hover {
    .feedback-fusion__field__range-marker {
      background: v-bind("theme.primary");

      div {
        display: block;
      }
    }
  }
}
</style>
