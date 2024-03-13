<template>
  <slot v-bind="childProps">
    <div v-if="prompt?.active" class="feedback-fusion__prompt">
      <div class="feedback-fusion__prompt-container">
        <div class="feedback-fusion__prompt-header">
          <div class="feedback-fusion__prompt-header-title">
            <slot name="title">
              {{ prompt?.title || i18next.t("") }}
            </slot>
          </div>

          <div class="feedback-fusion__prompt-header-subtitle">
            <slot name="subtitle">
              {{ i18next.t("page", { current: fieldPage, total: fieldPages }) }}
            </slot>
          </div>
        </div>

        <div class="feedback-fusion__prompt-fields">
          <slot name="field" v-for="field in fields" :key="field.id">
            <Field v-bind="{ ...field, value: data[field.id], theme: props.theme }"
              @update="event => data[field.id] = event" />
          </slot>
        </div>

        <div class="feedback-fusion__prompt-actions">
          <button @click="submitResponse" class="feedback-fusion__prompt-actions-submit">
            {{ i18next.t("submit") }}
          </button>
        </div>
      </div>
    </div>
  </slot>
</template>

<script setup lang="ts">
import Field from "./Field.vue";
import { FeedbackPromptField, FeedbackFusionState, FeedbackPrompt, SubmitFeedbackPromptResponseRequest } from "@onelitefeathernet/feedback-fusion-core";
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

const data = ref({} as { [key: string]: any });
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
      // TODO: do we actually already return the pages?
      fieldPages.value = Math.ceil(data.total / 10);
      fields.value = data.records;
    })
});

const submitResponse = async () => {
  // TODO: validate data types
  // transform data
  const body = {} as SubmitFeedbackPromptResponseRequest;
  // @ts-ignore
  Object.keys(data).forEach((key: string) => body[key] = { data: data[key] });

  // TODO: error handling 
  // TODO: show submit
  await client.submitResponse(props.prompt, body)
    .then(() => data.value = {});
};
</script>

<style lang="scss" scoped>
.feedback-fusion__prompt {
  color: v-bind("theme.text");
  width: 100%;

  .feedback-fusion__prompt-container {
    margin: auto;
    background-color: v-bind("theme.sheet");
    padding: 16px;
    overflow: hidden;

    .feedback-fusion__prompt-header {
      .feedback-fusion__prompt-header-title {
        font-weight: bold;
        font-size: 20px;
      }

      .feedback-fusion__prompt-header-subtitle {
        color: v-bind("theme.subtitle");
        font-size: 14px;
      }
    }

    .feedback-fusion__prompt-fields {
      padding: 10px 0;
    }

    .feedback-fusion__prompt-actions {
      button {
        text-transform: uppercase;
        font-weight: bold;
        letter-spacing: 2px;
        font-size: 13px;
        color: v-bind("theme.primary");

        position: relative;
        padding: 10px 15px;

        &:after {
          content: "";
          position: absolute;
          left: 0;
          right: 0;
          top: 0;
          bottom: 0;
          background: v-bind("theme.primary");
          opacity: 0;
          transition: 0.1s ease-out all;
        }

        &:hover {
          &:after {
            opacity: 0.1;
          }
        }
      }

      .feedback-fusion__prompt-actions-submit {
        float: right;
      }
    }
  }
}
</style>
