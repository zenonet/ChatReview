
<script lang="ts" setup>

import { defineComponent } from 'vue';
import ChatView from './components/ChatView.vue';
import { Chat, Message } from './model/chat';


let messages: Message[] = [
  new Message("Hello, how are you?"),
  new Message("No")
];
messages[1].isOwn = true;

let chat = new Chat();
chat.messages = messages;

</script>

<template>
  <div class="content">
    <h1>Rsbuild with Vue</h1>
    <p>Start building amazing things with Rsbuild.</p>
    <ChatView
      :chat="chat">
  </ChatView>
  </div>
</template>





<style scoped>
.content {
  display: flex;
  min-height: 100vh;
  line-height: 1.1;
  text-align: center;
  flex-direction: column;
  justify-content: center;
}

.content h1 {
  font-size: 3.6rem;
  font-weight: 700;
}

.content p {
  font-size: 1.2rem;
  font-weight: 400;
  opacity: 0.5;
}
</style>
