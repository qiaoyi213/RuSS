<script setup>
import { invoke } from '@tauri-apps/api/core';
import feedCard from './feed.vue';
</script>

<template>
    <h1>RuSS</h1>
    <feedCard />
</template>

<style scoped>
.rss-feed {
  font-family: Arial, sans-serif;
}
.rss-feed ul {
  list-style: none;
  padding: 0;
}
.rss-feed li {
  margin-bottom: 20px;
}
</style>
