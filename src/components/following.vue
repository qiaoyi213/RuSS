<script>
import { NButton, NMenu } from "naive-ui";
import { invoke } from '@tauri-apps/api/core';
import { ref, defineEmits } from 'vue';

export default {
    emits: [
        'changeSource'
    ],
    components: {
        NButton,
        NMenu,
    },
    data() {
        return {
            followings: [
            ]
        }
    },
    methods: {
        initSource(){
            this.$emit('changeSource', this.sources.value);   
        },
        sourcesClick(source) {
            this.$emit('changeSource', [source]); 
        }

    },
    setup(props, context) {
        const sources = ref({});
        invoke('getSources')
            .then(response => {
                sources.value = JSON.parse(response);
                context.emit('changeSource', sources.value.sources);
            }); 
        return {
            sources,
        }
    }
}

</script>

<template>
    <div class="head"> 
        <h1>Following</h1>
    </div>
    <div class="followings">
        <div class="following" v-for="source in sources.sources">
            <n-button @click="sourcesClick(source)"> {{ source.title }} </n-button>
        </div>
    </div>

</template>

<style>
.head {
    background-color: red;
}
.following {
    border-radius: 2px;
    background-color: green;
}

</style>
