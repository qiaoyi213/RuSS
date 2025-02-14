<script lang="ts">
import { NSwitch } from 'naive-ui';
import { ref, defineComponent } from 'vue';

export default defineComponent({
    components: {
        NSwitch
    },
    setup() {
        const darkMode = ref(false); 

        return {
            darkMode
        }
    }
})
</script>

<template>
    <h1>Appearance</h1>
    <p>Dark mode</p>
    <n-switch v-model:value="darkMode" />
    
</template>
