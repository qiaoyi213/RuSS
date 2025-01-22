<script>
import following from './following.vue';
export default {
    components: {
        following,
    }
}
</script>

<template>
    <div class="sidebar">
        <h1>RuSS</h1>
        <button class="new_button">+</button>
        <following />
    </div> 
</template>

<style>
.sidebar {
    background-color: #27282a;
    width: 255pt;
    height: 720pt;
}
</style>

