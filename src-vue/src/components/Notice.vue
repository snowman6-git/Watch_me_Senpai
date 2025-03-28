<script setup lang="ts">
defineProps({
  text: {
    type: String,
    required: true,
  },
})
import ActionBtn from './ActionBtn.vue'
</script>

<template>
  <main id="notice">
    
    <div id="text">{{ text }}</div>
  </main>
</template>

<style scoped>
  #notice{
    color: white;
    width: 25rem; height: 40rem;
    border-radius: 0.5rem;
    background-color: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(1rem);
    transition: 0.25s all;
    position: fixed;

    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    z-index: 99;
  }

  #text{
    width: 100%; height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

</style>