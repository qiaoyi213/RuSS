<script> 
import sidebar from './components/sidebar.vue';
import feed from './components/feed.vue';
import feeds from './components/feeds.vue';
import { NModal, NCard, NInput, NLayout, NLayoutSider } from 'naive-ui';
import { ref } from 'vue';
export default {
    components: {
        sidebar,
        feed,
        feeds,
        NModal,
        NCard,
        NInput,
        NLayout,
        NLayoutSider
    },
    setup() {
        return {
            showModal: ref(false),
            onNegativeClick() {
                console.log("A")
            },
            onPositiveClick() {
                console.log("G")
            }
        }
    },
    methods: {
        handleNewRSS(message){
            console.log(message)
            this.showModal = true;
            console.log(this.showModal)
        }
    }
}
</script>

<template>
    <n-layout style="height: 720px; position: relative">
        <n-layout position="absolute" has-sider>
            <n-layout-sider content-sytle="padding: 24px;" :native-scrollbar="false">
                <sidebar @messageSent="handleNewRSS" />
            </n-layout-sider>

            <n-layout content-styel="padding: 24px;" :native-scrollbar="false">
                <feeds/>
            </n-layout>

            <n-modal v-model:show="showModal"
                preset="dialog"
                style="width:600px"
                title="Add new RSS source"
                positive-text="Add it!"
                negative-text="Cancel"
                @positive-click="onPositiveClick"
                @negative-click="onNegativeClick"
            >
                <n-input></n-input>
            </n-modal>
        </n-layout>
    </n-layout>
</template>

<style>
</style>
