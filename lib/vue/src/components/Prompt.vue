<template>
  <slot v-bind="childProps">
    <VContainer>
      <VCard :loading="!!prompt">
        <VCardTitle>
          {{ prompt?.title || "Loading..." }}
        </VCardTitle>

        <VCardSubtitle>

        </VCardSubtitle>
      </VCard>
    </VContainer>
  </slot>
</template>

<script setup lang="ts">
import { FeedbackPromptField, FeedbackFusionState, FeedbackPrompt } from "@onelitefeathernet/feedback-fusion-core"
import { VContainer } from "vuetify/components/VGrid";
import { VCard, VCardText, VCardTitle, VCardActions, VCardSubtitle } from "vuetify/components/VCard";
import { inject, onMounted, ref } from "vue";

interface PromptProps {
  prompt: string;
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
