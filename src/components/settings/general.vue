<script setup lang="ts">
import { ref } from 'vue';
import { NForm, NFormItem, NSelect, NSwitch } from 'naive-ui';

const languageVal = ref('zh_TW');
const startupRefresh = ref(true);

const languageOptions = [
  {
    label: '正體中文（臺灣）',
    value: 'zh_TW',
  },
  {
    label: 'English',
    value: 'en_US',
  },
];
</script>

<template>
  <n-form label-placement="left" label-width="120">
    <n-form-item label="語言">
      <n-select v-model:value="languageVal" :options="languageOptions" />
    </n-form-item>

    <n-form-item label="啟動即更新">
      <n-switch v-model:value="startupRefresh" />
    </n-form-item>
  </n-form>
</template>
