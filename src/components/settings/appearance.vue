<script setup lang="ts">
import { ref } from 'vue';
import { NColorPicker, NForm, NFormItem, NSlider, NSwitch } from 'naive-ui';
import { useTheme } from '../../composables/theme';

const compactMode = ref(false);
const cardRoundness = ref(14);
const accentColor = ref('#1f95be');
const { isDarkMode } = useTheme();
</script>

<template>
  <n-form label-placement="left" label-width="120">
    <n-form-item label="深色模式">
      <n-switch v-model:value="isDarkMode" />
    </n-form-item>

    <n-form-item label="緊湊模式">
      <n-switch v-model:value="compactMode" />
    </n-form-item>

    <n-form-item label="卡片圓角">
      <n-slider v-model:value="cardRoundness" :min="6" :max="24" />
    </n-form-item>

    <n-form-item label="主色">
      <n-color-picker v-model:value="accentColor" :show-alpha="false" />
    </n-form-item>
  </n-form>
</template>
