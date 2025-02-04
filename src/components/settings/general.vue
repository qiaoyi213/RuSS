<script lang="ts">
import { defineComponent, ref } from 'vue';
import { NSelect } from 'naive-ui';  

export default defineComponent({
    components: {
        NSelect
    },
    setup() {
        const languageVal = ref("");
        const languageOptions = [
            {
                label: "正體中文（臺灣）",
                key: "zh_TW"
            },
            {
                label: "English",
                key: "en"
            }
        ];
        return {
            languageVal,
            languageOptions
        }
    }
});
</script>
<template>
    <n-select v-model:value="languageVal" :options="languageOptions"/>
</template>

<style>

</style>
