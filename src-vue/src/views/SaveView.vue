<template>

  <main>

  </main>

</template>

<script lang="ts">
const { invoke } = window.__TAURI__.core

invoke("list_save").then((info: any) => {
  console.log(info)
})

export default{
  components: {
  },
  methods: {
  },
  data()
  {
    return{
    }
  }
}
</script>

<style>

@keyframes moveBackground {
  0% {background-position: 100%;}
  100% {background-position: 0%;}
}

h1, h2{
  text-align: center;
  margin: 0;
  color: white;
}
#title{
  width: 100vw;
  background-color: rgba(0, 0, 0, 0.5);
  padding: 1rem;
  box-sizing: border-box;
  margin-bottom: 5rem;
  backdrop-filter: blur(0.25rem);
}

#panel{
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  width: 100%; height: 100vh;
  background-image: url("/sunbaekun_rose.png");
  background-repeat: repeat;
  background-size: 50%;
  animation: moveBackground 10s linear infinite;
}
#btn_case{
  display: flex;
  gap: 0.5rem;
  flex-direction: column;
}


</style>
