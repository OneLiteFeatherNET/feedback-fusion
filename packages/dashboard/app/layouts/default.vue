<template>
  <div>
    <LayoutNavigation />

    <v-container id="content">
      <slot />
    </v-container>
  </div>
</template>

<style lang="scss" scoped>
#content {
  margin-top: 64px;
  min-height: calc(100vh - 64px);
}
</style>
