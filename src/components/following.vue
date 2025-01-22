<script>
    export default {
        data() {
            return {
                followings: [
                    {
                        name: "NHK",
                    },
                    {
                        name: "CNN",
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
            <button>
                {{ following.name }}
            </button>
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
