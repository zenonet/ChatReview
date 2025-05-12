<script>
import { Message } from '../model/chat';

export default{
    props: {
        message: { required: true, type: Message}
    }
}

</script>



<template>
<div class="message" :class="{ 'own-message': message.isOwn }">
    <p>
        {{ message.content }}
    </p>
</div>
</template>




<style scoped>
.message{
    background: lime;
    color: black;
    padding: 2pt;
    margin: 2pt;
    border-radius: 5px;
    font-size: 1.5em;
    width: fit-content;
}

.own-message{
    align-self: flex-end;
}
</style>