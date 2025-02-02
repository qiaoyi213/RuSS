<script lang="ts">
import { NButton, NMenu } from "naive-ui";
import { invoke } from '@tauri-apps/api/core';
import { ref, defineEmits, defineComponent } from 'vue';

export default defineComponent ({
    emits: [
        'changeSource'
    ],
    components: {
        NButton,
        NMenu,
    },
    methods: {
        initSource(){
            this.$emit('changeSource', this.sources.value);   
        },
        sourcesClick(source: any) {
            this.$emit('changeSource', [source]); 
        }

    },
    setup(props, context) {
        const sources = ref<Record<string, any>>({});
        invoke<string>('getSources')
            .then((response: string) => {
                sources.value = JSON.parse(response);
                context.emit('changeSource', sources.value.sources);
            })
            .catch((error: any) => {
                console.error(error)
            }); 
        return {
            sources,
        }
    }
})

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
