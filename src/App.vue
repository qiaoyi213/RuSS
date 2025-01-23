<script> 
import sidebar from './components/sidebar.vue';
import feed from './components/feed.vue';
import feeds from './components/feeds.vue';
export default {
    components: {
        sidebar,
        feed,
        feeds
    },
}
</script>

<template>
    <div id="app" class="column">
        <div class="sidebar">
            <sidebar />
        </div>
        <div class="card">
            <feeds />
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
