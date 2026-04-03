<script setup lang="ts">
import { ref } from 'vue';
import type { Component } from 'vue';
import { NCard, NDivider, NMenu } from 'naive-ui';
import type { MenuOption } from 'naive-ui';
import GeneralSettings from './settings/general.vue';
import AppearanceSettings from './settings/appearance.vue';
import AboutSettings from './settings/about.vue';

const emit = defineEmits<{
  (event: 'close'): void;
}>();

type SettingsTab = 'general' | 'appearance' | 'about';

const selectedMenu = ref<SettingsTab>('general');

const menuOptions: MenuOption[] = [
  { label: '一般', key: 'general' },
  { label: '外觀', key: 'appearance' },
  { label: '關於', key: 'about' },
];

const contentMap: Record<SettingsTab, Component> = {
  general: GeneralSettings,
  appearance: AppearanceSettings,
  about: AboutSettings,
};
</script>

<template>
  <n-card class="settings-shell" :bordered="false" closable @close="emit('close')">
    <div class="settings-layout">
      <aside class="settings-menu">
        <n-menu v-model:value="selectedMenu" :options="menuOptions" />
      </aside>

      <n-divider vertical />

      <main class="settings-content">
        <component :is="contentMap[selectedMenu]" />
      </main>
    </div>
  </n-card>
</template>

<style scoped>
.settings-shell {
  border-radius: 16px;
  background: linear-gradient(160deg, #f6fbff 0%, #ffffff 45%, #f8fbff 100%);
}

.settings-layout {
  display: flex;
  min-height: 380px;
}

.settings-menu {
  width: 180px;
}

.settings-content {
  flex: 1;
  padding-left: 8px;
}

@media (max-width: 760px) {
  .settings-layout {
    flex-direction: column;
    gap: 10px;
  }

  .settings-menu {
    width: 100%;
  }

  .settings-content {
    padding-left: 0;
  }
}
</style>
