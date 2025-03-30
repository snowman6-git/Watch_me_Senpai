<template>
  <main>
    <h1>너의 이름은?</h1>
    <br>
    <input class="clear_input">
  </main>
</template>

<script>
import Btn from '../components/Btn.vue'
import ActionBtn from '../components/ActionBtn.vue'
import Notice from '../components/Notice.vue'

const { invoke } = window.__TAURI__.core;
invoke("text_input", { text : "스노우맨" }).then((info) => { //어떤 함수를 전달할지 json처럼 잡아줘야한다
    console.log(info)
})

export default{
  components: {
    Btn,
    Notice,
    ActionBtn,
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
