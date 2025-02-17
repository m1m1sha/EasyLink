<script setup lang="ts">

</script>

<template>
  <n-layout h-full position="absolute">
    <n-layout-content position="absolute" content-class="p-6" :native-scrollbar="false">
      <RouterView />
    </n-layout-content>
    <n-layout-footer position="absolute" bordered h-8>
      <Footer />
    </n-layout-footer>
  </n-layout>
</template>

<style scoped lang="postcss">
.n-layout {
  & :deep(.n-layout-scroll-container) {
    @apply h-full;
  }
}
</style>
