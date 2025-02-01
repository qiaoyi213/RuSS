<script>
import { NCard, NList, NListItem, NScrollbar, NDrawer, NDrawerContent, NButton } from 'naive-ui'
import { ref, defineProps } from 'vue';
import reader from './reader.vue';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { Readability } from '@mozilla/readability';

export default {
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
        focusReading (message) {
            this.$emit("MessageSent", message);
            this.active = false;
        },
    },
    props: [
        'feeds_list'
    ],
    setup(props) {
        const active = ref(false);
        const reading = ref(false);
        const nowReading = ref({});
        const readHtml = ref("");
        const feed = ref({});
        const read_feed = async (feed_url) => {
            active.value = true;   
            invoke('getFeed', {url: feed_url})
                .then((response) => {
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
                    } else {
                        console.log("Cloud not extract the article");
                    }
                })
                .catch(err => console.log(err));      
        } 

        return {
            active,
            reading,
            read_feed,
            nowReading,
            readHtml,
            feed
        }
    }
}
</script>

<template>
        <n-list hoverable clickable>
            <n-list-item v-for="feed in feeds_list">
                <n-card v-bind:title="feed.title" style="width:500px;" @click="read_feed(feed.link)">
                    {{ feed.description }}
                </n-card>
            </n-list-item>
        </n-list>

        <n-drawer v-model:show="active" resizable :default-width="600" placement="right">
            <n-drawer-content v-bind:title="feed['title']" closable :native-scrollbar="false">
                <n-button @click="focusReading(feed)">
                    Focus
                </n-button>
                <div v-html="readHtml" ></div>
            </n-drawer-content>
        </n-drawer>
</template>

<style>

.feeds {
    background-color: #73AD21;
    border-radius: 10px;
    width: 600px;
    max-height: 100%;
}
.image {
    width: 100px;
}
.abstract {
    font-size: 20px;
}

</style>
