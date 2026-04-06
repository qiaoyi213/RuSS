<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-shell';
import { Readability } from '@mozilla/readability';
import {
  NButton,
  NCard,
  NEmpty,
  NFlex,
  NGrid,
  NGridItem,
  NModal,
  NSpin,
  NTag,
  NText,
} from 'naive-ui';

interface FeedItem {
  title: string;
  link: string;
  pubDate?: string;
  description?: string;
}

defineProps<{
  feedsList: FeedItem[];
  loading: boolean;
  activeSourceTitle: string;
}>();

const viewerOpen = ref(false);
const readerLoading = ref(false);
const readHtml = ref('');
const readingFeed = ref<FeedItem | null>(null);
const readingFontSize = ref(18);

async function readFeed(feed: FeedItem): Promise<void> {
  viewerOpen.value = true;
  readerLoading.value = true;
  readingFeed.value = feed;
  readHtml.value = '';

  try {
    const htmlText = await invoke<string>('getFeed', { url: feed.link });
    const parser = new DOMParser();
    const doc = parser.parseFromString(htmlText, 'text/html');
    const article = new Readability(doc).parse();

    if (article?.content) {
      readHtml.value = article.content;
    } else {
      readHtml.value = '<p>無法擷取可閱讀內容，請改用原文模式。</p>';
    }
  } catch (error) {
    console.error(error);
    readHtml.value = '<p>文章載入失敗，請稍後再試。</p>';
  } finally {
    readerLoading.value = false;
  }
}

async function openOriginal(): Promise<void> {
  if (!readingFeed.value?.link) {
    return;
  }

  try {
    await open(readingFeed.value.link);
  } catch (error) {
    console.error(error);
  }
}

function closeReader(): void {
  viewerOpen.value = false;
}

function increaseFontSize(): void {
  readingFontSize.value = Math.min(26, readingFontSize.value + 1);
}

function decreaseFontSize(): void {
  readingFontSize.value = Math.max(14, readingFontSize.value - 1);
}

function resetFontSize(): void {
  readingFontSize.value = 18;
}
</script>

<template>
  <n-spin :show="loading">
    <n-empty v-if="!loading && feedsList.length === 0" description="目前沒有文章" size="large" />

    <n-grid v-else :x-gap="16" :y-gap="16" cols="1 s:1 m:2 l:2 xl:3" responsive="screen">
      <n-grid-item v-for="feed in feedsList" :key="feed.link + feed.title">
        <n-card class="feed-card" hoverable @click="readFeed(feed)">
          <n-flex vertical size="small">
            <n-tag v-if="activeSourceTitle" round type="info">{{ activeSourceTitle }}</n-tag>
            <h3 class="feed-title">{{ feed.title }}</h3>
            <n-text depth="3" class="feed-description">
              {{ feed.description || '無摘要資訊' }}
            </n-text>
            <n-text depth="3" class="feed-date">{{ feed.pubDate || '' }}</n-text>
          </n-flex>
        </n-card>
      </n-grid-item>
    </n-grid>
  </n-spin>

  <n-modal
    v-model:show="viewerOpen"
    preset="card"
    class="reader-modal"
    :bordered="false"
    closable
    @mask-click="closeReader"
  >
    <h1 class="reader-title" @click="openOriginal">
      {{ readingFeed?.title || '文章閱讀' }}
    </h1>

    <n-flex class="font-controls" align="center" justify="end" size="small">
      <n-button size="small" @click="decreaseFontSize">A-</n-button>
      <n-button size="small" @click="resetFontSize">{{ readingFontSize }}px</n-button>
      <n-button size="small" @click="increaseFontSize">A+</n-button>
    </n-flex>

    <n-spin :show="readerLoading">
      <article class="reading-content" :style="{ fontSize: `${readingFontSize}px` }" v-html="readHtml"></article>
    </n-spin>
  </n-modal>
</template>

<style scoped>
.feed-card {
  height: 100%;
  border-radius: 14px;
  border: 1px solid var(--feed-card-border);
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  cursor: pointer;
}

.feed-card:hover {
  transform: translateY(-3px);
  box-shadow: var(--feed-card-shadow-hover);
}

.feed-title {
  margin: 0;
  color: var(--feed-title-color);
  line-height: 1.35;
  font-size: 1rem;
}

.feed-description {
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
  min-height: 3.9em;
}

.feed-date {
  font-size: 0.78rem;
}

.reading-content {
  max-width: 72ch;
  margin: 0 auto;
  line-height: 1.82;
  color: var(--reader-content-color);
}

.reading-content :deep(img) {
  max-width: 100%;
  border-radius: 10px;
}

.reading-content :deep(pre) {
  white-space: pre-wrap;
}

.reader-modal {
  width: clamp(640px, 68vw, 860px);
  max-width: 92vw;
  max-height: 88vh;
  border-radius: 18px;
  overflow: auto;
  background: var(--reader-modal-bg);
}

.reader-title {
  margin: 0 0 16px;
  color: var(--reader-title-color);
  font-size: 1.9rem;
  line-height: 1.3;
  cursor: pointer;
  text-decoration: underline;
  text-decoration-color: var(--reader-title-underline);
  text-underline-offset: 5px;
}

.reader-title:hover {
  color: var(--reader-title-hover-color);
  text-decoration-color: var(--reader-title-hover-underline);
}

.font-controls {
  margin-bottom: 12px;
}

@media (max-width: 980px) {
  .reader-modal {
    width: 92vw;
  }
}
</style>
