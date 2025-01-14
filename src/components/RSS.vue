<script setup>
import tauriapi from '@tauri-apps/api';
const { invoke } = tauriapi.tauri;
let items = invoke('update_channels');
</script>

<template>
  <div class="rss-feed">
    <h2>RSS Feed</h2>
    <ul v-if="rssItems.length">
      <li v-for="(item, index) in rssItems" :key="index">
        <a :href="item.link" target="_blank">{{ item.title }}</a>
        <p>{{ item.description }}</p>
      </li>
    </ul>
    <div v-else>
      Loading RSS feeds...
    </div>
  </div>
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
