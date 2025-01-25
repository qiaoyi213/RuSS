<script>
import { NCard, NList, NListItem, NScrollbar, NDrawer, NDrawerContent, NButton } from 'naive-ui'
import { ref } from 'vue';
import reader from './reader.vue';

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
            feeds_list: [
                {
                    title: "First Feed",
                    abstract: "This is the first feed",
                    source: "https://abc.com",
                    content: "The content, first feed",
                }, 
                {
                    title: "Second Feed",
                    abstract: "This is the second feed",
                    source: "https://cde.com",
                    content: "The content, second feed",

                }
            ]
        }
    },
    methods: {
        focusReading () {
            this.$emit("MessageSent", true);
            this.active = false;
        }
    },
    setup() {
        const active = ref(false);
        const reading = ref(false);
        const read_feed = (feed_url) => {
                // channel = invoke('getFeedByUrl', feed_url);
                active.value = true;
        }

        const reading_mode = () => {
            active.value = false;
            reading.value = true;
            $emit('readingMode', true);
        }

        return {
            active,
            reading,
            read_feed,
            reading_mode
        }
    }
}
</script>


<template>
        <n-list hoverable clickable>
            <n-list-item v-for="feed in feeds_list">
                <n-card v-bind:title="feed.title" style="width:500px;" @click="read_feed(feed.title)">
                    <template #cover>
                        <img class="image" src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.thoughtco.com%2Fthmb%2Fe6FEm0_xDZ_kiajFKMFrBO6hgb0%3D%2F1500x0%2Ffilters%3Ano_upscale()%3Amax_bytes(150000)%3Astrip_icc()%2F5925354646_581f193d2c_o-56a28a335f9b58b7d0cbebe0.jpg&f=1&nofb=1&ipt=d8a5dbab505e5f9caae7952d248356172c1547b8a09fde16b1e975c0fca349ab&ipo=images" />
                    </template>
                    {{ feed.abstract }}
                </n-card>
            </n-list-item>
        </n-list>

        <n-drawer v-model:show="active" :width="502" :placement="right">
            <n-drawer-content title="Feed" closable :native-scrollbar="false">
                abc    
                <n-button @click="focusReading">
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
