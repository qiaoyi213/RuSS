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
    <n-button @click="closeReader">Close</n-button> 
</template>

