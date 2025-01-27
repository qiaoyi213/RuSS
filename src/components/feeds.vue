<script>
import { NCard, NList, NListItem, NScrollbar, NDrawer, NDrawerContent, NButton } from 'naive-ui'
import { ref } from 'vue';
import reader from './reader.vue';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
import { Readability } from '@mozilla/readability';
function greet(event) {
  alert(`Hello ${name.value}!`)
  // `event` 是 DOM 原生事件
  if (event) {
    alert(event.target.tagName)
  }
}

export default {
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
    setup() {
        const active = ref(false);
        const reading = ref(false);
        const rss_source = ref(["https://feeds.feedburner.com/rsscna/intworld"]);
        const nowReading = ref({});
        const readHtml = ref("");
        const feed = ref({});
        const read_feed = async (feed_url) => {
                // channel = invoke('getFeedByUrl', feed_url);
                active.value = true;   

                fetch(feed_url, {
                    method: 'GET',
                }).then(async (response) => {
                    const htmlText = await response.text().then((res) => {return res});
                    console.log(htmlText);
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
                .catch(err => console.error("fetch error", err));
        } 
        const feeds_list = ref([
                {
                    title: "First Feed",
                    description: "This is the first feed",
                    link: "https://abc.com",
                    content: "The content, first feed",
                } 
            ]);
        const refresh = () => {
            invoke('example_feed', { url: "https://feeds.feedburner.com/rsscna/intworld"})
                .then(response => {
                    console.log(response[0])
                    for(let i = 0; i < response.length; i++){
                        feeds_list.value[i] = JSON.parse(response[i]);
                    }
                })
        }
               return {
            active,
            reading,
            read_feed,
            refresh,
            feeds_list,
            nowReading,
            readHtml,
            feed

        }
    }
}
</script>


<template>
        <n-button @click="refresh">Refresh</n-button>
        <n-list hoverable clickable>
            <n-list-item v-for="feed in feeds_list">
                <n-card v-bind:title="feed.title" style="width:500px;" @click="read_feed(feed.link)">
                    {{ feed.description }}
                </n-card>
            </n-list-item>
        </n-list>

        <n-drawer v-model:show="active" :width="502" :placement="right">
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
