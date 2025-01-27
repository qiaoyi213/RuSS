<script>
import { NCard, NButton, NScrollbar } from 'naive-ui';
import { ref } from 'vue';
export default {
    components: {
        NCard,
        NButton,
        NScrollbar
    }, 
    methods: {
        closeReader() {
            console.log("close");
            this.$emit("messageSent", "closeReader");
        } 
    },
    props: [
        'feed',
    ]
}
</script>

<template>
    <n-card>
        <n-scrollbar style="max-height: 800px;">
        <div>
            {{ feed['title'] }}
        </div>
        <div v-html="feed['content']"></div>

        <n-button @click="closeReader">Close</n-button> 
        </n-scrollbar>
    </n-card>
</template>

