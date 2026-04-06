<script setup lang="ts">
import Following from './following.vue';

interface Source {
  title: string;
  link: string;
  description: string;
}

defineProps<{
  sources: Source[];
  selectedSource: Source | null;
}>();

const emit = defineEmits<{
  (event: 'select-source', source: Source): void;
  (event: 'clear-source'): void;
  (event: 'delete-source', source: Source): void;
}>();
</script>

<template>
  <div class="sidebar">
    <following
      :sources="sources"
      :selected-source="selectedSource"
      @select-source="emit('select-source', $event)"
      @clear-source="emit('clear-source')"
      @delete-source="emit('delete-source', $event)"
    />
  </div>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  min-height: 0;
}
</style>
