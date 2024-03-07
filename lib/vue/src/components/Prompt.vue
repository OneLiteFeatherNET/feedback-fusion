<template></template>

<script setup lang="ts">
import { FeedbackPromptField } from "@onelitefeathernet/feedback-fusion-core"
import { inject } from "vue";
import { FeedbackFusionState } from "../config";

interface PromptProps {
  baseURL: string;
  target: string;
  prompt: string;
}

const props = defineProps<PromptProps>();
const { client } = inject<FeedbackFusionState>('feedbackFusionState')!;

const page = ref(0);
const pages = ref(1);
const fields = ref([] as FeedbackPromptField[]);

onMounted(async () => {
})
</script>
