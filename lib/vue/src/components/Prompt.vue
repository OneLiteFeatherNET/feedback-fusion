<template>
  <slot v-bind="childProps">
    {{ prompt?.title }} 
  </slot>
</template>

<script setup lang="ts">
import { FeedbackPromptField, FeedbackFusionState, FeedbackPrompt } from "@onelitefeathernet/feedback-fusion-core"
import { inject, onMounted, ref } from "vue";
import i18next from "i18next"

interface PromptProps {
  prompt: string;
  locale?: string;
}

const props = defineProps<PromptProps>();
const { client } = inject<FeedbackFusionState>('feedbackFusionState')!;

const fieldPage = ref(0);
const fieldPages = ref(1);
const prompt = ref(undefined as FeedbackPrompt | undefined)
const fields = ref([] as FeedbackPromptField[]);

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
})
</script>
