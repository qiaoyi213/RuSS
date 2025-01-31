<script>
import { ref } from 'vue';
import following from './following.vue';
import { NButton } from 'naive-ui';

export default {
    emits:[
        'changeSource',
        'messageSent'
    ],
    components: {
        following,
        NButton
    },
    methods: {
        showModal (){
            this.$emit('messageSent', 'Hello Parent');
        },

        changeSource(source) {
            this.$emit('changeSource', source)  

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

        <following @changeSource="changeSource" />
    </div> 
</template>

<style>
.sidebar {
    background-color: #27282a;
    width: 255pt;
    max-height: 100%;
}
</style>

