<script>
import { NButton } from "naive-ui";

export default {
    components: {
        NButton
    },
    data() {
        return {
            followings: [
                {
                    name: "NHK",
                    url: "https://some.com/rss.xml"
                },
                {
                    name: "CNN",
                    url: "https://some.com/rss.xml"
                }
            ]
        }
    }
}

</script>

<template>
    <div class="head"> 
        <h1>Following</h1>
    </div>
    <div class="followings">
        <div class="following" v-for="following in followings">
            <n-button> {{ following.name }} </n-button>
        </div>
    </div>
</template>

<style>
.head {
    background-color: red;
}
.following {
    border-radius: 2px;
    background-color: white;
}

</style>
