<script lang="ts">
import { NButton, NMenu, NDropdown } from "naive-ui";
import { invoke } from '@tauri-apps/api/core';
import { ref, defineEmits, defineComponent, reactive } from 'vue';

export default defineComponent ({
    emits: [
        'changeSource',
    ],
    components: {
        NButton,
        NMenu,
        NDropdown
    },
    methods: {
        initSource(){
            this.$emit('changeSource', this.sources);   
        },
        sourcesClick(source: any) {
            this.$emit('changeSource', [source]); 
        },
        sourceHandler(sourceTitle: string) {
            this.dropdownStatus[sourceTitle] = !this.dropdownStatus[sourceTitle];
        }
    },
    setup(props, context) {
        const sources = ref<{ sources: any[] }>({ sources: [] });
        const selectedSource = ref<any>(null);
        const dropdownStatus = reactive({});;

        const selectSource = (source: any) => {
            selectedSource.value = source;
            context.emit('changeSource', [source]);
        };

        const clearSelection = () => {
            selectedSource.value = null;
            context.emit('changeSource', sources.value.sources);
        };
        

        const handleSelect = (event, sourceTitle:string) => {
            console.log(event)
            console.log(sourceTitle)
            if(event == "Delete") {
                invoke('deleteSource', {title: sourceTitle})
            }
        }

        const options = [
            {
                label: '刪除',
                key: 'Delete',
                disabled: false
            },
        ]

        invoke<string>('getSources')
            .then((response: string) => {
                sources.value = JSON.parse(response); 
                console.log(sources.value.sources.length); 
                for(let i=0;i<sources.value.sources.length;i++){
                    console.log(sources.value.sources[i])
                    dropdownStatus[sources.value.sources[i]['title']] = ref(false);
                    console.log(dropdownStatus)
                }
                context.emit('changeSource', sources.value.sources); 
            })
            .catch((error: any) => {
                console.error(error)
            }); 
        
        document.addEventListener('click', () => {
            for(let i=0;i<sources.value.sources.length;i++){
                    dropdownStatus[sources.value.sources[i]['title']] = ref(false);
            }
        })

        return {
            sources,
            selectedSource,
            selectSource,
            clearSelection,
            dropdownStatus,
            options,
            handleSelect
        }
    }
})

</script>

<template>
    <div class="head"> 
        <h1 class="following-title">Following</h1>
    </div>
    <div class="followings" @click="clearSelection">
        <div class="following" v-for="source in sources.sources" :key="source.title" style="margin: 5px;">
            <n-dropdown :show="dropdownStatus[source.title]" :options="options" @select="handleSelect($event, source.title)">
                <n-button @contextmenu.prevent="sourceHandler(source.title)" @click.stop="selectSource(source)" :class="{'following-button': true, 'selected': selectedSource === source}">
                    {{ source.title }}
                </n-button>
            </n-dropdown>
        </div>
    </div>
</template>

<style>
.head {
    background-color: #333;
    padding: 10px;
    border-radius: 5px;
    text-align: center;
}
.following-title {
    color: white;
    font-size: 22px;
    margin: 0;
}
.following {
    border-radius: 5px;
    padding: 5px;
}
.following-button {
    background-color: #007BFF;
    color: white;
    width: 100%;
    border-radius: 5px;
    transition: background-color 0.3s ease;
}
.following-button:hover {
    background-color: #0056b3;
}
.selected {
    background-color: #0056b3 !important;
}
.followings {
    padding: 10px;
    background-color: #f4f4f9;
    border-radius: 10px;
    max-height: 400px;
    overflow-y: auto;
}
</style>
