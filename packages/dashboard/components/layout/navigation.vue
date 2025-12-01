<template>
  <v-app-bar color="primary">
    <v-container>
      <v-row class="align-center">
        <v-app-bar-title class="d-flex align-center justify-start">
          <NuxtLink class="text-decoration-none bg-primary" to="/">
            Feedback-Fusion
          </NuxtLink>
        </v-app-bar-title>

        <v-spacer />

        <LayoutTheme class="mr-4" />
        <LayoutProfile v-if="session" />
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<script setup lang="ts">
import { oidcClient } from "~/composables/authorization";

const { data: session } = await oidcClient.useSession(useFetch);
</script>
