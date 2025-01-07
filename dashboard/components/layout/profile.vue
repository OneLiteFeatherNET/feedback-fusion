<template>
  <v-menu rounded>
    <template #activator="{ props }">
      <v-btn icon v-bind="props">
        <v-avatar color="secondary">
          {{ user.userName[0] }}
        </v-avatar>
      </v-btn>
    </template>

    <v-card>
      <v-card-text>
        <div class="text-center">
          <v-avatar color="secondary">
            {{ user.userName[0] }}
          </v-avatar>

          <h3>
            {{ user.userName }}
          </h3>
        </div>

        <v-divider class="mt-2 mb-2" />

        <v-btn @click="logout()" variant="text" rounded>
          {{ $t("navigation.logout") }}
        </v-btn>
      </v-card-text>
    </v-card>
  </v-menu>
</template>

<script setup lang="ts">
import { useOidcAuth } from "#imports";

const { user, logout } = useOidcAuth();
</script>
