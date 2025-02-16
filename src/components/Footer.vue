<script setup lang="ts">
import { useUpgrade } from '~/composables/upgrade'
</script>

<template>
  <n-space h-full items-center px-2>
    <n-button size="tiny" secondary strong @click="useUpgrade()">
      检查更新
    </n-button>
  </n-space>
</template>

<style scoped lang="postcss">

</style>
