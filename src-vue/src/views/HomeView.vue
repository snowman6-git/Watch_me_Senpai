<template>
  <Notice text="후배쿤, 벌써 포기하려는거야?" @click="notice_is_open = !notice_is_open" v-show="notice_is_open"/>
  <div id="panel">
    <div id="title">
      <h2>xx님,</h2>
      <h1>날 지켜봐줘!</h1>
    </div>

    <div id="btn_case">
      <Btn @click="list_save" text="이어하기"/>
      <Btn @click="$router.push('/newgame')" text="새로하기"/>
      <Btn text="갤러리"/>
      <Btn text="크레딧"/>
      <Btn @click="delete_save" text="모든 세이브 삭제"/>
      <Btn @click="notice_is_open = !notice_is_open" text="나가기"/>
    </div>

  </div>
</template>

<script lang="ts">
const { invoke } = window.__TAURI__.core

import Btn from '../components/Btn.vue'
import ActionBtn from '../components/ActionBtn.vue'
import Notice from '../components/Notice.vue'



export default{
  components: {
    Btn,
    Notice,
    ActionBtn,
  },
  methods: {
    delete_save(){
      invoke("delete_save")
    },
    list_save(){
      invoke("list_save").then((info: any) => {
        console.log(info)
      })
    }

  },
  data()
  {
    return{
      notice_is_open: false,
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
