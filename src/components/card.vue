<script>
export default {
    data() {
        return {
            title: 'lorem'
        }
    }
}
</script>


<template>
    <div class="card">
        <div class="title">
            <h1>{{ title }}</h1>
        </div>
        <div class="image"> 
            <img class="image" src="https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwww.thoughtco.com%2Fthmb%2Fe6FEm0_xDZ_kiajFKMFrBO6hgb0%3D%2F1500x0%2Ffilters%3Ano_upscale()%3Amax_bytes(150000)%3Astrip_icc()%2F5925354646_581f193d2c_o-56a28a335f9b58b7d0cbebe0.jpg&f=1&nofb=1&ipt=d8a5dbab505e5f9caae7952d248356172c1547b8a09fde16b1e975c0fca349ab&ipo=images" />
        </div>
        <div class="abstract">
            Lorem Ipsum is simply dummy text of the printing and  typesetting industry. 
            Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of  type and scrambled it to make a type specimen book. 
            It has survived not  only five centuries, but also the leap into electronic typesetting,  remaining essentially unchanged=
        </div>
    </div>
</template>

<style>

.card{
    background-color: #73AD21;
    border-radius: 10px;
    width: 800px;
}
.image {
    width: 400px;
}
.abstract {
    font-size: 20px;
}

</style>
