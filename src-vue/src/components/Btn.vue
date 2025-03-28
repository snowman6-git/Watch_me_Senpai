<script setup>
defineProps({
  text: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <main>
    <div id="text">{{ text }}</div>
  </main>
</template>

<style scoped>

  main{
    width: 10rem; height: 2.5rem;
    color: white;
    /* border: 0.2rem black solid; */
    border-radius: 0.5rem;
    background-color: gray;
    transition: 0.25s all;
  }

  main:hover{
    /* transform: scale(1.2); */
    /* transform: translateY(-1.5rem) */
    background-color: green;
    /* filter: invert(100%); */
  }

  #text{
    width: 100%; height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

</style>