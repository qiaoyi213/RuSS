<script>
import { NLayout, NLayoutSider, NLayoutContent } from 'naive-ui';

export default {
    components: {
        NLayout,
        NLayoutSider,
        NLayoutContent
    },
    setup() {

    }
}
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
