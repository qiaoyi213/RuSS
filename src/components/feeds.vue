<script>
import { NCard, NList, NListItem, NScrollbar} from 'naive-ui'
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
        greet
    },
    data() {
        return {
            card_title: "abc",
            feeds_list: [
                {
                    "title": "First Feed",
                    "abstract": "This is the first feed",
                    "content": "The content, first feed",
                }, 
                {
                    "title": "Second Feed",
                    "abstract": "This is the second feed",
                    "content": "The content, second feed"
                }
            ]
        }
    },
}
</script>


<template>
    <n-scrollbar class="feeds" style="max-height: 720px">
        <n-list hoverable clickable>
            <n-list-item v-for="feed in feeds_list">
                <n-card v-bind:title="feed.title" @click="greet">
                    <template #cover>
                        <img class="image" src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.thoughtco.com%2Fthmb%2Fe6FEm0_xDZ_kiajFKMFrBO6hgb0%3D%2F1500x0%2Ffilters%3Ano_upscale()%3Amax_bytes(150000)%3Astrip_icc()%2F5925354646_581f193d2c_o-56a28a335f9b58b7d0cbebe0.jpg&f=1&nofb=1&ipt=d8a5dbab505e5f9caae7952d248356172c1547b8a09fde16b1e975c0fca349ab&ipo=images" />
                    </template>
                    {{ feed. abstract }}
                </n-card>
            </n-list-item>
        </n-list>
    </n-scrollbar>
</template>

<style>

.feeds {
    background-color: #73AD21;
    border-radius: 10px;
    width: 800px;
}
.image {
    width: 100px;
}
.abstract {
    font-size: 20px;
}

</style>
