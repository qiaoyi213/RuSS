<script lang="ts">
import { NCard, NList, NListItem, NScrollbar, NDrawer, NDrawerContent, NButton } from 'naive-ui'
import { ref, defineProps, defineComponent } from 'vue';
import reader from './reader.vue';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { Readability } from '@mozilla/readability';
import { open } from '@tauri-apps/plugin-shell';

export default defineComponent ({
    emits: [
        'MessageSent'
    ],
    components: {
        NCard,
        NList,
        NListItem,
        NScrollbar,
        NDrawer,
        NDrawerContent,
        NButton,
        reader
    },
    methods: {
        focusReading (message: string) {
            this.$emit("MessageSent", message);
            this.active = false;
        }
    },
    props: {
        'feeds_list': Array as () => any[]
    },
    setup(props) {
        const active = ref<boolean>(false);
        const reading = ref<boolean>(false);
        const nowReading = ref<Record<string, any>>({});
        const readHtml = ref<string>("");
        const feed = ref<Record<string, any>>({});

        const read_feed = async (feed_url: string) => {
            active.value = true;

            try {
                const response = await invoke<string>('getFeed', {url: feed_url});
                console.log(response)
                let htmlText = response;
                const parser = new DOMParser();
                const doc = parser.parseFromString(htmlText, 'text/html');
                const reader = new Readability(doc); 
                let article = reader.parse();
                if (article) {
                    console.log("title:", article.title);
                    console.log("content:", article.content);
                    readHtml.value = article.content;
                    feed.value["title"] = article.title;
                    feed.value["content"] = article.content;
                    feed.value["link"] = feed_url;
                } else {
                    console.log("Could not extract the article");
                }
            } catch (err) {
                console.log(err);
            }
        } 
        const openURL = async (feed_url: string) => {
            console.log(feed_url)
            open(feed_url)
        } 
        return {
            active,
            reading,
            read_feed,
            nowReading,
            readHtml,
            feed,
            openURL
        }
    }
})
</script>

<template>
    <n-list hoverable clickable>
        <n-list-item v-for="feed in feeds_list" :key="feed.title">
            <n-card v-bind:title="feed.title" style="width:500px; margin-bottom: 20px;" @click="read_feed(feed.link)">
                <div style="font-size: 16px; color: #555;">{{ feed.description }}</div>
            </n-card>
        </n-list-item>
    </n-list>

    <n-drawer v-model:show="active" resizable :default-width="600" placement="right">
        <n-drawer-content v-bind:title="feed['title']" closable :native-scrollbar="false">
            <n-button @click="openURL(feed['link'])">閱讀原文</n-button>
            <div v-html="readHtml" style="line-height: 1.6; color: #333; padding: 10px;"></div>
        </n-drawer-content>
    </n-drawer>
</template>

<style>
.feeds {
    background-color: #73AD21;
    border-radius: 10px;
    width: 600px;
    max-height: 100%;
    padding: 15px;
}
.image {
    width: 100px;
}
.abstract {
    font-size: 20px;
}
</style>
