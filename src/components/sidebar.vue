<script>
import { ref } from 'vue';
import following from './following.vue';
import { NButton } from 'naive-ui';

export default {
    components: {
        following,
        NButton
    },
    methods: {
        showModal (){
            this.$emit('messageSent', 'Hello Parent');
        }
    }
}
</script>

<template>
    <div class="sidebar">
        <h1>RuSS</h1>
        <n-button @click="showModal">
            New RSS
        </n-button>

        <following />
    </div> 
</template>

<style>
.sidebar {
    background-color: #27282a;
    width: 255pt;
    height: 720pt;
}
</style>

