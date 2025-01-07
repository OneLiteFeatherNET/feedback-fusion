<template>
  <div id="login" class="justify-center align-center d-flex">
    <v-card style="width: 300px">
      <v-card-title>
        {{ $t("login.title") }}
      </v-card-title>

      <v-card-text class="mt-4 mb-4">
        <v-btn @click="login('oidc')" block color="primary">
          {{ $t("login.oidc") }}
        </v-btn>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { useOidcAuth } from "#imports";

const { login } = useOidcAuth();
</script>

<style lang="scss" scoped>
#login {
  height: 100vh;
  position: absolute;
  left: 0;
  top: 0;
  width: 100vw;
}
</style>
