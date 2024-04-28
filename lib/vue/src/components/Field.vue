<template>
  <div class="feedback-fusion__field">
    <div class="feedback-fusion__field-title">
      {{ props.title }}
    </div>

    <template v-if="props.fieldType === FieldType.TEXT">
      <input v-if="props.options.text.lines === 1" v-model="data" type="text" :placeholder="props.options.text.placeholder">
      <textarea v-else :rows="props.options.lines" v-model="data" :placeholder="props.options.text.placeholder" />
    </template>

    <div v-else-if="props.fieldType === FieldType.RATING" class="feedback-fusion__field__rating">
      <div class="feedback-fusion__field__rating-point" v-for="i in props.options.rating.max">
        <input v-model="data" type="radio" :name="props.id" :id="props.id + i" :value="i">
        <label :for="props.id + i">
          <svg xmlns="http://www.w3.org/2000/svg" :fill="starColor(i)" viewBox="0 0 24 24">
            <title>star-outline</title>
            <path
              d="M12,15.39L8.24,17.66L9.23,13.38L5.91,10.5L10.29,10.13L12,6.09L13.71,10.13L18.09,10.5L14.77,13.38L15.76,17.66M22,9.24L14.81,8.63L12,2L9.19,8.63L2,9.24L7.45,13.97L5.82,21L12,17.27L18.18,21L16.54,13.97L22,9.24Z" />
          </svg>
        </label>
      </div>
    </div>

    <Checkbox v-else-if="props.fieldType === FieldType.CHECKBOX" v-bind="props.options.checkbox" :value="data"
      :theme="props.theme" @update="event => data = event" />

    <Range v-else-if="props.fieldType === FieldType.RANGE" v-bind="props.options.range" :theme="props.theme"
      :value="data" @update="event => data = event" />

    <Selection v-else-if="props.fieldType === FieldType.SELECTION" v-bind="props.options.selection"
      :theme="props.theme" :value="data" @update="event => data = event" />

    <input v-else-if="props.fieldType === FieldType.NUMBER" type="number" v-bind="props.options.number">

    <div class="feedback-fusion__field-description">
      {{ props.description }}
    </div>

    <div v-if="props.type === 'number'" class="feedback-fusion__field-error">
      {{ i18next.t("field.number.error", { min: props.options.min, max: props.options.max }) }}
    </div>
  </div>
</template>

<script setup lang="ts">
import Checkbox from "./Checkbox.vue";
import Range from "./Range.vue";
import Selection from "./Selection.vue";
import { FeedbackFusionState, Field, FieldType } from '@onelitefeathernet/feedback-fusion-core';
import { computed, inject } from 'vue';
import i18next from "i18next";

interface FieldProps extends Field {
  value: any;
  theme?: string;
}

const { config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme])
const starColor = computed(() => (i: number) => data.value >= i ? theme.value.primary : theme.value.inactive)

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

<style lang="scss">
.feedback-fusion__field {
  margin-top: 25px;
  margin-bottom: 15px;

  >*>.feedback-fusion__field-input-container,
  >input,
  >textarea {
    outline: none;
    border: 1px solid v-bind("theme.inactive");
    border-radius: 4px;

    width: 100%;
    padding: 16px;

    color: v-bind("theme.text");
    font-size: 16px;
    line-height: 24px;

    transition: 0.2s ease-out all;

    &:focus {
      border: 1px solid v-bind("theme.primary");
    }

    &:invalid {
      border-color: v-bind("theme.error")
    }

    &:invalid~.feedback-fusion__field-error {
      display: block !important;
    }
  }

  .feedback-fusion__field__rating {
    .feedback-fusion__field__rating-point {
      display: inline-block;

      input {
        height: 0;
        width: 0;
        visibility: hidden;
        display: none;
      }

      svg {
        cursor: pointer;
        height: 35px;
      }
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

  .feedback-fusion__field-error {
    color: v-bind("theme.error");
    font-size: 11px;
    display: none;
  }

  &:focus-within {
    .feedback-fusion__field-title {
      color: v-bind("theme.primary")
    }
  }

}
</style>
