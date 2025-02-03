<script lang="ts"> 
import sidebar from './components/sidebar.vue';
import reader from './components/reader.vue';
import feeds from './components/feeds.vue';
import settings from './components/settings.vue';
import { NModal, NButton, NCard, NInput, NLayout, NLayoutSider } from 'naive-ui';
import { ref, inject, defineComponent } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

interface Source {
    title?: string;
    link?: string;
    description?: string;
}

export default defineComponent ({
    components: {
        sidebar,
        reader,
        feeds,
        settings,
        NModal,
        NCard,
        NInput,
        NLayout,
        NButton,
        NLayoutSider
    },
    setup() {
        const feed = ref<string>({});
        const feeds_list = ref<string[]>([]);
        const sources = ref<string[]>([]);
        const newSource = ref<Source>({});
        const newSourceInput  = ref<string>("");
        const showSettings = ref<boolean>(false);

        const onSetting = listen('settings', (event) => {
            showSettings.value = true;  
        });
        
        return {
            showModal: ref<boolean>(false),
            showReader: ref<boolean>(false),
            feed,
            feeds_list,
            sources,
            newSourceInput,
            newSource,
            showSettings
        };
    },
    methods: {
        handleNewRSS(message: string){
            console.log(message)
            this.showModal = true;
        },
        handleReading(message: string) {
            this.feed = message;
            this.showReader = true;
        },
        handleCloseReader(message: string) {
            this.showReader = false;
        },
        
        refresh() {
            this.feeds_list = [];
            for(let i = 0;i<this.sources.length;i++){
                invoke<string[]>('example_feed', { url: this.sources[i].link })
                    .then(response => {
                        for(let j= 0; j < response.length;j++) {
                            this.feeds_list[this.feeds_list.length] = JSON.parse(response[j]);
                        }
                    })
                    .catch(err => console.log(err));
            }
        },
        changeSource(sources: Feed[]) {
            this.sources = sources;
            this.refresh()
        },
        onChange() {
            invoke<string>('getSourceInfo', {url: newSourceInput.value})
                .then((msg)=> {
                    newSource.value = JSON.parse(msg)
                })
                .catch(err => console.log(err));
        },
        onNegativeClick() {
            console.log("Cancel")
        },
        onPositiveClick() { 
            invoke('addSource', {title: newSource.value.title, link: newSource.value.link, description: newSource.value.description});
        },
        handleSettingsClose(msg) {
            this.showSettings = false;
        }
    }
})
</script>

<template>
    <div style="position: relative; z-index: 1;">  
    <n-layout style="height: 720px; position: relative;">
        <n-layout position="absolute" has-sider>
            <n-layout-sider content-sytle="padding: 24px;" :native-scrollbar="false">
                <n-button @click="refresh">Refresh</n-button>
                <sidebar @messageSent="handleNewRSS" @changeSource="changeSource" />
            </n-layout-sider>
            
            <n-layout content-styel="padding: 24px;" :native-scrollbar="false">
                <feeds @MessageSent="handleReading" v-bind:feeds_list="feeds_list" />
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
                <n-input @change="onChange" v-model:value="newSourceInput"></n-input>
            </n-modal>
        </n-layout>
    </n-layout>
    </div>
    <div style="position: absolute; z-index: 2; top:1vh; left:1vw; right:1vw; border-radius: 10px;">
        <reader v-bind:feed="feed"  v-if="showReader" @messageSent="handleCloseReader"/>
    </div>

    <div style="position: absolute; z-index: 2; top:1vh; left:1vw; right:1vw; border-radius: 10px;">
        <settings v-if="showSettings" @messageSent="handleSettingsClose" />
    </div>
</template>

<style>
</style>
