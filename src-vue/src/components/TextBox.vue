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
    width: 100%; height: auto;
    padding: 1rem;
    color: white;
    bottom: 4rem;
    position: fixed;
    background-color: rgba(0, 0, 0, 0.5);
  }


  #text{
    width: 90%; height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

</style>