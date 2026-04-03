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
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(211, 234, 255, 0.1);
  border-radius: 14px;
  padding: 12px;
}

.following-header h3 {
  margin: 0;
  color: #eff8ff;
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
  background: rgba(240, 250, 255, 0.08);
  color: #eaf5ff;
  border-radius: 10px;
  padding: 10px 12px;
  transition: transform 0.15s ease, background-color 0.2s ease;
}

.source-button:hover {
  transform: translateY(-1px);
  background: rgba(240, 250, 255, 0.14);
}

.source-button--active {
  background: linear-gradient(130deg, #1f95be 0%, #41b5d9 100%);
  color: #ffffff;
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
  color: #d6ecff;
  background: rgba(255, 255, 255, 0.04);
}

.empty-state {
  margin: 18px 0;
}
</style>
