<script lang="ts">
import { NCard, NLayout, NLayoutSider, NLayoutContent, NMenu } from 'naive-ui';
import { ref, defineComponent, h} from 'vue';
import general from './settings/general.vue';
import appearance from './settings/appearance.vue';
import about from './settings/about.vue';


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
    'general': general,
    'appearance': appearance,
    'about': about
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
        const selectedMenu = ref('general');

        return {
            menuOptions,
            contentComponents,
            selectedMenu,
        } 
    },
    methods: {
        handleClose() {
            this.$emit('messageSent', 'settingsClose');
        },
        handleMenuChange(msg) {
            this.selectedMenu = msg
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
                <component v-bind:is="contentComponents[selectedMenu]" />
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

