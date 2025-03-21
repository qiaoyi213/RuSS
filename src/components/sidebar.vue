<script lang="ts">
import { ref, defineComponent } from 'vue';
import following from './following.vue';
import { NButton } from 'naive-ui';

export default defineComponent ({
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
        changeSource(source: any) {
            this.$emit('changeSource', source)  
        }
    }
})
</script>

<template>
    <div class="sidebar">
        <div class="following-list">
            <following @changeSource="changeSource" />
        </div>
    </div> 
</template>

<style>
body {
    height: 100%;
    padding: 0;
    margin: 0;
    font-family: 'Arial', sans-serif;
    background-color: #f4f4f9;
}

.sidebar {
    background-color: #27282a;
    width: 220px;
    height: 100vh;
    padding: 15px;
    box-shadow: 2px 0 5px rgba(0,0,0,0.1);
    display: flex;
    flex-direction: column;
    align-items: center;
}

.sidebar-title {
    color: #fff;
    text-align: center;
    font-size: 20px;
    margin-bottom: 15px;
}

.sidebar-button {
    margin: 5px 0;
    background-color: #4CAF50;
    color: white;
    width: 100%;
    border-radius: 5px;
}

.following-list {
    width: 100%;
    overflow-y: auto;
    margin-top: 10px;
}
</style>
