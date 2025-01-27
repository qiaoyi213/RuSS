<script>
import { NCard, NButton } from 'naive-ui';
import { ref } from 'vue';
export default {
    components: {
        NCard,
        NButton
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
        <h1>{{feed['title']}}</h1>
        <div v-html="feed['content']"></div>
        <n-button @click="closeReader">Close</n-button> 
</template>

