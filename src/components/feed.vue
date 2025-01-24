<script>
export default {
    components: {
    }, 
    data() {
        return {
            feed: {
                title: "Some news title",
                abstract: "News abstract",
                content: "Lorem ipsum bababa",
            }
        }
    }
}
</script>

<template>
    <div class="feed">
        <h1>Feed Example</h1>
    </div>
</template>

<style>
.feed {
    background-color: green;
    width: 50pt;
    height: 720pt;
}
</style>
