<template>
  <v-menu rounded>
    <template #activator="{ props }">
      <v-btn icon v-bind="props">
        <UserAvatar :username="session.user.name" />
      </v-btn>
    </template>

    <v-card>
      <v-card-text>
        <div class="text-center">
          <UserAvatar :username="session.user.name" />

          <h3>
            {{ session.user.name }}
          </h3>
        </div>

        <v-divider class="mt-2 mb-2" />

        <v-btn @click="logout" variant="text" rounded>
          {{ $t("navigation.logout") }}
        </v-btn>
      </v-card-text>
    </v-card>
  </v-menu>
</template>

<script setup lang="ts">
import { navigateTo } from "#imports";
import {
  useOIDCClient,
  useAuthorizationStore,
  useAuthSession,
} from "~/composables/authorization";

const { session } = await useAuthSession();
const store = useAuthorizationStore();
const oidcClient = useOIDCClient();

async function logout() {
  store.logout();

  await oidcClient.signOut({
    fetchOptions: {
      onSuccess: () => {
        navigateTo("/auth/login");
      },
    },
  });
}
</script>
