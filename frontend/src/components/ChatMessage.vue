<script>
import { Message } from '../model/chat';

export default{
    props: {
        message: { required: true, type: Message}
    }
}

</script>



<template>
<div class="message" :class="{ 'own-message': message.isOwn, 'others-message': !message.isOwn }">
    <p style="margin: 0;">
        {{ message.content }}
    </p>
</div>
</template>


<style scoped>
.message{
    background: lime;
    color: black;
    padding: 15pt;
    margin: 2pt;
    border-radius: 20px;
    font-size: 1.5em;
    width: fit-content;
    user-select: none; /* TOOD: Think about this, this might be a stupid idea */
}

.others-message{
    align-self: flex-start;
    background: cyan;
    border-bottom-left-radius: 0;
}

.own-message{
    align-self: flex-end;
    border-bottom-right-radius: 0;
}
</style>