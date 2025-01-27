<script> 
import sidebar from './components/sidebar.vue';
import reader from './components/reader.vue';
import feeds from './components/feeds.vue';
import { NModal, NCard, NInput, NLayout, NLayoutSider } from 'naive-ui';
import { ref } from 'vue';
export default {
    components: {
        sidebar,
        reader,
        feeds,
        NModal,
        NCard,
        NInput,
        NLayout,
        NLayoutSider
    },
    setup() {
        const feed = ref({});
        return {
            showModal: ref(false),
            showReader: ref(false),
            feed,
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
        },
        handleReading(message) {
            this.feed = message;
            this.showReader = true;
        },
        handleCloseReader(message) {
            this.showReader = false;
        }
    }
}
</script>

<template>
    <div style="position: relative; z-index: 1;">  
    <n-layout style="height: 720px; position: relative;">
        <n-layout position="absolute" has-sider>
            <n-layout-sider content-sytle="padding: 24px;" :native-scrollbar="false">
                <sidebar @messageSent="handleNewRSS" />
            </n-layout-sider>
            
            <n-layout content-styel="padding: 24px;" :native-scrollbar="false">
                <feeds @MessageSent="handleReading" />
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
    </div>
    <div style="position: absolute; z-index: 2; top:50px; left:50px; background-color:white; border-radius: 10px;">
        <reader v-bind:feed="feed" style="position: absolute; z-index: 2; top: 50px; left:50px; width:800px; height: 600px;" v-if="showReader" @messageSent="handleCloseReader"/>
    </div>
</template>

<style>
</style>
