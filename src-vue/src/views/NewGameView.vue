<template>
  <main>
    <h1>너의 이름은?</h1>
    <br>

    <input ref="player_name" class="clear_input">
    <br>
    <Btn @click="handleClick" text="시작하기!"/>

</main>
</template>

<script lang="ts">
import Btn from '../components/Btn.vue'
import ActionBtn from '../components/ActionBtn.vue'
import Notice from '../components/Notice.vue'

const { invoke } = window.__TAURI__.core;

function save_name(name: String){
  invoke("new_save", { name : name }).then((info: any) => {

    console.log(info)
  })
}


export default{
  components: {
    Btn,
    Notice,
    ActionBtn,
  },

  methods: {
    handleClick() {
      let a = save_name(this.$refs.player_name.value);
    },
  },

  data()
  {
    return{
    }
  }
}
</script>

<style>
main{
  width: 100%; height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
}
.clear_input{
  
  /* text-underline-position: below;
  text-underline-offset: 0.1rem;
  text-decoration : underline; */
  
  width: 7rem;
  text-align: center;

  font-size: 1.25rem;
  padding: 0.5rem;
  border-bottom: 0.1rem solid white;
}

</style>
