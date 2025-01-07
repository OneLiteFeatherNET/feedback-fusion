<template>
  <v-expansion-panels color="secondary">
    <v-expansion-panel :title="$t('prompt.preview')">
      <v-expansion-panel-text>
        <v-tabs bg-color="secondary" v-model="tab">
          <v-tab value="preview">
            {{ $t("prompt.preview") }}
          </v-tab>

          <v-tab value="code">
            {{ $t("prompt.code") }}
          </v-tab>
        </v-tabs>

        <v-tabs-window v-model="tab">
          <v-tabs-window-item value="preview">
            <div class="d-flex justify-center align-center">
              <feedback-fusion-prompt
                :baseUrl="config.public.feedbackFusionEndpoint"
                :promptId="prompt"
                class="mt-4 mb-4"
                style="width: 500px; max-width: 95vw"
              />
            </div>
          </v-tabs-window-item>

          <v-tabs-window-item value="code">
            <pre>
              <code>
              &lt;feedback-fusion-prompt
                baseUrl="{{ config.public.feedbackFusionEndpoint }}"
                promptId="{{ prompt }}"
              /&gt;
              </code>
            </pre>
          </v-tabs-window-item>
        </v-tabs-window>
      </v-expansion-panel-text>
    </v-expansion-panel>
  </v-expansion-panels>
</template>

<script setup lang="ts">
import { defineProps, useRuntimeConfig, ref } from "#imports";
import "@onelitefeathernet/feedback-fusion";

const config = useRuntimeConfig();
const tab = ref("preview");

const props = defineProps({
  prompt: String,
});
</script>
