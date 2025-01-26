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
        'abc'
    ],
    data() {
        return {
            feed: {
                title: "Some news title",
                abstract: "News abstract",
                content: "Lorem ipsum bababa",
            }
        }
    }
}
</script>

<template>
    <h1>{{feed.title}}</h1>
    <div v-html="abc"></div>
    <n-button @click="closeReader">Close</n-button> 
</template>

