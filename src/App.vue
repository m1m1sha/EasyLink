<script setup lang="ts">
import { darkTheme, dateZhCN, zhCN } from 'naive-ui'
</script>

<template>
  <n-config-provider h-full :theme="isDark ? darkTheme : null" :locale="zhCN" :date-locale="dateZhCN">
    <RouterView />
  </n-config-provider>
</template>
