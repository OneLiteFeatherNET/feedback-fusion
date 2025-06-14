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

        <v-btn :to="localePath('/iam')" class="mr-4" v-if="authorization.hasPermission('Authorize', 'Read')">
          {{ $t("navigation.iam") }}
        </v-btn>
        <LayoutTheme class="mr-4" />
        <LayoutProfile v-if="loggedIn" />
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<script setup lang="ts">
import { useOidcAuth, useLocalePath } from "#imports";
import { useAuthorizationStore } from "~/composables/authorization";

const localePath = useLocalePath();
const { loggedIn } = useOidcAuth();
const authorization = useAuthorizationStore();
</script>
