<script lang="ts">
import { NLayout, NLayoutSider, NLayoutContent } from 'naive-ui';
import { defineComponent } from 'vue';
export default defineComponent ({
    components: {
        NLayout,
        NLayoutSider,
        NLayoutContent
    },
})
</script>


<template>
    <n-layout has-sider>
        <n-layout-sider>
            General
        </n-layout-sider>
        <n-layout-content>
            Hello
        </n-layout-content>
    </n-layout>
</template>
