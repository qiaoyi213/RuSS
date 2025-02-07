<script lang="ts">
import { defineComponent, ref } from 'vue';
import { NSelect, NSwitch } from 'naive-ui';  

export default defineComponent({
    components: {
        NSelect,
        NSwitch
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
                key: "en_US"
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
