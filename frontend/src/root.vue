<script setup>
 import { RouterLink, RouterView } from 'vue-router';


// Using v-text is the only way it works without the whole app crashing.
//  I've got no idea why  and I just wasted an hour trying to find out
// Also, I would've put the comment in the template to the router link where it belongs but the app also crashes when
// I use comments in a vue file. I am starting to have javascript or vue js or whatever is doing all this

</script>

<template>
    <nav style="height: 40px;">
      <RouterLink to="/login" v-text="'Login'"></RouterLink>
</nav>
  <div id="app">
    <RouterView></RouterView>
  </div>
</template>