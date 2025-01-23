<script> 
import sidebar from './components/sidebar.vue';
import feed from './components/feed.vue';
import { NButton } from 'naive-ui';
export default {
    components: {
        sidebar,
        feed
    },
}
</script>

<template>
    <div id="app" class="column">
        <div class="sidebar">
            <sidebar />
        </div>
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
