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
import { useRouter } from "#imports";
import { oidcClient, useAuthorizationStore } from "~/composables/authorization";

const { data: session } = await oidcClient.useSession(useFetch);
const store = useAuthorizationStore();
const router = useRouter();

async function logout() {
  await oidcClient.signOut();

  store.logout();
  router.push("/auth/login");
}
</script>
