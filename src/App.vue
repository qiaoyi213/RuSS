<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import {
  NAlert,
  NButton,
  NCard,
  NFlex,
  NInput,
  NLayout,
  NLayoutContent,
  NLayoutSider,
  NModal,
  NTag,
  NText,
  createDiscreteApi,
} from 'naive-ui';
import Sidebar from './components/sidebar.vue';
import Feeds from './components/feeds.vue';
import Settings from './components/settings.vue';

interface Source {
  title: string;
  link: string;
  description: string;
}

interface FeedItem {
  title: string;
  link: string;
  pubDate?: string;
  description?: string;
}

const { message } = createDiscreteApi(['message']);

const sources = ref<Source[]>([]);
const feedsList = ref<FeedItem[]>([]);
const selectedSource = ref<Source | null>(null);
const loadingFeeds = ref(false);

const showAddSourceModal = ref(false);
const sourceInput = ref('');
const sourcePreview = ref<Source | null>(null);
const previewLoading = ref(false);

const showSettings = ref(false);
let unlistenSettings: null | (() => void) = null;

const sourceCountLabel = computed(() => `${sources.value.length} sources`);
const feedCountLabel = computed(() => `${feedsList.value.length} articles`);

async function loadSources(): Promise<void> {
  try {
    const response = await invoke<string>('getSources');
    const parsed = JSON.parse(response) as { sources?: Source[] };
    sources.value = parsed.sources ?? [];

    if (selectedSource.value) {
      const stillExists = sources.value.find((item) => item.title === selectedSource.value?.title);
      if (!stillExists) {
        selectedSource.value = null;
      }
    }

    await refreshFeeds();
  } catch (error) {
    console.error(error);
    message.error('無法讀取來源清單');
  }
}

async function refreshFeeds(): Promise<void> {
  loadingFeeds.value = true;

  try {
    const targetSources = selectedSource.value ? [selectedSource.value] : sources.value;
    const feedChunks = await Promise.all(
      targetSources.map(async (source) => {
        const response = await invoke<string[]>('example_feed', { url: source.link });
        return response
          .map((item) => {
            try {
              return JSON.parse(item) as FeedItem;
            } catch {
              return null;
            }
          })
          .filter((item): item is FeedItem => item !== null);
      }),
    );

    feedsList.value = feedChunks.flat();
  } catch (error) {
    console.error(error);
    message.error('更新文章失敗，請稍後再試');
    feedsList.value = [];
  } finally {
    loadingFeeds.value = false;
  }
}

function handleSelectSource(source: Source): void {
  selectedSource.value = source;
  void refreshFeeds();
}

function handleClearSource(): void {
  selectedSource.value = null;
  void refreshFeeds();
}

async function handleDeleteSource(source: Source): Promise<void> {
  try {
    await invoke('deleteSource', { title: source.title });
    message.success(`已刪除來源：${source.title}`);
    await loadSources();
  } catch (error) {
    console.error(error);
    message.error('刪除來源失敗');
  }
}

async function previewSource(): Promise<void> {
  if (!sourceInput.value.trim()) {
    sourcePreview.value = null;
    return;
  }

  previewLoading.value = true;
  sourcePreview.value = null;

  try {
    const response = await invoke<string>('getSourceInfo', { url: sourceInput.value.trim() });
    sourcePreview.value = JSON.parse(response) as Source;
  } catch (error) {
    console.error(error);
    sourcePreview.value = null;
    message.warning('無法解析此 RSS，請確認網址');
  } finally {
    previewLoading.value = false;
  }
}

async function addSource(): Promise<void> {
  if (!sourcePreview.value) {
    message.warning('請先輸入有效 RSS 並完成預覽');
    return;
  }

  try {
    await invoke('addSource', {
      title: sourcePreview.value.title,
      description: sourcePreview.value.description,
      link: sourcePreview.value.link,
    });

    message.success(`已新增來源：${sourcePreview.value.title}`);
    showAddSourceModal.value = false;
    sourceInput.value = '';
    sourcePreview.value = null;
    await loadSources();
  } catch (error) {
    console.error(error);
    message.error('新增來源失敗');
  }
}

function openAddModal(): void {
  sourceInput.value = '';
  sourcePreview.value = null;
  showAddSourceModal.value = true;
}

onMounted(async () => {
  await loadSources();

  unlistenSettings = await listen('settings', () => {
    showSettings.value = true;
  });
});

onBeforeUnmount(() => {
  unlistenSettings?.();
});
</script>

<template>
  <n-layout class="app-shell" has-sider>
    <n-layout-sider class="left-panel" :width="300" collapse-mode="width" :native-scrollbar="false">
      <div class="brand-block">
        <h1>RuSS</h1>
        <n-text depth="3">Read less noise, keep signal.</n-text>
      </div>

      <n-flex vertical size="small" class="action-row">
        <n-button type="primary" size="large" @click="refreshFeeds">更新文章</n-button>
        <n-button class="add-feed-btn" size="large" @click="openAddModal">新增 Feed 源</n-button>
        <n-button tertiary class="settings-btn" size="large" @click="showSettings = true">設定</n-button>
      </n-flex>

      <n-flex class="meta-row" size="small">
        <n-tag round>{{ sourceCountLabel }}</n-tag>
        <n-tag round type="success">{{ feedCountLabel }}</n-tag>
      </n-flex>

      <sidebar
        :sources="sources"
        :selected-source="selectedSource"
        @select-source="handleSelectSource"
        @clear-source="handleClearSource"
        @delete-source="handleDeleteSource"
      />
    </n-layout-sider>

    <n-layout-content class="main-panel" :native-scrollbar="false">
      <n-card class="feed-panel" :bordered="false">
        <template #header>
          <n-flex justify="space-between" align="center">
            <div>
              <h2 class="panel-title">文章列表</h2>
              <n-text depth="3">
                {{ selectedSource ? `目前來源：${selectedSource.title}` : '目前來源：全部' }}
              </n-text>
            </div>
          </n-flex>
        </template>

        <feeds :feeds-list="feedsList" :loading="loadingFeeds" :active-source-title="selectedSource?.title ?? ''" />
      </n-card>
    </n-layout-content>
  </n-layout>

  <n-modal v-model:show="showAddSourceModal" preset="card" class="add-source-modal" title="新增 RSS 來源">
    <n-flex vertical size="large">
      <n-input
        v-model:value="sourceInput"
        clearable
        placeholder="貼上 RSS URL，例如：https://example.com/feed.xml"
        @blur="previewSource"
      />

      <n-button :loading="previewLoading" secondary @click="previewSource">預覽來源資訊</n-button>

      <n-alert v-if="!sourcePreview && sourceInput" type="warning" :show-icon="false">
        尚未取得來源資訊，請先預覽後再新增。
      </n-alert>

      <n-card v-if="sourcePreview" size="small" :bordered="false" class="preview-card">
        <n-flex vertical size="small">
          <strong>{{ sourcePreview.title }}</strong>
          <n-text depth="3">{{ sourcePreview.description }}</n-text>
          <n-text depth="3">{{ sourcePreview.link }}</n-text>
        </n-flex>
      </n-card>

      <n-flex justify="end" size="small">
        <n-button @click="showAddSourceModal = false">取消</n-button>
        <n-button type="primary" :disabled="!sourcePreview" @click="addSource">新增</n-button>
      </n-flex>
    </n-flex>
  </n-modal>

  <n-modal v-model:show="showSettings" preset="card" class="settings-modal" title="設定">
    <settings @close="showSettings = false" />
  </n-modal>
</template>

<style scoped>
.app-shell {
  height: 100vh;
  overflow: hidden;
  background: radial-gradient(circle at 20% 10%, #dff4f2 0%, #f3f8fd 40%, #f8f5f0 100%);
}

.left-panel {
  height: 100vh;
  padding: 24px 18px;
  overflow: hidden;
  background: linear-gradient(160deg, #0f1720 0%, #142434 45%, #1b3349 100%);
  border-right: 1px solid rgba(255, 255, 255, 0.08);
}

.brand-block h1 {
  margin: 0;
  color: #ffffff;
  letter-spacing: 0.04em;
  font-size: 1.7rem;
}

.brand-block :deep(.n-text) {
  color: rgba(222, 239, 255, 0.72);
}

.action-row {
  margin: 18px 0 16px;
}

.meta-row {
  margin-bottom: 16px;
}

.add-feed-btn {
  background: linear-gradient(135deg, #ff7a18 0%, #ff3d54 100%);
  color: #fff;
  border: 0;
  font-weight: 700;
  box-shadow: 0 12px 24px rgba(255, 80, 80, 0.3);
}

.add-feed-btn:hover {
  filter: brightness(1.04);
}

.settings-btn {
  background: rgba(255, 255, 255, 0.1);
  color: #e8f4ff;
  border: 1px solid rgba(213, 235, 255, 0.25);
}

.settings-btn:hover {
  background: rgba(255, 255, 255, 0.16);
}

.settings-btn:active,
.settings-btn:focus,
.settings-btn:focus-visible {
  background: rgba(255, 255, 255, 0.1);
  color: #e8f4ff;
  outline: none;
  box-shadow: none;
}

.settings-btn {
  -webkit-tap-highlight-color: transparent;
}

.main-panel {
  height: 100vh;
  padding: 24px;
  overflow-y: auto;
}

.feed-panel {
  min-height: auto;
  border-radius: 20px;
  background: rgba(255, 255, 255, 0.74);
  backdrop-filter: blur(6px);
  box-shadow: 0 16px 40px rgba(28, 65, 96, 0.12);
}

.panel-title {
  margin: 0;
  font-size: 1.5rem;
  color: #1d2a3b;
}

.add-source-modal,
.settings-modal {
  width: min(760px, 92vw);
  border-radius: 16px;
}

.preview-card {
  background: linear-gradient(145deg, #eef8ff 0%, #f7fbff 100%);
}

@media (max-width: 980px) {
  .app-shell {
    display: block;
  }

  .left-panel {
    width: 100% !important;
    height: auto;
  }

  .feed-panel {
    min-height: auto;
  }
}
</style>
