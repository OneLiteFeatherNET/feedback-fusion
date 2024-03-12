<template>
  <slot v-bind="childProps">
    <div class="feedback-fusion__prompt">
      <div class="feedback-fusion__prompt-container">
        <div class="feedback-fusion__prompt-header">
          <div class="feedback-fusion__prompt-header-title">
            <slot name="title">
              {{ prompt?.title }}
            </slot>
          </div>

          <div class="feedback-fusion__prompt-header-subtitle">
            <slot name="subtitle">
              {{ i18next.t("page", { current: fieldPage, total: fieldPages }) }}
            </slot>
          </div>
        </div>

        <slot name="fields">
          <div class="feedback-fusion__prompt-fields">
            <Field v-for="field in fields" :key="field.id" v-bind="{ ...field, value: data[field.id]   }" />
          </div>
        </slot>
      </div>
    </div>
  </slot>
</template>

<script setup lang="ts">
import Field from "./Field.vue";
import { FeedbackPromptField, FeedbackFusionState, FeedbackPrompt } from "@onelitefeathernet/feedback-fusion-core";
import { computed, inject, onMounted, ref } from "vue";
import i18next from "i18next";

interface PromptProps {
  prompt: string;
  locale?: string;
  theme?: string;
}

const props = defineProps<PromptProps>();
const { client, config } = inject<FeedbackFusionState>('feedbackFusionState')!;
const theme = computed(() => config.themes[props.theme || config.defaultTheme]);

const fieldPage = ref(0);
const fieldPages = ref(1);
const prompt = ref(undefined as FeedbackPrompt | undefined)
const fields = ref([] as FeedbackPromptField[]);

const data = ref({} as { [key:string]: any });
const childProps = ref({
  prompt,
  fields,
  fieldPage,
  fieldPages
});

onMounted(async () => {
  // fetch the prompt information
  await client.getPrompt(props.prompt)
    .then((data) => prompt.value = data);

  // fetch the first field page 
  await client.getFields(props.prompt, 1, 10)
    .then((data) => {
      fieldPages.value = Math.ceil(data.total / 10);
      fields.value = data.records;
    })
});
</script>

<style lang="scss" scoped>
.feedback-fusion__prompt {
  color: v-bind("theme.text");
  width: 100%;

  .feedback-fusion__prompt-container {
    margin: auto;
    background-color: v-bind("theme.sheet");
    padding: 16px;

    .feedback-fusion__prompt-header {
      .feedback-fusion__prompt-header-title {
        font-weight: bold;
        font-size: 20px;
      }

      .feedback-fusion__prompt-header-subtitle {
        color: v-bind("theme.subtitle");
      }
    }

    .feedback-fusion__prompt-fields {
      padding: 20px 0;
    }
  }
}
</style>
