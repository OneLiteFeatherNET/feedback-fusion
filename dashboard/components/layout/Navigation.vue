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

        <LayoutAuthentication v-if="loggedIn" />
      </v-row>
    </v-container>
  </v-app-bar>
</template>

<script setup lang="ts">
import { useOidcAuth } from "#imports";

const { loggedIn } = useOidcAuth();
</script>
