<script setup>
import Form from './components/Form.vue'
</script>

<template>
  <header></header>

  <main>
    <div class="main-container">
      <Form />
    </div>
  </main>
</template>

<style>
body {
  font-family: 'Arial', sans-serif;
  background-color: #2E3440; /* Nord Polar Night */
  margin: 0;
  padding: 0;
  color: #D8DEE9; /* Nord Snow Storm */
}
header {
  height: 20px;
}
main {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
}
.main-container {
  max-width: 600px;
  width: 90%;
}
</style>
