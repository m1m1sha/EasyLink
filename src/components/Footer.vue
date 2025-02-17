<script setup lang="ts">
</script>

<template>
  <n-space h-full items-center p-1.25 pb-1>
    <n-popover trigger="hover">
      <template #trigger>
        <n-button text>
          <div i-carbon-logo-github h-5 w-5 />
        </n-button>
      </template>
      <span>Github</span>
    </n-popover>
    <n-popover trigger="hover">
      <template #trigger>
        <n-button text>
          <div i-carbon-update-now h-5 w-5 />
        </n-button>
      </template>
      <span>检查更新</span>
    </n-popover>
  </n-space>
</template>

<style scoped lang="postcss">

</style>
