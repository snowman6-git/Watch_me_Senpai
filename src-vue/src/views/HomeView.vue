<template>
  <Notice v-show="isVisible"/>
  <div id="panel">
  

    <div id="title">
      <h2>xx님,</h2>
      <h1>날 지켜봐줘!</h1>
    </div>

    <div id="btn_case">
      <Btn text="이어하기"/>
      <Btn text="새로하기"/>
      <Btn text="갤러리"/>
      <Btn text="크레딧"/>
      <Btn text="나가기"/>
    </div>

  </div>
</template>

<script>
import Btn from '../components/Btn.vue'
import Notice from '../components/Notice.vue'
import { ref } from 'vue';

export default{
  components: {
    Btn,
    Notice
  },
  data(){
    return{
      notice_is_open: true,
      btns: ["이어하기", "새로하기", "갤러리", "크레딧", "나가기"],
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
