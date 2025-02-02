<script lang="ts">
import { NCard, NButton, NScrollbar, NAlert } from 'naive-ui';
import { ref, defineComponent} from 'vue';
export default defineComponent ({
    components: {
        NCard,
        NButton,
        NScrollbar,
        NAlert
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
})
</script>

<template>
    <n-card>
        <n-scrollbar style="max-height: 800px;">
        <div>
            {{ feed['title'] }}
        </div>
        <n-alert title="Warning" type="warning">
            This is provided by Mozilla Readability. If you have trouble when you reaeding, please click here to read in your browser.
        </n-alert>
        <div v-html="feed['content']"></div>

        <n-button @click="closeReader">Close</n-button> 
        </n-scrollbar>
    </n-card>
</template>

