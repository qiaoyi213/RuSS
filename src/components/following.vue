<script setup lang="ts">
import { computed } from 'vue';
import { NButton, NDropdown, NEmpty, NFlex, NScrollbar, NText } from 'naive-ui';

interface Source {
  title: string;
  link: string;
  description: string;
}

const props = defineProps<{
  sources: Source[];
  selectedSource: Source | null;
}>();

const emit = defineEmits<{
  (event: 'select-source', source: Source): void;
  (event: 'clear-source'): void;
  (event: 'delete-source', source: Source): void;
}>();

const hasSources = computed(() => props.sources.length > 0);
const deleteOption = [{ label: '刪除來源', key: 'delete' }];

function onDropdownSelect(key: string, source: Source): void {
  if (key === 'delete') {
    emit('delete-source', source);
  }
}
</script>

<template>
  <section class="following-panel">
    <n-flex justify="space-between" align="center" class="following-header">
      <h3>Following</h3>
      <n-button quaternary size="small" @click="emit('clear-source')">全部</n-button>
    </n-flex>

    <n-scrollbar style="max-height: calc(100vh - 310px)">
      <div v-if="hasSources" class="source-list">
        <div v-for="source in sources" :key="source.title" class="source-item">
          <button
            class="source-button"
            :class="{ 'source-button--active': selectedSource?.title === source.title }"
            @click="emit('select-source', source)"
          >
            <strong>{{ source.title }}</strong>
            <n-text depth="3" class="source-desc">{{ source.description }}</n-text>
          </button>

          <n-dropdown trigger="click" :options="deleteOption" @select="onDropdownSelect($event as string, source)">
            <n-button quaternary class="source-action">⋯</n-button>
          </n-dropdown>
        </div>
      </div>

      <n-empty v-else description="尚未有 RSS 來源" class="empty-state" />
    </n-scrollbar>
  </section>
</template>

<style scoped>
.following-panel {
  background: var(--following-panel-bg);
  border: 1px solid var(--following-panel-border);
  border-radius: 14px;
  padding: 12px;
}

.following-header h3 {
  margin: 0;
  color: var(--following-header-color);
  font-size: 1rem;
  letter-spacing: 0.02em;
}

.source-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.source-item {
  display: flex;
  gap: 6px;
  align-items: stretch;
}

.source-button {
  flex: 1;
  text-align: left;
  border: 0;
  cursor: pointer;
  background: var(--source-button-bg);
  color: var(--source-button-color);
  border-radius: 10px;
  padding: 10px 12px;
  transition: transform 0.15s ease, background-color 0.2s ease;
}

.source-button:hover {
  transform: translateY(-1px);
  background: var(--source-button-bg-hover);
}

.source-button--active {
  background: var(--source-button-active-bg);
  color: var(--source-button-active-color);
}

.source-desc {
  display: -webkit-box;
  margin-top: 3px;
  font-size: 0.78rem;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.source-action {
  width: 36px;
  border-radius: 10px;
  color: var(--source-action-color);
  background: var(--source-action-bg);
}

.empty-state {
  margin: 18px 0;
}
</style>
