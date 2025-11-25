<template>
  <v-menu rounded>
    <template #activator="{ props }">
      <v-btn icon v-bind="props">
        <UserAvatar :username="user.userName" />
      </v-btn>
    </template>

    <v-card>
      <v-card-text>
        <div class="text-center">
          <UserAvatar :username="user.userName" />

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
