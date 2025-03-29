<script setup lang="ts">
defineProps({
  text: {
    type: String,
    required: true,
  },
})
</script>

<template>
  <div id="text">X</div>
</template>

<style scoped>
  #text{
    border-radius: 0.5rem;
    color: white;
    width: 2rem; height: 2rem;
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>