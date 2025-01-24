<script> 
import sidebar from './components/sidebar.vue';
import feed from './components/feed.vue';
import feeds from './components/feeds.vue';
import { NModal, NCard, NInput } from 'naive-ui';
import { ref } from 'vue';
export default {
    components: {
        sidebar,
        feed,
        feeds,
        NModal,
        NCard,
        NInput
    },
    setup() {
        return {
            showModal: ref(false),
            onNegativeClick() {
                console.log("A")
            },
            onPositiveClick() {
                console.log("G")
            }
        }
    },
    methods: {
        handleNewRSS(message){
            console.log(message)
            this.showModal = true;
            console.log(this.showModal)
        }
    }
}
</script>

<template>
    <div id="app" class="column">
        <div class="sidebar">
            <sidebar @messageSent="handleNewRSS" />
        </div>
        <div class="card">
            <feeds />
        </div>
        <n-modal v-model:show="showModal"
            preset="dialog"
            style="width:600px"
            title="Add new RSS source"
            positive-text="Add it!"
            negative-text="Cancel"
            @positive-click="onPositiveClick"
            @negative-click="onNegativeClick"

        >
            <n-input>
            </n-input>
        </n-modal>
    </div>
</template>

<style>
.sidebar{
    float: left;
}
.feeds{
    float: left;
}
.feed {
    float: left;
}
</style>
