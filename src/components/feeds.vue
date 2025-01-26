<script>
import { NCard, NList, NListItem, NScrollbar, NDrawer, NDrawerContent, NButton } from 'naive-ui'
import { ref } from 'vue';
import reader from './reader.vue';
import { invoke } from '@tauri-apps/api/core';
import { fetch } from '@tauri-apps/plugin-http';
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
    data() {
        return {
            card_title: "abc",
        }
    },
    methods: {
        focusReading (message) {
            console.log(message);
            this.$emit("MessageSent", message);
            this.active = false;
        },
    },
    setup() {
        const active = ref(false);
        const reading = ref(false);
        const emit_msg = ref("");
        const rss_source = ref(["https://feeds.feedburner.com/rsscna/intworld"]);
        const nowReading = ref({});
        const read_feed = async (feed_url) => {
                // channel = invoke('getFeedByUrl', feed_url);
                emit_msg.value = feed_url; 
                    
                fetch(feed_url, {
                    method: 'GET',
                }).then(async (response) => {
                    const reader = response.body.getReader();
                    const decoder = new TextDecoder();
                    let result = '';
                    while(true) {
                        const { done, value } = await reader.read();
                        if(done) {
                            break;
                        }
                        result += decoder.decode(value, { stream: true });
                    }
                    result += decoder.decode();
                    console.log(result);
                });
                active.value = true;
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
            emit_msg,
            refresh,
            feeds_list,
            nowReading
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
            <n-drawer-content v-bind:title="emit_msg" closable :native-scrollbar="false">
                <n-button @click="focusReading(emit_msg)">
                    Focus
                </n-button>
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
