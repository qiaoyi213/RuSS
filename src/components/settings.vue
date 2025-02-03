<script lang="ts">
import { NCard, NLayout, NLayoutSider, NLayoutContent, NMenu } from 'naive-ui';
import { ref, defineComponent } from 'vue';


const menuOptions: MenuOption[] = [
    {
        label: '一般',
        key: 'general'
    },
    {
        label: '外觀',
        key: 'appearance'
    },
    {
        label: '關於',
        key: 'about'
    }
];

const contentComponents = {
    'general': {

    },
    'appearance': {

    },
    'about': {

    }
};

export default defineComponent ({
    components: {
        NLayout,
        NCard,
        NLayoutSider,
        NLayoutContent,
        NMenu, 
    },
    setup() {
        const selectedComponent = ref('general');

        return {
            menuOptions,
            selectedComponent
        } 
    },
    methods: {
        handleClose() {
            this.$emit('messageSent', 'settingsClose');
        },
        handleMenuChange(msg) {
            console.log(msg)
            this.selectedComponent = msg
        }
    },

})
</script>


<template>
    <n-card id="settings-card" title="設定" closable @close="handleClose"> 
        <n-layout has-sider> 
            <n-layout-sider>
                <n-menu :options="menuOptions" @update:value="handleMenuChange"/>
            </n-layout-sider>
            <n-layout-content>
                <component :is="" />
            </n-layout-content>
        </n-layout>
    </n-card> 
</template>

<style>
#settings-card {
    max-width: 800px;
    background-color: #171717; 
}
</style>

