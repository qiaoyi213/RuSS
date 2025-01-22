<script> 
import sidebar from './components/sidebar.vue';
import Feeds from './Feeds.vue';
import feed from './components/feed.vue';
export default {
    components: {
        sidebar,
        Feeds,
        feed
    },
}
</script>

<template>
    <div id="app" class="column">
        <div class="sidebar">
            <sidebar />
        </div>
        <div class="feeds">
            <Feeds /> 
        </div>
        <div class="feed">
            <feed />
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
