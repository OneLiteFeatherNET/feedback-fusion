<template>
  <v-btn variant="text" v-bind="$attrs" :icon="icon" @click="changeTheme" />
</template>

<script setup lang="ts">
import { computed, useColorMode } from "#imports";
import { useTheme } from "vuetify";

const theme = useTheme();
const colorMode = useColorMode();
const icon = computed(() =>
  colorMode.value === "light" ? "mdi-weather-night" : "mdi-weather-sunny",
);
const changeTheme = () => {
  colorMode.preference = colorMode.value === "light" ? "dark" : "light";
  colorMode.value = colorMode.preference;
  syncTheme();
};

const syncTheme = () => {
  theme.global.name.value = colorMode.value;
};

if (process.client) {
  syncTheme();
}
</script>
