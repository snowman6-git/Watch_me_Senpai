<template>
  <main>
    <h1>너의 이름은?</h1>
    <br>

    <input ref="player_name" class="clear_input">
    <br>
    <Btn @click="save_name(this.$refs.player_name.value)" text="시작하기!"/>
     <TextBox :text="message" @click="tbox_is_open = !tbox_is_open" v-show="tbox_is_open"/>

</main>
</template>

<script lang="ts">
import Btn from '../components/Btn.vue'
import ActionBtn from '../components/ActionBtn.vue'
import Notice from '../components/Notice.vue'
import TextBox from '../components/TextBox.vue'

const { invoke } = window.__TAURI__.core;


export default{
  components: {
    Btn,
    Notice,
    ActionBtn,
    TextBox
  },

  methods: {
    save_name(name: String){
      invoke("new_save", { name : name }).then((info: any) => {
        if (info == 406) {
          this.message = "세상에 벌써 이스터에그를 찾았다니 축하합니다, 보상으로 선물을 드릴테니 집주소를 주세요, 농담아닙니다, 집주소 주세요, 달라고 야 어디가."
        } if(info == 403) {
          this.message = "너 누구야"
        } if (info == 200){
          this.message = `"${name}"을(를) 사용합니다, 괜찮은가요?`
        }
          this.tbox_is_open = true
          console.log(info)
      })
    }

  },

  data()
  {
    return{
      tbox_is_open: false,
      message: ""
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
