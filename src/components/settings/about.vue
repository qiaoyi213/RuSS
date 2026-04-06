<script setup lang="ts">
import { NFlex, NTag, NText } from 'naive-ui';
</script>

<template>
  <n-flex vertical size="small">
    <h3>RuSS</h3>
    <n-text depth="3">RuSS 是以 Tauri + Vue 打造的桌面 RSS 閱讀器，專注於快速追蹤與舒適閱讀。</n-text>
    <n-flex>
      <n-tag type="info" round>Tauri</n-tag>
      <n-tag type="success" round>Vue 3</n-tag>
      <n-tag type="warning" round>Naive UI</n-tag>
    </n-flex>
  </n-flex>
</template>

<style scoped>
h3 {
  margin: 0;
}
</style>
